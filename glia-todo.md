# Task Plan: R Installation & Documentation (Session: 2026-04-10)
- [x] **Refine Installation Tests**:
    - [x] Update `test/installations/README.md` with detailed instructions.
    - [x] Improve `test/installations/build_rtest.sh` with logging and error handling.
- [x] **Documentation Sync**:
    - [x] Verify root `README.md` contains R installation path mapping and `mise` integration.
- [x] **Cleanup**:
    - [x] Confirmed `test/installations/Dockerfile.testclone` is already removed.
- [ ] **Conventional Commits Enhancement**:
    - [ ] Add `docs` as a valid scope category for documentation changes.

# Task Plan: R Installation & Documentation
- [x] **Update Root README**:
    - Document the R installation process using `mise run setup:r-deps`.
    - [x] Explain that `renv` is used for dependency management and it generates a `.env.r` file.
    - [x] Clarify that `mise` automatically loads `.env.r` to set `R_LIBS_USER`, ensuring R uses the project-local library.
- [x] **Cleanup & Automate Installation Tests**:
    - [x] Remove redundant `test/installations/Dockerfile.testclone`.
    - [x] Create `test/installations/build_rtest.sh` to automate building and running the R installation test.
    - [x] Update `test/installations/README.md` to reflect these changes.
- [ ] **Mise R Plugin Migration**:
    - **Pitch**: The `asdf-r` plugin for `mise` ensures that the exact R version (e.g., 4.4.1) is used across all development, CI, and production environments. It automates the compilation of R from source, eliminating version mismatches and "works on my machine" errors while providing a seamless, version-pinned toolchain.
    - **Task**: Fully integrate the `asdf-r` plugin into the `mise` configuration and update the installation tests to rely on `mise`-managed R instead of system R.

# Parallel Development Branches

# Make README pitch-ready
- [x] Identify and document missing system dependencies (`cmake`, `libuv1-dev`, `pandoc`)
- [ ] Readme needs r installation path mapping
  - **Plan**: Document how `mise run setup:r-deps` initializes the `renv` environment and generates a `.env.r` file with `R_LIBS_USER` for correct R library path configuration. Instruct users on how to source this file or ensure `mise` handles it for R sessions.
- [ ] cli gui 
 

#######

- [/] downgrade to 3.10 for better interop (Done in `downgrade-python-3.10`)
- [x] health check: is it legit (Endpoints identified: `/health/live`, `/health/ready`)
- [x] lazygit plugin: no files staged means it puts in messed up scope label (Done in `fix-conventional-commits`)
- [x] backend migrations fix (Done in `fix-conventional-commits`)
- [x] TODO: rename to something that reflects that this Enqueues the payload to a background worker.
- [x] Make hardcoded "http://localhost" in tests and benchmarks configurable via environment variables.
- [ ] Readme needs r installation path mapping
- [ ] Test the R installation in a bare container
  - **Plan**: Create a Dockerfile starting from a minimal Ubuntu image. Install `R`, `rustc`, `cargo`, `mise`, and all identified system dependencies (`build-essential`, `libuv1-dev`, `pandoc`, `libgit2-dev`, `libsecret-1-dev`, `libxml2-dev`, `libfribidi-dev`, `libharfbuzz-dev`, `libtiff-dev`, `libwebp-dev`, `libssl-dev`, `libcurl4-openssl-dev`). Copy the project, run `mise run setup:all`, `mise gliar:build`, and execute `gliar`'s R tests. Iterate and debug until successful.

- [ ] encrypt comms, start with https (STASHED)
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
