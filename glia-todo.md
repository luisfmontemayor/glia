# Configuration & Bug Fixes
- [x] **Mise: Centralize configuration**
    - [x] Add `API_HOST` and `CORE_FLUSH_TIMEOUT_SEC` to `mise.toml`.
    - [x] Use `API_HOST` in `API_INGEST_URL` instead of `POSTGRES_HOST`.
- [x] **Core: Use configurable flush timeout**
    - [x] Update `core/src/core.rs` to use `CORE_FLUSH_TIMEOUT_SEC` instead of hardcoded 5s.
- [x] **Python Client: Remove hardcoded defaults**
    - [x] Remove hardcoded 2.0s timeout in `push_telemetry`.
    - [x] Ensure it strictly respects `API_INGEST_URL`.
- [x] **R Client: Remove hardcoded defaults**
    - [x] Remove hardcoded "http://localhost:8000" in `glia_init`.
    - [x] Remove hardcoded 10.0s timeout in `GliaClient$new`.
- [x] **Validation: Ensure TDD coverage**
    - [x] Update core tests to verify `CORE_FLUSH_TIMEOUT_SEC`.

- [x] **TDD: Implement batching tests in `core/src/core.rs`**
    - [x] Add `test_batching_by_count`: Verify 1000 items are sent as one batch.
    - [x] Add `test_batching_by_time`: Verify 1 item is sent after 2 seconds.
    - [x] Add `test_flush_drains_buffer`: Verify `flush()` sends items even if buffer is not full or timed out.
    - [x] Add `test_batching_with_env_vars`: Verify configurability via `CORE_BATCH_SIZE` and `CORE_BATCH_TIMEOUT_SEC`.
- [x] **Core: Implement batching logic**
    - [x] Update worker thread to use a buffer (`Vec<String>`).
    - [x] Implement a select-style loop with timeout for channel reception.
    - [x] Implement batch merging logic (re-wrap JSON list items into a single list).
    - [x] Ensure `Flush` message drains the buffer before acknowledging.
    - [x] Make batch size and timeout configurable via environment variables (`CORE_BATCH_SIZE`, `CORE_BATCH_TIMEOUT_SEC`).
    - [x] **Refactor: Move to async `tokio` channels**
        - [x] Replace `crossbeam_channel` with `tokio::sync::mpsc`.
        - [x] Update `Cargo.toml` with `time` and `macros` features for tokio.
        - [x] Fix `mockito` tests to use `async/await` and `tokio::test`.
- [x] **Validation: Cross-client compatibility**
    - [x] Verify `glia_python` still works (it currently wraps single items in `[]`).
    - [x] Verify `gliar` still works.



# Make README pitch-ready
- [x] Identify and document missing system dependencies (`cmake`, `libuv1-dev`, `pandoc`)
- [ ] Readme needs r installation path mapping
- [ ] cli gui 
 

#######
- [ ] remove return from R, archaic
- [ ] encrypt comms, start with https
- [ ] Identify the currently hardcoded vars to put in a config file / mise toml
- [ ] scopes do not add most common ancestor (gliar/1/2 and gliar/1/3 list gliar as common and not gliar/1) 
- [ ] TODO: rename to something that reflects that this Enqueues the payload to a background worker.
- [ ] cli gui 
- [ ] Readme needs r installation path mapping 
- [ ] change walltime to ms not secs and make into int
- [ ] health check: is it legit  
- [ ] cloud, by using the sandbpx software for aws, k8 and ansible
- [ ] downgrade to 3.10 for better interop
- [ ] change python version comment to use with mise run sync python or whatever it is
- [ ] lazygit plugin: no files staged means it puts in messed up scope label
- [ ] store memory as kb instead of mb
- [ ] work on type safety
- [ ] **Future:** Add I/O metrics to models and clients (Postponed).
- [ ] An interactive mode
- [ ] API_INJEST_URL to GLIA_INJEST_URL
- [ ] remove return from R
- [ ] backend migrations fix, so that scope doesn't include hash  but ends at backend/migrations/versions






## Technical Context
- **Backend:** FastAPI (Python 3.12+)
- **Database:** PostgreSQL (Optimized for high-frequency writes)
- **Infrastructure:** Docker / Compose
- **Tooling:** `uv` (Python), `mise` (Env management), `conventional commits`

## Technical Enemies:
- The client having too much observer effect error
- Errors because of far too large pool of jobs
- Data missing because of intermittent connection
- Data missing because of overwhelmed backend



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
    - [x] Add cmake to mise tools
    - Manual installs
        - [x] Docker
        - [x] mise
    - General dependencies:
        - [x] R
        - [x] Python
            - [x] FastAPI
    - dev dependencies
        - [x] gum
    - [x] make dev dependencies (cmake, etc documented in README)
- [x] Docs
    - [x] Basic README
- [x] Identify testing
    - [x] Backend TDD
    - [x] E2E Integration tests
    - [x] Fixed broken mise task paths
- [x] Git
    - [x] Merge branches protections, only stuff I approve can be merged
    - [x] merged branches get deleted
    - [x] Consolidated atomic commit strategy

### Backend
- [x] Data Schema
    - [x] Choose compulsory variables
        - run_id, program_name, user_name, script_sha256, exit_code_int, started_at, ended_at, cpu_time_sec, cpu_percent, max_rss_mb
