#!/usr/bin/env python3
# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://open.spotify.com/track/4ewSenduOmU9Vo40jPnK4T?si=ece2bb53df9f4aa6

import sys

from . import utils


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


def list_stage_files() -> list[str]:
    output = utils.run_command(["git", "diff", "--cached", "--name-only"])
    if not output:
        return []
    return output.split("\n")


def stage_all_files():
    utils.run_command(["git", "add", "--all"], capture=False)


def get_staged_files():
    staged_files = list_stage_files()
    if not staged_files:
        if confirm_stage_all():
            stage_all_files()
        else:
            sys.exit(0)
    return list_stage_files()
