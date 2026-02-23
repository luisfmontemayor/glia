# Written by Luis Felipe Montemayor, sometime around December of 2025

COMMIT_TYPES: list[str] = ["feat", "fix", "refactor", "test", "chore"]
SCOPE_CATEGORIES: set[str] = {
    "backend",
    "glia_python",
    "glia_core",
    "gliar",
    "infrastructural",
    "scripts",
    "libs",
}
INFRA_FILES: set[str] = {
    "mise.toml",
    "mise.lock",
    "pyproject.toml",
    "uv.lock",
    "ruff.toml",
    ".gitattributes",
    ".gitignore",
    "lazygit.yml",
}
INFRA_DIRS: set[str] = {".git", ".vscode"}
NO_SCOPE_STR = "None"
