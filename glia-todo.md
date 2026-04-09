# Parallel Development Branches

### Branch: `downgrade-python-3.10`
- [x] Downgrade Python version in `.python-version`, `pyproject.toml`, `mise.toml`.
- [x] Fix sync script to handle `3.10` versioning correctly.
- [x] Refactor `datetime.UTC` to `datetime.timezone.utc` for 3.10 compatibility.
- [x] Add `from __future__ import annotations` for `|` type operator support in 3.10.
- [ ] Verify `glia_python` tests pass in a clean 3.10 environment (WSL/Docker issue).

### Branch: `fix-conventional-commits`
- [x] Fix backend migrations scope to strip hashes and end at `backend/migrations/versions`.
- [x] Fix lazygit wizard freeze when no files are staged (removed interactive `gum` prompts).
- [x] Add `Nothing Staged` scope option when no files are staged.
- [x] Add `test_scopes.py` for regression testing of scope logic.

---

# Make README pitch-ready
- [x] Identify and document missing system dependencies (`cmake`, `libuv1-dev`, `pandoc`)
- [ ] Readme needs r installation path mapping
- [ ] cli gui 
 

#######
- [ ] encrypt comms, start with https (STASHED)
- [/] downgrade to 3.10 for better interop (Done in `downgrade-python-3.10`)
- [ ] health check: is it legit (Endpoints identified: `/health/live`, `/health/ready`)
- [x] lazygit plugin: no files staged means it puts in messed up scope label (Done in `fix-conventional-commits`)
- [x] backend migrations fix (Done in `fix-conventional-commits`)
- [x] TODO: rename to something that reflects that this Enqueues the payload to a background worker.
- [x] Make hardcoded "http://localhost" in tests and benchmarks configurable via environment variables.
- [ ] Readme needs r installation path mapping
- [ ] Test the R installation in a bare container

- [ ] cloud, by using the sandbpx software for aws, k8 and ansible
- [ ] cli gui 
- [ ] work on type safety
- [ ] **Future:** Add I/O metrics to models and clients (Postponed).
- [ ] An interactive mode







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
