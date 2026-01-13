#!/usr/bin/env -S uv run -q
# Written by Luis Felipe Montemayor, sometime around January of 2026
# https://open.spotify.com/track/2hkJ7fYPM5V27pjxiPG2gg?si=f7a42e6525d44707
import re
import sys
from pathlib import Path
from sys import stderr

VERSION_FILE: Path = Path(".python-version")


def get_python_version(version_file: Path = VERSION_FILE) -> str:
    version: str = version_file.read_text().strip()
    return version


def get_version_keys(version: str) -> list[tuple[Path, str, str]]:
    return [
        (
            Path("pyproject.toml"),
            r'(requires-python\s*=\s*)"[^"]*"',
            rf'\1">={version}"',
        ),
        (
            Path("glia_python/pyproject.toml"),
            r'(requires-python\s*=\s*)"[^"]*"',
            rf'\1">={version}"',
        ),
        (
            Path("libs/python/glia_common/pyproject.toml"),
            r'(requires-python\s*=\s*)"[^"]*"',
            rf'\1">={version}"',
        ),
        (
            Path("pyproject.toml"),
            r'(?s)(\[tool\.ty\.environment\][^\[]*?python-version\s*=\s*)"[^"]*"',
            rf'\1"{version.rstrip(".0")}"',
        ),
        (
            Path("mise.toml"),
            r'(python\s*=\s*)"[^"]*"',
            rf'\1"{version}"',
        ),
    ]


def check_untracked_toml_files(tracked_paths: set[Path]) -> None:
    ignored_dirs: set[str] = {".venv", "node_modules", ".git", "dist", "build"}
    ignored_files: set[Path] = {
        Path("ruff.toml"),
    }

    all_toml_files: set[Path] = set()
    for path in Path(".").rglob("*.toml"):
        if any(ignored in path.parts for ignored in ignored_dirs):
            continue
        all_toml_files.add(path)

    untracked: set[Path] = all_toml_files - ignored_files - tracked_paths

    if untracked:
        print(
            "Error: Untracked .toml files found. Add to get_version_keys() or blacklist:",
            file=stderr,
        )
        for file in sorted(untracked):
            print(f"  - {file}", file=stderr)
        sys.exit(1)


def sync_python_versions(version_file: Path = VERSION_FILE) -> None:
    if not version_file.exists():
        print(f"Error: {version_file} not found.", file=stderr)
        sys.exit(1)

    version: str = get_python_version(version_file)
    targets: list[tuple[Path, str, str]] = get_version_keys(version)

    path_to_transforms: dict[Path, list[tuple[str, str]]] = {}
    for path, pattern, replacement in targets:
        path_to_transforms.setdefault(path, []).append((pattern, replacement))

    tracked_paths: set[Path] = set(path_to_transforms.keys())

    for file_path, transforms in path_to_transforms.items():
        if not file_path.exists():
            print(f"Skipping: {file_path} (not found)", file=stderr)
            continue

        original_content: str = file_path.read_text()
        current_content: str = original_content

        for pattern, replacement in transforms:
            current_content = re.sub(pattern, replacement, current_content)

        if original_content != current_content:
            file_path.write_text(current_content)
            print(f"Updated: {file_path}", file=stderr)
        else:
            print(f"File already updated: {file_path}", file=stderr)

    check_untracked_toml_files(tracked_paths)


if __name__ == "__main__":
    sync_python_versions()
