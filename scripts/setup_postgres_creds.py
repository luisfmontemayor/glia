#!/usr/bin/env -S uv run
# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://open.spotify.com/track/1OW3pXLhVdMm7giLW6siEm?si=84f15023bf694f0f
import signal
import sys
from pathlib import Path

from glia_common import cli, system

ENV_FILE = Path(".env")
PG_KEYS = ["POSTGRES_USER", "POSTGRES_PASSWORD", "POSTGRES_DB"]


def signal_handler(sig, frame):
    sys.exit(1)


signal.signal(signal.SIGINT, signal_handler)


def get_verified_password() -> str:
    password = cli.gum_input("Enter Postgres Password:", password=True)

    if not password:
        print("Password cannot be empty.", file=sys.stderr)
        return get_verified_password()

    password_confirm = cli.gum_input("Confirm Password:", password=True)

    if password != password_confirm:
        print("Passwords do not match. Try again.", file=sys.stderr)
        return get_verified_password()

    return password


def get_postgres_config() -> dict[str, str]:
    print("Configuring Postgres credentials...")

    user = cli.gum_input("Enter Postgres User:", value="glia_user")
    if not user or user == "":
        print("Error: no user returned. Exiting.", file=sys.stderr)
        sys.exit(1)

    db_name = cli.gum_input("Enter Postgres DB Name:", value="glia_db")
    if not db_name or db_name == "":
        print("Error: no database name returned. Exiting.", file=sys.stderr)
        sys.exit(1)

    return {
        "POSTGRES_USER": user,
        "POSTGRES_PASSWORD": get_verified_password(),
        "POSTGRES_DB": db_name,
    }


def write_env(config: dict[str, str], overwrite: bool = False):
    original_lines = []
    if ENV_FILE.exists():
        with open(ENV_FILE) as f:
            original_lines = f.readlines()

    if overwrite:
        original_lines = [
            line
            for line in original_lines
            if not any(line.strip().startswith(k + "=") for k in PG_KEYS)
        ]

    if original_lines and not original_lines[-1].endswith("\n"):
        original_lines[-1] += "\n"

    prefix = "\n" if original_lines else ""

    new_block = [f"{prefix}# --- Postgres Configuration (Managed by setupEnv.py) ---\n"]
    for key in PG_KEYS:
        new_block.append(f"{key}={config[key]}\n")

    with open(ENV_FILE, "w") as f:
        f.writelines(original_lines + new_block)

    print(f"Successfully wrote configuration to {ENV_FILE}", file=sys.stderr)


def main():
    # We check existence first because parse_env_file exits on failure,
    # but for setup, a missing file is a valid starting state.
    if ENV_FILE.exists():
        existing_vars = system.parse_env_file(ENV_FILE)
    else:
        existing_vars = {}

    keys_exist = any(key in existing_vars for key in PG_KEYS)

    if keys_exist:
        should_overwrite = cli.gum_confirm(
            "Postgres configuration found in .env. Overwrite postgres variables?"
        )
        if not should_overwrite:
            print("Skipping Postgres configuration.")
            sys.exit(0)

    config = get_postgres_config()
    write_env(config, overwrite=keys_exist)


if __name__ == "__main__":
    main()
