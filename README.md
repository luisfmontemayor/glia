# Glia

> [!NOTE]  
> *Currently a work in progress*

A telemetry suite for technical data teams working on joint linux systems.

## Setup
1. If you don't have `mise` installed: [Install mise](https://mise.jdx.dev/getting-started.html), a tool for managing language runtimes and environment variables.
2. [Install Docker](https://www.docker.com/get-started/).
3. Run `mise run setup:all`. This command will:
   - Install all required language runtimes (Python, R, Rust).
   - Install all project dependencies (Python packages via `uv`, R packages via `renv`).
   - Set up the local PostgreSQL database in a Docker container.
   - Configure helper tools like `lazygit`.

### R Environment & Dependency Management
This project uses **renv** to manage R dependencies in an isolated, project-local library within the `gliar/` directory.

1. **Installation**: Run `mise run setup:r-deps` to initialize the environment and install packages defined in `gliar/DESCRIPTION`.
2. **Library Mapping**: The setup task automatically generates a `.env.r` file in the project root. This file defines `R_LIBS_USER`, pointing R to the local library.
3. **Automatic Activation**: `mise` is configured to automatically load `.env.r`. When you run R commands via `mise run` or within a `mise`-activated shell, your session will correctly use the project-local dependencies.

If you are not using `mise` for your R session, you can manually source the configuration:
```bash
source .env.r
R
```

## Development
This section provides guidelines for contributing to the project.

### Linting & Formatting
We use `ruff` for Python and `lintr` for R to maintain code quality and consistency. The easiest way to run them is via `mise`:

- **Run all linters**:
  ```bash
  mise run lint:all
  ```
- **Run and fix Python files**:
  ```bash
  mise run lint:python
  ```
- **Run R linter**:
  ```bash
  mise run lint:gliar
  ```

Key linting rules include:
- **Python (Ruff)**: Enforces removal of unused imports, consistent import sorting, and the use of modern Python syntax (e.g., `A | B` for type hints instead of `Union[A, B]`).
- **R (lintr)**: Discourages explicit `return()` calls in favor of implicit returns, following the Tidyverse style guide.

Configuration files (`ruff.toml` and `.lintr`) are available to fine-tune the rules.

### Conventional commits
Commit messages in this repository follow the [conventional commits specification](https://www.conventionalcommits.org).

There are two ways to make compliant commits easily:

#### 1. Using Lazygit (Recommended)
The setup script configures a custom command for `lazygit`.
- Press `C` in the **Files** panel to open the Conventional Commit menu.
- It will automatically suggest scopes based on your staged files.

#### 2. Using the CLI Helper
You can run the helper script directly:
```bash
python3 -m scripts.conventional_commits
```

### VS Code Config
Recommended VS Code extensions (configured in `.vscode/extensions.json`):
* [Python](https://marketplace.visualstudio.com/items?itemName=ms-python.python)
* [Ruff](https://marketplace.visualstudio.com/items?itemName=charliermarsh.ruff)
* [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

## Project Structure
- **Backend**: Built with [FastAPI](https://fastapi.tiangolo.com/) (Python).
- **Clients**: Written in their respective languages, for Python and R
