"""The repository's tasks: install, format, lint, build, test and release.

The Makefile and the workflows call these by name and hold no logic of their own.

Usage: python -m tools.tasks <install|format|lint|build|test|release> [version]
"""

import os
import re
import shutil
import subprocess
import sys
from collections.abc import Callable
from enum import StrEnum

from github import Auth, Github, UnknownObjectException

from tools.build_skill import ROOT, BuildError, build

PLUGIN = "erp"
CLAUDE = ROOT / "node_modules" / ".bin" / "claude"
VERSION = re.compile(r"^v\d+\.\d+\.\d+$")


class Task(StrEnum):
    INSTALL = "install"
    FORMAT = "format"
    LINT = "lint"
    BUILD = "build"
    TEST = "test"
    RELEASE = "release"


def run(*command: str) -> None:
    # npm, uv's tool runner, git and Claude Code's plugin validator are command-line
    # programs with no Python API, so they run as subprocesses.
    subprocess.run(command, cwd=ROOT, check=True)


def nothing_staged() -> bool:
    # git has no Python API in this project; its exit status is the answer.
    return subprocess.run(("git", "diff", "--cached", "--quiet"), cwd=ROOT, check=False).returncode == 0


def install() -> None:
    """Install the pinned Claude Code, whose plugin validator the build runs."""
    run("npm", "ci")


def format_code() -> None:
    run("uv", "run", "--frozen", "ruff", "format", "tools")


def lint() -> None:
    run("uv", "run", "--frozen", "ruff", "check", "tools")
    run("uv", "run", "--frozen", "ruff", "format", "--check", "tools")
    run("uv", "run", "--frozen", "mypy", "tools")


def validate(*paths: str) -> None:
    for path in paths:
        run(str(CLAUDE), "plugin", "validate", path)


def build_plugin() -> None:
    build("0.0.0", ROOT / "dist" / PLUGIN)


def test() -> None:
    """Build the plugin from the documents and validate the plugin and the marketplace file."""
    build_plugin()
    validate(f"dist/{PLUGIN}", ".")


def release(version: str) -> None:
    """Build at `version`, commit the built plugin on main, tag it and publish the release."""
    if not VERSION.match(version):
        raise BuildError(f"version must look like v1.2.3, got '{version}'")
    repository = Github(auth=Auth.Token(os.environ["GH_TOKEN"])).get_repo(os.environ["GITHUB_REPOSITORY"])
    try:
        repository.get_git_ref(f"tags/{version}")
    except UnknownObjectException:
        pass
    else:
        raise BuildError(f"{version} is already released")
    lint()
    test()
    build(version.removeprefix("v"), ROOT / "plugins" / PLUGIN)
    validate(f"plugins/{PLUGIN}", ".")
    run("git", "add", f"plugins/{PLUGIN}")
    if nothing_staged():
        raise BuildError("the built plugin is identical to the released one; nothing to release")
    run("git", "config", "user.name", "github-actions[bot]")
    run("git", "config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com")
    run("git", "commit", "--message", f"release({PLUGIN}): {version}")
    run("git", "push", "origin", "HEAD:main")
    archive = shutil.make_archive(str(ROOT / f"{PLUGIN}-plugin-{version}"), "zip", ROOT / "plugins", PLUGIN)
    published = repository.create_git_release(
        tag=version,
        name=f"{PLUGIN} plugin {version}",
        target_commitish="main",
        generate_release_notes=True,
    )
    published.upload_asset(archive)


TASKS: dict[Task, Callable[[], None]] = {
    Task.INSTALL: install,
    Task.FORMAT: format_code,
    Task.LINT: lint,
    Task.BUILD: build_plugin,
    Task.TEST: test,
}


def main(arguments: list[str]) -> int:
    try:
        task = Task(arguments[0]) if arguments else None
    except ValueError:
        task = None
    try:
        if task is Task.RELEASE and len(arguments) == 2:
            release(arguments[1])
        elif task in TASKS and len(arguments) == 1:
            TASKS[task]()
        else:
            print((__doc__ or "").strip().splitlines()[-1], file=sys.stderr)
            return 2
    except BuildError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    except subprocess.CalledProcessError as error:
        return error.returncode or 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
