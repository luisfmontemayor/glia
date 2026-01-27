# Glia

> [!NOTE]  
> *Currently a work in progress*

A telemetry suite for technical data teams working on joint linux systems.

## Getting started
1. If you don't have `mise` installed: [Install mise](https://mise.jdx.dev/getting-started.html), a tool for managing installed language runtimes and environment variables.
2. [Install Docker](https://www.docker.com/get-started/)
3. Install the following dependencies: `libgit2-dev`, `libsecret-1-dev`, `libxml2-dev`, `libfribidi-dev`, `libharfbuzz-dev`, `libtiff-dev` and `libwebp-dev`


### Getting Started
1. Run `mise run full-setup`, which in turn:
   - Runs `mise run setup-deps`
      - Install Python, R, uv, gum, and ruff.
      - Sync python dependencies using `uv`.
      - Configure `lazygit` integration by copying the configuration file.
      
   - Runs `mise run setup-db`
      - Sets up postgres env variables.

## Development
### Linting & Formatting
This project uses [Ruff](https://docs.astral.sh/ruff/) for both linting and formatting.
1. `ruff check .` to lint.
2. `ruff format .` to format.

## Conventional commits
Commit messages in this repository follow the [conventional commits specification](https://www.conventionalcommits.org).

There are two ways to make compliant commits easily:

### 1. Using Lazygit (Recommended)
The setup script configures a custom command for `lazygit`.
- Press `C` in the **Files** panel to open the Conventional Commit menu.
- It will automatically suggest scopes based on your staged files.

### 2. Using the CLI Helper
You can run the helper script directly:
```bash
python3 -m scripts.conventionalCommits
```

## VS Code config
Recommended VS Code extensions (configured in `.vscode/extensions.json`):
* [Python](https://marketplace.visualstudio.com/items?itemName=ms-python.python)
* [Ruff](https://marketplace.visualstudio.com/items?itemName=charliermarsh.ruff)
* [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

## Project Structure
- **Backend**: Built with [FastAPI](https://fastapi.tiangolo.com/) (Python).
- **Clients**: Written in their respective languages, for Python and R
