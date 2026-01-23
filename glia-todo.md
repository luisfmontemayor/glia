# Glia Clients Implementation Roadmap

## Phase 1: Python Client Foundation & Metrics
*Goal: Create the `glia_python` package and implement the logic to capture raw system telemetry.*
- [x] Initialize `glia_python` project structure using `uv`.
- [x] Add core dependency: `psutil` (for cross-platform system monitoring).
- [x] Implement `SystemTracker` class:
  - [x] Capture **CPU Usage** (User/System time).
  - [x] Capture **RAM Usage** (Peak RSS).
  - [x] Capture **Wall Time** (Start/End deltas).
- [x] Implement Metadata collection:
  - [x] Hostname, User, OS version.
  - [x] Script context (filename, arguments).

## Phase 2: Python Wrapper & Developer Experience (DX)
*Goal: Abstract the complexity so developers can instrument code with a single line.*
- [x] Design the `Glia` singleton or main entry point.
- [x] Implement the Context Manager (`with glia.tracker(): ...`) for scoping specific blocks.
- [x] Implement the Decorator (`@glia.track`) for function-level monitoring.
- [x] Add support for **Custom Values** (allowing users to pass a dictionary of extra metrics/tags).

## Phase 3: Shared Core & Python Network Layer
*Goal: Move network logic to a high-performance Rust core and bind it to Python.*
- [x] Initialize `glia_core` Rust crate.
- [x] **Core Logic**: Implement `perform_push` using `reqwest` (blocking client).
  - [x] Automatic URL formatting and JSON serialization.
  - [x] Timeout handling and error mapping.
- [x] **Python Bindings**: Implement `pyo3` wrapper.
  - [x] Map Rust `PushResult` to Python class.
  - [x] Expose `push_telemetry` function to Python.
- [x] **Integration**: Update `glia_python` to use `glia_core` instead of `httpx`.

## Phase 4: The R Client Implementation (`glia_r`)
*Goal: Replicate the Python DX while leveraging the existing Rust core for networking.*
- [x] Initialize `glia_r` package structure using `rextendr`.
- [x] **Sub-Phase 1: Rust Integration (The "Glue")**
    - [x] Configure `glia_core` to support R builds (feature flags `python` vs `r`).
    - [x] Write R-specific Rust wrapper `r_module.rs` (Extendr).
    - [ ] Link `glia_r` to `glia_core` via local path dependency in `Cargo.toml`.
    - [ ] Verify `glia_r` compilation excludes Python symbols.
- [ ] **Sub-Phase 2: Metrics Foundation (Pure R)**
    - [ ] Add dependencies: `processx` (or base `gc`/`proc.time`) via `DESCRIPTION`.
    - [ ] Implement `SystemTracker` (R6 or S3 class).
        - [ ] Capture **RAM** (Using `gc()` reset/diff or OS calls).
        - [ ] Capture **CPU/Wall Time** (Using `proc.time()`).
        - [ ] Capture **Metadata** (R version, Platform, User).
- [ ] **Sub-Phase 3: Network Layer**
    - [ ] Expose `push_telemetry` from `glia_core` to R namespace.
    - [ ] Implement `glia_send()` wrapper in R to call the Rust function.
- [ ] **Sub-Phase 4: Developer Experience & Management**
    - [ ] Implement `glia_init()` / `glia_tracker` (Context object).
    - [ ] Setup declarative dependency management (`renv` + `DESCRIPTION`).

## Phase 5: Monorepo Orchestration & Verification
*Goal: robust build tools and end-to-end testing.*
- [x] Modularize `mise` tasks (split `mise.toml` into `.mise/tasks/*` scripts).
- [x] Configure "Polyglot" builds:
    - [x] `rust:develop` for Python.
    - [x] `rust:build-r` for R.
- [ ] **End-to-End Verification**:
  - [ ] Run the Python client in a script against `localhost:8000`.
  - [ ] Run the R client in a script against `localhost:8000`.
  - [ ] Verify Data Integrity (JSONB fields, RAM accuracy).


#######
- [ ] Add client build and installs to mise
- [ ] downgrade to 3.10 for better interop
- [ ] change python version comment to use with mise run sync python or whatever it is
- [ ] lazygit plugin: no files staged means it puts in messed up scope label
- [ ] store memory as kb instead of mb
- [ ] work on type safety
- [ ] **Future:** Add I/O metrics to models and clients (Postponed).
- [ ] An interactive mode

## 🛠️ Technical Context
- **Backend:** FastAPI (Python 3.12+)
- **Database:** PostgreSQL (Optimized for high-frequency writes)
- **Infrastructure:** Docker / Compose
- **Tooling:** `uv` (Python), `mise` (Env management), `conventional commits`

### v0 Scope
- 2 Clients: Python and R (Single script model).
- 1 Data Pipeline: Client -> FastAPI -> PostgreSQL.

### v1 Scope
- CLI client for bash scripting
- Nextflow program for scripting
- Queue and queue worker implemented
- I/O Metrics implementation



### Setup
- [x] Lock files
- [x] A task runner
- [x] CCs 
- [x] Basic dir structure
- [x] Dependencies
    - [ ]? Make a Makefile
    - Manual installs
        - [x] Docker
        - [x] mise
    - General dependencies:
        - [x] R
        - [x] Python
            - [x] FastAPI
    - dev dependencies
        - [x] gum
    - [ ] make dev dependencies
- [ ] Docs
    - [x] Basic README
- [ ] Identify testing
- [ ] Git
    - [x] Merge branches protections, only stuff I approve can be merged
    - [x] merged branches get deleted

### Backend
- [x] Data Schema
    - [x] Choose compulsory variables
        - run_id, program_name, user_name, script_sha256, exit_code_int, started_at, ended_at, cpu_time_sec, cpu_percent, max_rss_mb