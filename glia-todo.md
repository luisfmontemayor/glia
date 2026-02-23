# Glia Clients Implementation Roadmap

## Phase 1: Python Client Foundation & Metrics
- [x] Initialize `glia_python` project structure using `uv`.
- [x] Add core dependency: `psutil`.
- [x] Implement `SystemTracker` class (CPU, RAM, Wall Time).
- [x] Implement Metadata collection (Hostname, User, OS, Script context).

## Phase 2: Python Wrapper & Developer Experience (DX)
- [x] Design the `Glia` singleton/main entry point.
- [x] Implement Context Manager (`with Glia.tracker(): ...`).
- [x] Implement Decorator (`@Glia.track`).
- [x] Add support for Custom Values/Metadata.

## Phase 3: Shared Core & Python Network Layer
- [x] Initialize `glia_core` Rust crate.
- [x] **Core Logic**: Implement `perform_push` using `reqwest`.
- [x] **Python Bindings**: Implement `pyo3` wrapper.
- [x] **Integration**: Update `glia_python` to use `glia_core`.

## Phase 4: The R Client Implementation (`gliar`)
- [x] Initialize `gliar` package structure using `rextendr`.
- [x] **Sub-Phase 1: Rust Integration (The "Glue")**
    - [x] Configure `glia_core` to support R builds (feature flags `python` vs `r`).
    - [x] Write R-specific Rust wrapper `r_module.rs` (Extendr).
    - [x] Link `gliar` to `glia_core` via local path dependency.
- [x] **Sub-Phase 2: Metrics Foundation (Pure R)**
    - [x] Add dependencies: `ps`, `R6`, `digest`, `uuid`.
    - [x] Implement `SystemTracker` (R6 class).
        - [x] Capture **RAM** (Using `ps::ps_memory_info`).
        - [x] Capture **CPU/Wall Time** (Using `ps::ps_cpu_times` and `Sys.time`).
        - [x] Capture **Metadata** (Sys.info, script path).
- [x] **Sub-Phase 3: Network Layer**
    - [x] Expose `push_telemetry` from `glia_core` to R namespace.
    - [x] Implement `GliaClient` R6 wrapper to call the Rust function.
- [x] **Sub-Phase 4: Developer Experience & Management**
    - [x] Implement user-facing `glia_init()` or higher-level wrapper in `R/glia.R`.
    - [x] Setup declarative dependency management (`renv` + `DESCRIPTION`).

## Phase 5: Monorepo Orchestration & Verification
- [x] Modularize `mise` tasks.
- [x] Configure "Polyglot" builds (`rust:develop`, `rust:build-r`).
- [x] **End-to-End Verification**:
    - [x] Run Python client against `localhost:8000`.
    - [x] Run R client against `localhost:8000`.
    - [x] Verify Data Integrity (JSONB fields, RAM accuracy).

#######
-  [ ] // TODO let ingest be passed from param and not acted upon, in core.rs
- [ ] Readme needs r installation path mapping 
- [ ] change walltime to ms not secs and make into int
- [ ] health check: is it legit  
- [ ] cloud, by using the sandbpx software for aws, k8 and ansible
- [ ] cli gui 
- [ ] mise scope, with immediately after being mise/mise.toml or mise/gliar for example
- [ ] add readme to scope
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