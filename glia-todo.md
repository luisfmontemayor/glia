# Glia Clients Implementation Roadmap

## Phase 1: Python Client Foundation & Metrics
*Goal: Create the `glia-python` package and implement the logic to capture raw system telemetry.*
- [ ] Initialize `glia-python` project structure using `uv`.
- [ ] Add core dependency: `psutil` (for cross-platform system monitoring).
- [ ] Implement `SystemTracker` class:
  - [ ] Capture **CPU Usage** (User/System time).
  - [ ] Capture **RAM Usage** (Peak RSS).
  - [ ] Capture **Wall Time** (Start/End deltas).
  - [ ] Capture **I/O Counters** (Read/Write bytes).
- [ ] Implement Metadata collection:
  - [ ] Hostname, User, OS version.
  - [ ] Script context (filename, arguments).

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







# Setup
-- [x] Lock files
    
- [x] A task runner
- [x] CCs 
- [x] Basic dir structure
- [x] Dependencies
    - []? Make a Makefile
    - Manual installs
        - [x] Docker
        - [x] mise
    - General dependencies:
        - [x] R
        - [x] Python
            - [x] FastAPI
    - dev dependencies
        - [x] gum
        
- [ ] Docs
    - [x] Basic README
- [ ] Identify testing
- [ ] Git
    - [x] Merge branches protections, only stiff I approve can be merged
    - [x] merged branches get deleted


# Backend
- [x] Data Schema
    - [x] Choose compulsory variables
        - run_id
        - program_name
        - user_name
        - script_sha256
        - exit_code_int
        - started_at
        - ended_at
        - cpu_time_sec
        - cpu_percent
        - max_rss_mb
    
# Clients
- [ ] Architecture - use
    - [ ] How will it actually be implemented in R?
    - [ ] How will it actually be implemented in Python?    

 
 
 # give me a summary of the entirety of the glia project for a new chat to understand what it has become thus far been taken thus far, in a copyable markdown code block
# Philosophy
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