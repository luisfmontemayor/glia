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


def gum_input(
    header: str, placeholder: str = "", value: str = "", password: bool = False
) -> str | None:
    cmd = ["gum", "input", "--header", header]
    if placeholder:
        cmd.extend(["--placeholder", placeholder])
    if value:
        cmd.extend(["--value", value])
    if password:
        cmd.append("--password")

    result = subprocess.run(
        cmd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=None,
    )

    if result.returncode == 0:
        return result.stdout.strip()
    return None


def gum_choose(header: str, options: list[str]) -> str | None:
    if not options:
        return None

    cmd: list[str] = ["gum", "choose", "--header", header]
    if len(options) == 1:
        cmd.append("--select-if-one")
    cmd.extend(options)

    result = subprocess.run(
        cmd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=None,  # The interactive output
    )

    if result.returncode == 0:
        return result.stdout.strip()
    return None


def gum_confirm(prompt: str) -> bool:
    return subprocess.run(["gum", "confirm", prompt]).returncode == 0
