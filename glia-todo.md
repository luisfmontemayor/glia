# Batch Ingestion Implementation
- [x] **TDD: Create reproduction test case for batch ingestion**
    - [x] Define expected behavior for `POST /ingest/batch`
    - [x] Create test in `backend/test/main_test.py`
- [x] **Backend: Data Modeling**
    - [x] Define `JobBatchCreate` if necessary (or use `list[JobCreate]`)
- [x] **Backend: Implementation**
    - [x] Implement `POST /ingest/batch` endpoint in `backend/main.py`
    - [x] Ensure atomic transactions for batch writes
    - [x] Handle potential errors gracefully (partial success vs full rollback)
    - [x] Refactor to **Batch-Only** (removed single item endpoint to prevent logic drift)
- [x] **Clients Synchronization**
    - [x] Update Python client to wrap single metrics in lists
    - [x] Update R client to wrap single metrics in lists
    - [x] Flatten R metadata structure to match Backend/Python schema
- [x] **Validation**
    - [x] Run backend tests
    - [x] Verify database state after batch ingestion
    - [x] Fix and verify E2E tests for both clients
    - [x] Update benchmark scripts for batch compatibility

# Make README pitch-ready
- [x] Identify and document missing system dependencies (`cmake`, `libuv1-dev`, `pandoc`)
- [ ] Readme needs r installation path mapping
- [ ] cli gui 
 

#######

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






## 🛠️ Technical Context
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
