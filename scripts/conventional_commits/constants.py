# Written by Luis Felipe Montemayor, sometime around December of 2025

COMMIT_TYPES: list[str] = ["feat", "fix", "refactor", "test", "chore"]
SCOPE_CATEGORIES: set[str] = {
    "backend",
    "glia_python",
    "core",
    "gliar",
    "infrastructural",
    "scripts",
    "libs",
    "test",
}
INFRA_FILES: set[str] = {
    "pyproject.toml",
    "uv.lock",
    "ruff.toml",
    ".gitattributes",
    ".gitignore",
    "lazygit.yml",
}
INFRA_DIRS: set[str] = {".git", ".vscode", ".config/mise/conf.d/"}

EXACT_FILE_MATCHES: dict[str, str] = {
    "README.md": "README",
    "glia-todo.md": "todo",
    "mise.toml": "mise.toml",
    "mise.lock": "mise.lock",
}

PATH_PREFIX_MATCHES: list[tuple[tuple[str, ...], str]] = [
    ((".config", "mise", "conf.d"), "mise/{filename_no_ext}"),
    (("backend", "migrations", "versions"), "backend/migrations/versions"),
]

NO_SCOPE_STR = "None"
NOTHING_STAGED_STR = "Nothing Staged"
