# Written by Luis Felipe Montemayor, sometime around December of 2025

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
