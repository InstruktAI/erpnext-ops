"""The repository's tasks: install, format, lint, build, test and release.

The Makefile and the workflows call these by name and hold no logic of their own.

Usage: python -m tools.tasks <install|format|lint|build|test|release> [version]
"""

import re
import shutil
import subprocess
import sys

from tools.build_skill import ROOT, BuildError, build

PLUGIN = "erp"
CLAUDE = ROOT / "node_modules" / ".bin" / "claude"
VERSION = re.compile(r"^v\d+\.\d+\.\d+$")


def run(*command: str) -> None:
    subprocess.run(command, cwd=ROOT, check=True)


def succeeds(*command: str) -> bool:
    return subprocess.run(command, cwd=ROOT, capture_output=True, check=False).returncode == 0


def install() -> None:
    """Install the pinned Claude Code, whose plugin validator the build runs."""
    run("npm", "ci")


def format_code() -> None:
    run(sys.executable, "-m", "ruff", "format", "tools")


def lint() -> None:
    run(sys.executable, "-m", "ruff", "check", "tools")
    run(sys.executable, "-m", "ruff", "format", "--check", "tools")
    run(sys.executable, "-m", "mypy", "tools")


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
    if succeeds("git", "ls-remote", "--exit-code", "--tags", "origin", f"refs/tags/{version}"):
        raise BuildError(f"{version} is already released")
    lint()
    test()
    build(version.removeprefix("v"), ROOT / "plugins" / PLUGIN)
    validate(f"plugins/{PLUGIN}", ".")
    run("git", "add", f"plugins/{PLUGIN}")
    if succeeds("git", "diff", "--cached", "--quiet"):
        raise BuildError("the built plugin is identical to the released one; nothing to release")
    run("git", "config", "user.name", "github-actions[bot]")
    run("git", "config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com")
    run("git", "commit", "--message", f"release({PLUGIN}): {version}")
    run("git", "push", "origin", "HEAD:main")
    archive = shutil.make_archive(str(ROOT / f"{PLUGIN}-plugin-{version}"), "zip", ROOT / "plugins", PLUGIN)
    run(
        "gh",
        "release",
        "create",
        version,
        archive,
        "--target",
        "main",
        "--title",
        f"{PLUGIN} plugin {version}",
        "--generate-notes",
    )


TASKS = {"install": install, "format": format_code, "lint": lint, "build": build_plugin, "test": test}


def main(arguments: list[str]) -> int:
    name = arguments[0] if arguments else ""
    try:
        if name == "release" and len(arguments) == 2:
            release(arguments[1])
        elif name in TASKS and len(arguments) == 1:
            TASKS[name]()
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
