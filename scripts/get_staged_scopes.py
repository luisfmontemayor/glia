#!/usr/bin/env -S uv run
# Written by Luis Felipe Montemayor, sometime around December of 2025
from conventional_commits import scopes

if __name__ == "__main__":
    for file in scopes.get_staged_scopes():
        print(file)
