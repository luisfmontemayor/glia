- [x] verify metrics tab legend uses exactly `[Tab / Shift+Tab]` (subagent: Command Legend Formatter)
- [x] update time window legend in the header to exactly `[t / T]` (subagent: Command Legend Formatter)
- [x] update test assertion in `tui/tests/header.rs` to check for `[t / T]` or `t / T` (subagent: Command Legend Formatter)
- [x] update controls table row in `tui/README.md` to `t / T` (subagent: Command Legend Formatter)
- [x] update documentation in `tui/docs/UI.md` to `[t / T]` (subagent: Command Legend Formatter)
- [x] run cargo test to verify all tests pass (subagent: Command Legend Formatter)
- [x] commit changes (subagent: Command Legend Formatter)
- [x] revert axis line alignment changes in chart.rs (subagent: Layout Reverter)
- [x] remove test_axis_line_symmetric_overhang from chart_layout.rs (subagent: Layout Reverter)
- [x] run cargo test --package tui to verify (subagent: Layout Reverter)
- [x] commit changes with 'style(tui/chart): revert axis line alignment with barchart area' (subagent: Layout Reverter)
- [x] fix(tui/app): reset selected column to first when transitioning from Row mode (subagent: Table Transition Fixer)
  - [x] add tests for transition to app.rs tests (subagent: Table Transition Fixer)
  - [x] update app.rs to check previous focus mode and force selection of first column when transitioning from Row (subagent: Table Transition Fixer)
- [ ] consider changing job status in blame mode for something more informative
- [ ] support row selection for chart subsetting by specific job
- [ ] Support clicking for pane selection
- [ ] Ensure there are no red flags on tui code
- [ ] test with more data
- [ ] put ticks on blame graph
- [ ] revise commit history to see which commits can be squashed together, which can be shifted around, big nice cleanup of commit history
- [ ] add ticks to the y axis
- [ ] verifyt that tests are not silly, that they're actually sensible
- [ ] Remove "value" heading from chart - uninformative
- [x] if row selected and press c -> first column selected. If r clicked and column selected: first row in view (if scrolled) selected. if enter pressed in any moment: first cell in selected col or selected row selected. only 3 selection modes: row, column or cell for now. default is first row selected
- [ ] tab + shift shortcut not labelled
- [ ] y axis and y axis values don't have the same distance as origin.

# TUI Dashboard (Session: 2026-04-10)
- [x] Fix TUI bugs: Blame mode, Column mode, Cell expansion, Enter key selection, Unused mut warning.
- [x] UI/UX: Lower 0 value grey tone of barchart to half the whiteness.
- [x] UI/UX: Improve TUI legend/footer (ESC to select up, 'b' command, etc.).
- [x] Documentation: Update TUI README with current features.
- [ ] Technical Debt: Rename `core` crate to avoid namespace collision (User task).
- [ ] ensure that benchmarks tests keeps a library with the git commit hash, runtime and setup type (local/distributed/k8s, whatever setup)
- [ ] add env var to add cores to the R installation
- [ ] add most up to date conventional commits and make sure that the descriptions files for it are retained
- [ ] Make README pitch-ready
- [x] Identify and document missing system dependencies (`cmake`, `libuv1-dev`, `pandoc`)
- [ ] Readme needs r installation path mapping
  - **Plan**: Document how `mise run setup:r-deps` initializes the `renv` environment and generates a `.env.r` file with `R_LIBS_USER` for corr>
- [ ] cli gui (TUI Dashboard)
- [ ] migrate from UV to mise if it really does do everything uv does. Weight pros and cons and pick

#######
- [ ] fix: why does backend to db benchmark not show any throughput:
```
## Benchmark:   Backend-to-DB
Latency (ms):           268.09      719.26     2976.05     6436.28     9690.97
Throughput (j/s):         0.00        0.00        0.00        0.00        0.00
```
- [ ] mise tasks: You are currently using sources and outputs for R dependencies, but you aren't using them for the Python uv sync or Rust cargo build task>
- [ ] mise watch for automatic migrations on changes to the schema. Why would I not want to migrate? THink of that too.
- [ ] assess depend in mise tomls and assess if it needs to be changed to a "pre" hook
- [ ] RENV_CONFIG_PAK_ENABLED should always be enabled. always use pak
- [/] downgrade to 3.10 for better interop (Done in `downgrade-python-3.10`)
- [x] health check: is it legit (Endpoints identified: `/health/live`, `/health/ready`)
- [x] lazygit plugin: no files staged means it puts in messed up scope label (Done in `fix-conventional-commits`)
- [x] backend migrations fix (Done in `fix-conventional-commits`)
- [x] TODO: rename to something that reflects that this Enqueues the payload to a background worker.
- [x] Make hardcoded "http://localhost" in tests and benchmarks configurable via environment variables.
- [ ] Readme needs r installation path mapping
- [ ] Test the R installation in a bare container
  - **Plan**: Create a Dockerfile starting from a minimal Ubuntu image. Install `R`, `rustc`, `cargo`, `mise`, and all identified system depend>

- [ ] encrypt comms, start with https?
- [ ] cloud, by using the sandbox software for aws, k8 and ansible
- [x] cli gui (TUI Dashboard)
- [ ] work on type safety
- [ ] **Future:** Add I/O metrics to models and clients (Postponed).
- [ ] An interactive mode






# Task Plan: R Installation & Documentation (Session: 2026-04-10)
- [ ] **Mise R Plugin Integration**:
    - Add the custom `mise-r` plugin URL to `mise.toml` so `mise install` works properly for new users.
    - Update `test/installations/Dockerfile.Rtest` to verify the unbiased installation of R through the plugin.
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

# ARCHIVE
- [x] Create `tui/tui-design.md` detailing the color palette, functional mappings, and layout aesthetics.
- [x] Phase 1: Scaffold TUI project with Ratatui and Tokio.
- [x] Phase 2: Implement route mapping and data models.
- [x] Phase 3: Implement core logic and visuals (Sparklines, metric switching).
- [x] Phase 4: Polish (Panic hooks, formatting).
- [x] Phase 5: Add stacked job status chart and top scripts table.
