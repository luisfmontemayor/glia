# Make README pitch-ready
- [ ] Readme needs r installation path mapping
- [ ] cli gui 
 

#######

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