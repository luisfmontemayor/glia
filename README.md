# Glia

> [!NOTE]  
> *Currently a work in progress*

A telemetry suite for technical data teams working on joint linux systems.

## Installation & Setup
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

## Usage
Currently, glia clients include `glia_python` for Python and `gliar` for R:

### Python Client
The Python client uses a `JobTracker` to automatically capture execution metrics for your scripts or specific code blocks.

1.  **Initialize and Track (Context Manager)**:
    The easiest way to use the tracker is as a context manager. It will automatically start tracking on entry and capture/send metrics on exit (including error handling).

    ```python
    from glia_python.tracker import JobTracker

    with JobTracker(program_name="my_python_script", context={"dataset": "MNIST"}) as tracker:
        # Your main script logic here
        print("Running a simulated data processing task...")
        import time
        time.sleep(2)
        tracker.log_metadata({"epochs": 10, "accuracy": 0.95})
        print("Task completed.")
    ```

2.  **Manual Start and Capture**:
    For more granular control, you can manually start and capture metrics.

    ```python
    from glia_python.tracker import JobTracker

    tracker = JobTracker(program_name="my_custom_task")
    tracker.start() # Start tracking manually

    try:
        print("Performing some operations...")
        # ... Your code ...
        tracker.log_metadata({"status": "success"})
        exit_code = 0
    except Exception as e:
        print(f"An error occurred: {e}")
        tracker.log_metadata({"error_message": str(e)})
        exit_code = 1
    finally:
        # Capture metrics with the final exit code
        metrics = tracker.capture(exit_code=exit_code)
        # Manually push telemetry
        from glia_python.network import push_telemetry
        push_telemetry(metrics)
    ```

### R Client

The R client offers functions to initialize the tracking, track expressions, and wrap functions for automatic telemetry.

1.  **Initialize the Client**:
    Configure the global Glia client with your API endpoint and optional global metadata. This should typically be done once at the start of your R session or script.

    ```R
    library(gliar)

    glia_init(
      api_url = "http://localhost:8000/ingest", # Or set GLIA_API_URL environment variable
      app_name = "MyRAnalysis",
      app_version = "1.0.0",
      tags = list(project = "research")
    )
    ```

2.  **Track an R Expression**:
    Use `glia_track()` to execute an R expression and automatically capture metrics.

    ```R
    # Track a block of code
    result <- glia_track({
      message("Starting complex calculation...")
      Sys.sleep(1.5)
      sum(rnorm(1000))
    },
    name = "ComplexCalculation",
    description = "Calculates sum of 1000 random normal values",
    tags = list(stage = "modeling")
    )
    print(paste("Calculation result:", result))
    ```

3.  **Wrap a Function for Tracking**:
    Use `glia_wrap()` to create a new version of an existing function that automatically tracks its execution every time it's called.

    ```R
    my_function <- function(data) {
      Sys.sleep(0.1)
      mean(data)
    }

    # Create a tracked version
    tracked_my_function <- glia_wrap(my_function, name = "MeanCalculation")

    # Call the tracked function
    data_sample <- 1:100
    mean_val <- tracked_my_function(data_sample)
    print(paste("Mean value:", mean_val))
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
