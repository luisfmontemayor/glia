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

## Phase 3: Python Network Layer
*Goal: Serialize the telemetry and push it to the running Backend API.*
- [x] Add dependency: `httpx` (for robust HTTP requests).
- [x] Create the Payload Builder: Map `SystemTracker` data to the JSON schema defined in the Backend.
- [x] Implement `send_telemetry()`:
  - [x] Configuration handling (API URL via env vars or config file).
  - [x] Error handling (Suppress connection errors to prevent crashing the main job).
  - [x] Timeout management (fail fast if the backend is unreachable).

## Phase 4: The R Client Implementation (`glia-r`)
*Goal: Replicate the Python architecture (Metrics -> DX -> Push).*
- [ ] Initialize `glia-r` package structure.
- [ ] **Sub-Phase 1: Metrics Foundation**
    - [ ] Add dependencies: `processx` (or base `gc`/`proc.time`), `jsonlite`.
    - [ ] Implement `SystemTracker` (R6 or S3 class).
        - [ ] Capture **RAM** (Using `gc()` reset/diff or `OS` calls).
        - [ ] Capture **CPU/Wall Time** (Using `proc.time()`).
        - [ ] Capture **Metadata** (R version, Platform, User).
- [ ] **Sub-Phase 2: Developer Experience (DX)**
    - [ ] Implement `glia_init()` / `glia_tracker` (Context object).
    - [ ] Implement Function Wrapper (equivalent to Python Decorator).
- [ ] **Sub-Phase 3: Network Layer**
    - [ ] Add dependency: `httr2`.
    - [ ] Implement `glia_send()` to push JSON payload.

## Phase 5: End-to-End Verification
*Goal: Verify that ephemeral jobs actually populate the database.*
- [ ] Run the Python client in a script against `localhost:8000`.
- [ ] Run the R client in a script against `localhost:8000`.
- [ ] Verify Data Integrity:
  - [ ] Check if `JSONB` custom fields are queried correctly.
  - [ ] Confirm "Peak RAM" numbers look realistic compared to OS monitors.



#######
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