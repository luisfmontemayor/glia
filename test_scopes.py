from pathlib import Path
from scripts.conventional_commits.scopes import add_scope_category

def test_scope_category():
    test_cases = [
        ("backend/migrations/versions/0c69793f9b18_.py", "backend/migrations/versions"),
        ("backend/main.py", "backend/main"),
        ("glia_python/src/glia_python/network.py", "glia_python/network"),
        ("README.md", "README"),
        ("mise.toml", "mise.toml"),
        (".config/mise/conf.d/api.toml", "mise/api"),
    ]

    for filepath, expected in test_cases:
        result = add_scope_category(filepath)
        print(f"File: {filepath} -> Result: {result} (Expected: {expected})")
        assert result == expected

if __name__ == "__main__":
    test_scope_category()
