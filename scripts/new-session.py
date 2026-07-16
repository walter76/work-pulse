"""Initialize a new session log from template.md."""

import argparse
import re
import sys
from datetime import date
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SESSIONS = ROOT / "sessions"
TEMPLATE = SESSIONS / "template.md"
INDEX = SESSIONS / "INDEX.md"


def make_slug(task: str) -> str:
    slug = task.lower().strip()
    slug = re.sub(r"[^a-z0-9]+", "-", slug)
    slug = slug.strip("-")
    return slug


def unique_path(date_str: str, slug: str) -> Path:
    base = SESSIONS / f"{date_str}-{slug}.md"
    if not base.exists():
        return base
    n = 2
    while True:
        candidate = SESSIONS / f"{date_str}-{slug}-{n}.md"
        if not candidate.exists():
            return candidate
        n += 1


def update_index(date_str: str, filename: str, status: str, task: str) -> None:
    row = f"| {date_str} | [{task}]({filename}) | {status} |  |\n"
    if not INDEX.exists():
        INDEX.write_text(
            "| Date | Session | Status | Summary |\n"
            "|------|---------|--------|---------|\n"
            + row
        )
        return

    lines = INDEX.read_text().splitlines(keepends=True)
    # Find the header separator line (|------|...)
    sep_idx = None
    for i, line in enumerate(lines):
        if line.startswith("|------"):
            sep_idx = i
            break

    if sep_idx is not None:
        lines.insert(sep_idx + 1, row)
    else:
        lines.append(row)

    INDEX.write_text("".join(lines))


def main() -> None:
    parser = argparse.ArgumentParser(description="Create a new session log")
    parser.add_argument("-t", "--task", required=True, help="Short task description")
    parser.add_argument("-a", "--agent", default="", help="Agent name (default: empty)")
    parser.add_argument("--date", default=None, help="Date override YYYY-MM-DD (default: today)")
    args = parser.parse_args()

    if not TEMPLATE.exists():
        print(f"Error: template not found: {TEMPLATE}", file=sys.stderr)
        sys.exit(1)

    today = args.date or date.today().isoformat()
    slug = make_slug(args.task)
    dest = unique_path(today, slug)

    content = TEMPLATE.read_text()
    content = content.replace("{{DATE}}", today)
    content = content.replace("{{TASK}}", args.task)
    content = content.replace("{{AGENT}}", args.agent)

    dest.write_text(content)
    print(f"Created: {dest.relative_to(ROOT)}")

    update_index(today, dest.name, "in-progress", args.task)
    print(f"Updated: {INDEX.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
