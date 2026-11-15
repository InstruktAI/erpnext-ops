"""Build the `erpnext` Claude plugin from the published ERPNext documents.

The documents under docs/global/erpnext are the only source of doctrine. This script renders
them into one skill: the surface spec, principles and policy inline, procedures and
concepts as reference files that load on demand. skill/manifest.json says which documents
go where; skill/preamble.md is the skill's own packaging text.

Usage: build_skill.py [--version X.Y.Z] [--out DIR]
"""

import argparse
import json
import re
import shutil
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs" / "global" / "erpnext"
SKILL = ROOT / "skill"

MAX_SKILL_LINES = 500
MAX_DESCRIPTION_CHARS = 1024

DOC_REF = re.compile(r"@?(?:~|/[^\s`]*?)/\.teleclaude/docs/([a-z-]+)/([a-z0-9/_-]+)\.md")
PLANNED = re.compile(
    r"^<!-- planned:[a-z0-9-]+ -->\n.*?^<!-- /planned:[a-z0-9-]+ -->\n",
    re.MULTILINE | re.DOTALL,
)
PLANNED_CHANGE = re.compile(
    r"^<!-- planned-change:([a-z0-9-]+) -->\n(.*?)^<!-- change:\1 -->\n.*?^<!-- /planned-change:\1 -->\n",
    re.MULTILINE | re.DOTALL,
)


class BuildError(Exception):
    pass


@dataclass(frozen=True)
class Manifest:
    """What skill/manifest.json declares: the skill's identity and where each document goes."""

    name: str
    title: str
    plugin_description: str
    description: str
    inline: tuple[str, ...]
    exclude: tuple[str, ...]

    @classmethod
    def load(cls, path: Path) -> "Manifest":
        raw = json.loads(path.read_text())
        if not isinstance(raw, dict):
            raise BuildError(f"{path.name}: expected an object")

        def text(key: str) -> str:
            value = raw.get(key)
            if not isinstance(value, str) or not value:
                raise BuildError(f"{path.name}: '{key}' must be a non-empty string")
            return value

        def ids(key: str) -> tuple[str, ...]:
            value = raw.get(key)
            if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
                raise BuildError(f"{path.name}: '{key}' must be a list of document ids")
            return tuple(value)

        return cls(
            text("name"), text("title"), text("plugin_description"), text("description"), ids("inline"), ids("exclude")
        )


def read_snippet(path: Path) -> tuple[dict[str, str], str]:
    """Return a snippet's frontmatter and its body as it reads today."""
    text = path.read_text()
    match = re.match(r"---\n(.*?)\n---\n", text, re.DOTALL)
    if not match:
        raise BuildError(f"{path.relative_to(ROOT)}: no frontmatter")
    meta: dict[str, str] = {}
    for line in match.group(1).splitlines():
        key, sep, value = line.partition(":")
        if sep and not line.startswith(" "):
            meta[key.strip()] = value.strip()
    body = text[match.end() :]
    # Pending material belongs to work that has not landed: additions are left out and a
    # pending replacement keeps the text it would replace.
    body = PLANNED.sub("", body)
    body = PLANNED_CHANGE.sub(lambda m: m.group(2), body)
    return meta, body.strip() + "\n"


def snippet_id(path: Path) -> str:
    return "erpnext/" + path.relative_to(DOCS).with_suffix("").as_posix()


def reference_name(sid: str) -> str:
    _, kind, *rest = sid.split("/")
    return f"{kind}-{'-'.join(rest)}.md"


def split_required_reads(body: str) -> tuple[list[str], str]:
    match = re.search(r"^## Required reads\n(.*?)(?=^## )", body, re.MULTILINE | re.DOTALL)
    if not match:
        return [], body
    required = [f"{domain}/{rest}" for domain, rest in DOC_REF.findall(match.group(1))]
    return required, body[: match.start()] + body[match.end() :]


def rewrite_links(body: str, published: set[str], inline: set[str]) -> str:
    """Point document links at the skill's own files and drop links that leave it."""
    lines = []
    for line in body.splitlines():
        match = DOC_REF.search(line)
        if match:
            sid = f"{match.group(1)}/{match.group(2)}"
            if sid in inline:
                line = line.replace(match.group(0), "this skill's SKILL.md")
            elif sid in published:
                line = line.replace(match.group(0), f"references/{reference_name(sid)}")
            else:
                continue
        lines.append(line)
    text = "\n".join(lines) + "\n"
    return re.sub(r"`erpnext/procedure/([a-z0-9-]+)`", r"`\1`", text)


def build(version: str, out: Path) -> None:
    manifest = Manifest.load(SKILL / "manifest.json")
    inline = list(manifest.inline)
    exclude = set(manifest.exclude)
    description = manifest.description
    if len(description) > MAX_DESCRIPTION_CHARS:
        raise BuildError(f"skill description is {len(description)} characters; the limit is {MAX_DESCRIPTION_CHARS}")

    snippets: dict[str, tuple[dict[str, str], str]] = {}
    for path in sorted(DOCS.rglob("*.md")):
        if path.name.startswith("baseline") or path.stem.count("."):
            continue  # manifests, and companions that carry a snippet's pending rewrite
        meta, body = read_snippet(path)
        if "delivered_by" in meta:
            continue  # not live yet
        snippets[snippet_id(path)] = (meta, body)

    unknown = [sid for sid in [*inline, *exclude] if sid not in snippets]
    if unknown:
        raise BuildError(f"skill/manifest.json names documents that do not exist: {unknown}")
    published = {sid for sid in snippets if sid not in exclude}

    skill_dir = out / "skills" / manifest.name
    references = skill_dir / "references"
    shutil.rmtree(out, ignore_errors=True)
    references.mkdir(parents=True)
    (out / ".claude-plugin").mkdir()
    (out / ".claude-plugin" / "plugin.json").write_text(
        json.dumps(
            {
                "name": manifest.name,
                "version": version,
                "description": manifest.plugin_description,
            },
            indent=2,
        )
        + "\n"
    )

    parts = [
        f"---\nname: {manifest.name}\ndescription: {json.dumps(description)}\n---\n",
        f"# {manifest.title}\n",
    ]
    parts.append((SKILL / "preamble.md").read_text().strip() + "\n")
    for sid in sorted(s for s in published if "/procedure/" in s):
        meta, _ = snippets[sid]
        parts.append(f"- `references/{reference_name(sid)}`: {meta.get('activation_trigger') or meta['description']}")
    parts.append("\n## Background\n\nRead when the work touches it.\n")
    for sid in sorted(s for s in published if "/concept/" in s):
        parts.append(f"- `references/{reference_name(sid)}`: {snippets[sid][0]['description']}")
    parts.append(
        "\n## Surface, principles and rules\n\nThese govern all work and are already loaded with this skill.\n"
    )
    for sid in inline:
        _, body = split_required_reads(snippets[sid][1])
        body = rewrite_links(body, published, set(inline))
        parts.append(
            re.sub(
                r"^(#+) ",
                lambda m: "#" * (len(m.group(1)) + 2) + " ",
                body,
                flags=re.MULTILINE,
            )
        )
    skill_md = "\n".join(parts)
    if skill_md.count("\n") > MAX_SKILL_LINES:
        raise BuildError(f"SKILL.md is {skill_md.count(chr(10))} lines; the limit is {MAX_SKILL_LINES}")
    (skill_dir / "SKILL.md").write_text(skill_md)

    for sid in sorted(published - set(inline)):
        required, body = split_required_reads(snippets[sid][1])
        first = [f"`references/{reference_name(r)}`" for r in required if r in published and r not in inline]
        head = f"Read first, unless already read in this conversation: {', '.join(first)}.\n\n" if first else ""
        (references / reference_name(sid)).write_text(head + rewrite_links(body, published, set(inline)))

    missing = [ref for ref in re.findall(r"references/[a-z0-9-]+\.md", skill_md) if not (skill_dir / ref).exists()]
    if missing:
        raise BuildError(f"SKILL.md points at reference files that were not written: {sorted(set(missing))}")
    print(
        f"built {manifest.name} {version}: {len(published)} documents, SKILL.md {skill_md.count(chr(10))} lines, "
        f"{len(list(references.iterdir()))} reference files -> {out.relative_to(ROOT) if out.is_relative_to(ROOT) else out}"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=(__doc__ or "").splitlines()[0])
    parser.add_argument("--version", default="0.0.0")
    parser.add_argument("--out", type=Path, default=ROOT / "dist" / "erpnext")
    args = parser.parse_args()
    try:
        build(args.version, args.out.resolve())
    except BuildError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
