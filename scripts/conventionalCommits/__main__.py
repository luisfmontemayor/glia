#!/usr/bin/env python3
# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://youtu.be/lBueLHd2Ojw?t=1083

import shutil
import signal
import subprocess
import sys
from pathlib import Path

from . import stageFiles, utils

SYSTEM_DEPENDENCIES = ["gum"]
COMMIT_TYPES = ["feat", "fix", "refactor", "test", "chore"]
SCOPE_CATEGORIES = {"backend", "clients", "infrastructural", "helpers"}
INFRA_FILES = {
    "mise.toml",
    "mise.lock",
    "pyproject.toml",
    "uv.lock",
    "ruff.toml",
    ".gitattributes",
    ".gitignore",
}
INFRA_DIRS = {".git", ".vscode"}
NO_SCOPE_STR = "(None)"

git_commit_cmd = ["git", "commit", "-m"]


def signal_handler(sig, frame):
    sys.exit(0)


def is_infra_file(
    filename: str, filepath: Path, INFRA_FILES: set[str] = INFRA_FILES
) -> bool:
    return filename in INFRA_FILES or any(
        dir for dir in INFRA_DIRS if dir in filepath.parts
    )


def prefix_scope_category(filepath: str) -> str:
    """
    Prefixes a filepath with a scope category
    """
    path = Path(filepath)
    parts = path.parts
    root = parts[0] if parts else ""
    filename = path.name

    if is_infra_file(filename, filepath=path):
        return "infrastructural"

    if root == "scripts":
        if len(parts) > 1:
            return f"helpers/{path.stem}"
        return "helpers"

    if root == "backend":
        relative = Path(*parts[1:])
        return f"backend/{relative.with_suffix('')}"

    if root == "clients":
        relative = Path(*parts[1:])
        return f"clients/{relative.with_suffix('')}"

    return f"{path.parent}/{path.stem}".strip("./")


def get_common_path(scopes: list[str]) -> str | None:
    if not scopes:
        return None

    split_scopes = [s.split("/") for s in scopes]
    common_parts = []

    for parts in zip(*split_scopes, strict=False):
        if all(p == parts[0] for p in parts):
            common_parts.append(parts[0])
        else:
            break

    return "/".join(common_parts) if common_parts else None


def main():
    missing_deps = [dep for dep in SYSTEM_DEPENDENCIES if not shutil.which(dep)]
    if missing_deps:
        print(
            f"Error: The following dependencies are missing: {', '.join(missing_deps)}."
        )
        print("Run `mise run setup` to install them.")
        sys.exit(1)
    if len(sys.argv) > 1 and any(
        help_arg in sys.argv[1:2] for help_arg in ["-h", "--help"]
    ):
        print("Usage: ./commit.py [type] [message]")
        sys.exit(0)
    signal.signal(signal.SIGINT, signal_handler)

    staged_files = stageFiles.get_staged_files()

    preselected_commit_type = (
        sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] in COMMIT_TYPES else None
    )
    if preselected_commit_type:
        commit_type = preselected_commit_type
    else:
        commit_type = utils.gum_choose("Choose a commit type", COMMIT_TYPES)
        if not commit_type:
            sys.exit(0)

    unique_staged_scopes = [prefix_scope_category(f) for f in staged_files]
    possible_scope_choices = []
    possible_scope_choices.extend(list(set(unique_staged_scopes)))
    active_categories = {
        scope.split("/")[0]
        for scope in unique_staged_scopes
        if scope.split("/")[0] in SCOPE_CATEGORIES
    }
    common_scope = get_common_path(unique_staged_scopes)
    if common_scope and common_scope not in possible_scope_choices:
        possible_scope_choices.insert(0, common_scope)
    for cat in active_categories:
        if cat not in possible_scope_choices:
            possible_scope_choices.append(cat)
    possible_scope_choices.append(NO_SCOPE_STR)
    scope = utils.gum_choose("Choose a commit scope", list(possible_scope_choices))
    if not scope:
        sys.exit(0)

    preselected_message = " ".join(sys.argv[2:]) if len(sys.argv) > 2 else ""
    # If type was picked interactively, but message was passed as arg 2?
    # Logic: if argv[1] was type, message is argv[2+]. If argv[1] wasn't type, check if argv[1] is message
    if (
        not preselected_message
        and len(sys.argv) > 1
        and sys.argv[1] not in COMMIT_TYPES
    ):
        preselected_message = " ".join(sys.argv[1:])
    commit_prefix = f"{commit_type}({scope}):"
    if preselected_message:
        conventional_commit_message = f"{commit_prefix} {preselected_message}"
    else:
        gum_result = subprocess.run(
            [
                "gum",
                "input",
                "--placeholder",
                "Summary of changes",
                "--prompt",
                f"> {commit_prefix} ",
            ],
            text=True,
            stdout=subprocess.PIPE,  # Capture the result
            stderr=None,
        )

        if not gum_result or gum_result.returncode != 0:
            print("Aborted.")
            sys.exit(1)

        conventional_commit_message = f"{commit_prefix} {gum_result.stdout.strip()}"

    _ = subprocess.run(git_commit_cmd + [conventional_commit_message])


if __name__ == "__main__":
    main()
