#!/usr/bin/env python3
# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://youtu.be/lBueLHd2Ojw?t=1083

import shutil
import signal
import subprocess
import sys

from glia_common import cli

from .constants import COMMIT_TYPES
from .scopes import get_staged_scopes

SYSTEM_DEPENDENCIES = ["gum"]

git_commit_cmd = ["git", "commit", "-m"]


def signal_handler(sig, frame):
    sys.exit(0)


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

    preselected_commit_type = (
        sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] in COMMIT_TYPES else None
    )
    if preselected_commit_type:
        commit_type = preselected_commit_type
    else:
        commit_type = cli.gum_choose("Choose a commit type", COMMIT_TYPES)
        if not commit_type:
            sys.exit(0)

    possible_scope_choices = get_staged_scopes()
    scope = cli.gum_choose("Choose a commit scope", list(possible_scope_choices))
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
