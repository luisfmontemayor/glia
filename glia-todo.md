# Glia Clients Implementation Roadmap

## Phase 1: Python Client Foundation & Metrics
*Goal: Create the `glia_python` package and implement the logic to capture raw system telemetry.*
- [x] Initialize `glia_python` project structure using `uv`.
- [x] Add core dependency: `psutil` (for cross-platform system monitoring).
- [x] Implement `SystemTracker` class:
  - [x] Capture **CPU Usage** (User/System time).
  - [x] Capture **RAM Usage** (Peak RSS).
  - [x] Capture **Wall Time** (Start/End deltas).
  - [x] Capture **I/O Counters** (Read/Write bytes).
- [x] Implement Metadata collection:
  - [x] Hostname, User, OS version.
  - [x] Script context (filename, arguments).

## Phase 2: Python Wrapper & Developer Experience (DX)
*Goal: Abstract the complexity so developers can instrument code with a single line.*
- [ ] Design the `Glia` singleton or main entry point.
- [ ] Implement the Context Manager (`with glia.tracker(): ...`) for scoping specific blocks.
- [ ] Implement the Decorator (`@glia.track`) for function-level monitoring.
- [ ] Add support for **Custom Values** (allowing users to pass a dictionary of extra metrics/tags).
- [ ] Implement **Data Footprint** tracking (logic to accept file paths/directories and calculate sizes).

## Phase 3: Python Network Layer
*Goal: Serialize the telemetry and push it to the running Backend API.*
- [ ] Add dependency: `httpx` (for robust HTTP requests).
- [ ] Create the Payload Builder: Map `SystemTracker` data to the JSON schema defined in the Backend.
- [ ] Implement `send_telemetry()`:
  - [ ] Configuration handling (API URL via env vars or config file).
  - [ ] Error handling (Suppress connection errors to prevent crashing the main job).
  - [ ] Timeout management (fail fast if the backend is unreachable).

## Phase 4: The R Client Implementation (`glia-r`)
*Goal: Replicate the push-architecture for R scripts and ephemeral jobs.*
- [ ] Initialize `glia-r` package structure.
- [ ] Add dependencies: `httr2` (networking), `jsonlite` (serialization), `processx` or base `gc` (metrics).
- [ ] Implement `glia_init()` and `glia_send()` functions.
- [ ] Implement resource tracking in R:
  - [ ] Use `gc()` logic for memory.
  - [ ] Use `proc.time()` for CPU/Wall time.
- [ ] Ensure API payload compatibility with the Python backend models.

## Phase 5: End-to-End Verification
*Goal: Verify that ephemeral jobs actually populate the database.*
- [ ] Run the Python client in a script against `localhost:8000`.
- [ ] Run the R client in a script against `localhost:8000`.
- [ ] Verify Data Integrity:
  - [ ] Check if `JSONB` custom fields are queried correctly.
  - [ ] Confirm "Peak RAM" numbers look realistic compared to OS monitors.






#######
- [ ] lazygit plugin: no files staged means it puts in messed up scope label
- [ ] store memory as kb instead of mb
- [ ] add i/o metrics to models and to client
- [ ] work on type safety

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


### Clients
- [ ] Architecture - use
    - [ ] How will it actually be implemented in R?
    - [ ] How will it actually be implemented in Python?
- [ ] Phase 1: Python `SystemTracker` (CPU, RAM, Wall Time, I/O)
- [ ] Phase 2: Python DX (Context Manager `with glia.tracker()`, Decorators)
- [ ] Phase 3: Python Network Layer (`httpx` push logic)
- [ ] Phase 4: R Client Implementation (`httr2`, `gc()`, `proc.time()`)



---

## 🧠 Philosophy & Goals
### The Problem
- Gauge tool usage and adoption vs alternatives.
- Passive benchmarking without manual user intervention.

### Stakeholder Value
- **The User:** See performance gains without engineering friction.
- **The Dev/Admin:** Verify tool efficiency and optimize performance based on real-world telemetry.

- [ ] What am I actually trying to solve?
    - I want to gauge how much my tool is being used, to see how much adoption it has vs other alternatives
    - I want to have ways of doing benchmarking passively, without having to request people doing to submit their stats
- [ ] What do people actually care about?
    ## The User
    - Their gains vs previous program
    - Not being slowed down by engineerial details
    ## The Dev / Admin
    - Ensuring people are using the new tools
    - That new tool is actually more efficient
    - Ways to optimise their tool
- [ ] The max scope of v0
    - 2 clients: Python and R
        - Collection of data - single script model
    - 1 way of pushing data to database
    - 1 database
    - 1 way of getting a JSON dump
- [ ] The max scope of v0
    - 1 Nextflow client