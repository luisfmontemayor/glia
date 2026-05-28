# To Do
## Test Audit Bugs (2026-05-26)
### CRITICAL
- [ ] Python `network_test.py`: `.value` instead of `.return_value` on `mock_getenv` — test passes vacuously
- [ ] Backend `main_test.py`: tests hit the real Postgres DB — no session override, no test DB, no isolation
- [ ] Backend `main_test.py`: requires a live running server — integration tests disguised as unit tests, use `ASGITransport` instead
- [ ] R `test-glia.R`: namespace mocks (`SystemTracker`, `.glia_env$client`) leak across tests — no teardown/`withr::defer`
- [ ] R `test-tracker.R`: `mockery::stub` on `ps::ps_cpu_times` silently ignores process handle — tracker could pass `NULL` and tests pass

### WARNING
- [ ] Python `network_test.py`: missing `_global_config.clear()` autouse fixture — stale config can skip env var path
- [ ] Python `tracker_test.py`: 5 tests leak real `push_telemetry` → FFI calls through `__exit__` (L47, L65, L139, L149, L159)
- [ ] Python `tracker_test.py`: `platform` mock incomplete — `time.time()` and `psutil` leak real system values
- [ ] Backend `main_test.py`: cleanup uses production engine directly — no env guard against accidental prod `DELETE`
- [ ] Backend `test/`: no `conftest.py` with shared fixtures for test DB, client, or cleanup
- [ ] Backend `main_test.py`: assertions only check `status_code` + `run_id` — no field integrity verification
- [ ] R `test-glia.R`: mock tracker `capture()` returns 2 fields vs real 15+ — payload shape contract untested
- [ ] R `test-glia.R`: no assertion on payload contents passed to `send_job_run` — only call count checked
- [ ] R `test-network.R`: `GliaClient$new()` with no args depends on `GLIA_API_URL` env var from runner
- [ ] R `test-tracker.R`: test 2 bypasses `start()` by setting private fields directly — `start()` correctness untested

### INFO
- [ ] Backend: no tests for error paths (invalid payloads, DB unreachable, duplicate `run_id`)
- [ ] Backend: `GET /telemetry` endpoint has zero test coverage
- [ ] Backend: `test_api_status` shells out to `mise run api:status` — tests infra, not code
- [ ] R `test-network.R`: `mockery::stub` on R6 methods is brittle — could break with R6 version changes
- [ ] R `test-tracker.R`: `stub(tracker$capture, "Sys.time", Sys.time)` is a no-op identity stub
- [ ] Python `network_test.py`: smoke tests (`test_rust_panic_caught`, `test_non_utf8_payload`) call real FFI — integration tests in unit test file
## Testing
- [ ] test with more data
- [ ] verifyt that tests are not silly, that they're actually sensible
- [x] ensure that benchmarks tests keeps a library with the git commit hash, runtime and setup type (local/distributed/k8s, whatever setup)
- [x] fix: why does backend to db benchmark not show any throughput:
```
## Benchmark:   Backend-to-DB
Latency (ms):           268.09      719.26     2976.05     6436.28     9690.97
Throughput (j/s):         0.00        0.00        0.00        0.00        0.00
```

## Conventional Commits & Git History
- [ ] revise commit history to see which commits can be squashed together, which can be shifted around, big nice cleanup of commit history
- [ ] **Conventional Commits Enhancement**: Add `docs` as a valid scope category for documentation changes.

## Codebase Refactoring & Type Safety
- [ ] work on type safety

## Features & Queue System
- [ ] **Future:** Add I/O metrics to models and clients (Postponed).
- [ ] An interactive mode
- [ ] CLI client for bash scripting
- [ ] Nextflow program for scripting
- [ ] Queue and queue worker implemented
- [ ] I/O Metrics implementation

## Infrastructure, Cloud & Security
- [ ] encrypt comms, start with https?
- [ ] cloud, by using the sandbox software for aws, k8 and ansible

## Documentation
- [ ] Make README pitch-ready

## Client Resilience & Connection Handling - potential problems
- [ ] The client having too much observer effect error
- [ ] Errors because of far too large pool of jobs
- [ ] Data missing because of intermittent connection
- [ ] Data missing because of overwhelmed backend

## Architecture Context (Notes)
- **Backend:** FastAPI (Python 3.12+)
- **Database:** PostgreSQL (Optimized for high-frequency writes)
- **Infrastructure:** Docker / Compose
- **Tooling:** `uv` (Python), `mise` (Env management), `conventional commits`
- 2 Clients: Python and R (Single script model).
- 1 Data Pipeline: Client -> FastAPI -> PostgreSQL.

## Completed / Moved General Tasks
- [x] add env var to add cores to the R installation (Moved to Orchestration checklist at top)
- [x] Readme needs r installation path mapping (Done)
- [x] Test the R installation in a bare container (Done, automated via `Dockerfile.Rtest`)
  - **Plan**: Create a Dockerfile starting from a minimal Ubuntu image. Install `R`, `rustc`, `cargo`, `mise`, and all identified system depend>
- [x] **Mise R Plugin Integration** (Moved to Orchestration checklist at top):
    - Add the custom `mise-r` plugin URL to `mise.toml` so `mise install` works properly for new users.
    - Update `test/installations/Dockerfile.Rtest` to verify the unbiased installation of R through the plugin.
- [x] Step 1: Python Client Config & Interactive Logic. Implement pseudo-stateless config and interactive detection in glia_python.
- [x] Step 2: Python Tests. Provide test coverage for the Python configuration changes.
- [x] Step 3: R Client Config & Warn-and-Drop Fallback. Convert R to pseudo-stateless, add warn-and-drop, and interactive detection.
- [x] Step 4: R Tests. Provide test coverage for the R configuration changes.
- [x] add most up to date conventional commits and make sure that the descriptions files for it are retained
- [x] Readme needs r installation path mapping
  - **Plan**: Document how `mise run setup:r-deps` initializes the `renv` environment and generates a `.env.r` file with `R_LIBS_USER` for corr>

# In Progress
- [/] downgrade to 3.10 for better interop (Done in `downgrade-python-3.10`)


# Done
- [x] Identify and document missing system dependencies (`cmake`, `libuv1-dev`, `pandoc`)
- [x] health check: is it legit (Endpoints identified: `/health/live`, `/health/ready`)
- [x] lazygit plugin: no files staged means it puts in messed up scope label (Done in `fix-conventional-commits`)
- [x] backend migrations fix (Done in `fix-conventional-commits`)
- [x] TODO: rename to something that reflects that this Enqueues the payload to a background worker.
- [x] Make hardcoded "http://localhost" in tests and benchmarks configurable via environment variables.
- [x] **Refine Installation Tests**:
    - [x] Update `test/installations/README.md` with detailed instructions.
    - [x] Improve `test/installations/build_rtest.sh` with logging and error handling.
- [x] **Documentation Sync**:
    - [x] Verify root `README.md` contains R installation path mapping and `mise` integration.
- [x] **Cleanup**:
    - [x] Confirmed `test/installations/Dockerfile.testclone` is already removed.
- [x] **Update Root README**:
    - Document the R installation process using `mise run setup:r-deps`.
    - [x] Explain that `renv` is used for dependency management and it generates a `.env.r` file.
    - [x] Clarify that `mise` automatically loads `.env.r` to set `R_LIBS_USER`, ensuring R uses the project-local library.
- [x] **Cleanup & Automate Installation Tests**:
    - [x] Remove redundant `test/installations/Dockerfile.testclone`.
    - [x] Create `test/installations/build_rtest.sh` to automate building and running the R installation test.
    - [x] Update `test/installations/README.md` to reflect these changes.






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

- [x] Data Schema
    - [x] Choose compulsory variables
        - run_id, program_name, user_name, script_sha256, exit_code_int, started_at, ended_at, cpu_time_sec, cpu_percent, max_rss_mb


# Notes & Archive
# To Do
# In Progress
# Done
# Notes & Archive
# TUI Dashboard (Session: 2026-04-10)
#######
# Task Plan: R Installation & Documentation (Session: 2026-04-10)
# Task Plan: R Installation & Documentation
## Technical Context
## Technical Enemies:
### v0 Scope
### v1 Scope
### Setup
### Backend
# ARCHIVE
