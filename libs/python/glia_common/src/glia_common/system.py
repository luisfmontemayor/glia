# Written by Luis Felipe Montemayor, sometime around December of 2025
# https://open.spotify.com/track/5rKofmG2wSD3pdZcNpkz6T?si=2ba4f58be4b54277
import subprocess


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
