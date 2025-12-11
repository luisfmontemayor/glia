#!/usr/bin/env python3
# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://open.spotify.com/track/4ewSenduOmU9Vo40jPnK4T?si=ece2bb53df9f4aa6

import sys
from pathlib import Path

from . import utils
from .constants import INFRA_DIRS, INFRA_FILES, NO_SCOPE_STR


def changed_files_exist():
    unstaged = utils.run_command(
        ["git", "ls-files", "--others", "--exclude-standard", "--modified"]
    )
    return True if unstaged else False


def confirm_stage_all():
    if not changed_files_exist():
        print("No changed files exist.")
        return False
    else:
        return utils.gum_confirm("Stage all files?")


def list_staged_files() -> list[str]:
    output = utils.run_command(["git", "diff", "--cached", "--name-only"])
    if not output:
        return []
    return output.split("\n")


def stage_all_files():
    utils.run_command(["git", "add", "--all"], capture=False)


def get_staged_files():
    staged_files = list_staged_files()
    if not staged_files:
        if confirm_stage_all():
            stage_all_files()
        else:
            sys.exit(0)
    return list_staged_files()


def is_infra_file(
    filename: str, filepath: Path, INFRA_FILES: set[str] = INFRA_FILES
) -> bool:
    return filename in INFRA_FILES or any(
        dir for dir in INFRA_DIRS if dir in filepath.parts
    )


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


def add_scope_category(filepath: str) -> str:
    path = Path(filepath)
    parts = path.parts
    root = parts[0] if parts else ""
    filename = path.name

    if is_infra_file(filename, filepath=path):
        return "infrastructural"

    if root == "scripts":
        if len(parts) > 2:
            return f"helpers/{parts[1]}"
        if len(parts) > 1:
            return f"helpers/{path.stem}"
        else:
            return "helpers"

    if root == "backend":
        relative = Path(*parts[1:])
        return f"backend/{relative.with_suffix('')}"

    if root == "clients":
        relative = Path(*parts[1:])
        return f"clients/{relative.with_suffix('')}"

    return NO_SCOPE_STR


def get_staged_scopes():
    unique_staged_scopes = [add_scope_category(f) for f in get_staged_files()]
    possible_scope_choices = []
    possible_scope_choices.extend(list(set(unique_staged_scopes)))

    common_scope = get_common_path(unique_staged_scopes)
    if common_scope and common_scope not in possible_scope_choices:
        possible_scope_choices.insert(0, common_scope)

    if NO_SCOPE_STR not in possible_scope_choices:
        possible_scope_choices.append(NO_SCOPE_STR)

    return sorted(possible_scope_choices)
