#!/bin/bash
set -euo pipefail

print_success() {
    printf "\033[0;36mSuccessfully copied the repo desc!\033[0m\n"
}

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
DESC="${repo_root}/../llm-project-summary.txt"
if [ ! -f "$DESC" ]; then
    echo "Error: File '$DESC' not found."
    exit 1
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
    pbcopy < "$DESC"
    print_success
elif command -v wl-copy &> /dev/null; then
    wl-copy < "$DESC"
    print_success
elif command -v clip.exe &> /dev/null; then
    cat "$DESC" | clip.exe
    print_success
elif command -v xclip &> /dev/null; then
    xclip -selection clipboard < "$DESC"
    print_success
else
    echo "Error: No clipboard utility (pbcopy, wl-copy, xclip, clip.exe) found."
    exit 1
fi
