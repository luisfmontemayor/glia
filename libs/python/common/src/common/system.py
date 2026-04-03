# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://open.spotify.com/track/5rKofmG2wSD3pdZcNpkz6T?si=2ba4f58be4b54277
import subprocess
import sys
from pathlib import Path


def run_command(command: list[str], capture: bool = True) -> str | None:
    try:
        result = subprocess.run(
            command,
            text=True,
            capture_output=capture,
            check=False,  # handled manually
        )
        if result.returncode != 0 and capture:
            return None
        return result.stdout.strip() if capture else str(result.returncode)
    except FileNotFoundError:
        return None


def changed_files_exist():
    unstaged = run_command(
        ["git", "ls-files", "--others", "--exclude-standard", "--modified"]
    )
    return True if unstaged else False


def parse_env_file(env_path: str | Path = ".env") -> dict[str, str]:
    path = Path(env_path)
    if not path.exists():
        print(f"Error: {path} not found.")
        sys.exit(1)

    config = {}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "=" in line:
                key, value = line.split("=", 1)
                config[key.strip()] = value.strip()
    return config
