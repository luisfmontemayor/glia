# Glia TUI Refactor TODO

## HIGH PRIORITY (Performance & Hardening)
- [ ] **Layout Responsiveness Fix**: Change fixed percentages to fixed lengths (Constraint::Length(3)) for Header, Tabs, and Footer in `ui.rs` to support 80x24 terminals. (@flash-executor)
- [ ] **Summaries Caching**: Add `summaries` field to `App` and implement `refresh_summaries()` to avoid recomputing every frame. (@pro-executor for logic, @flash-executor for UI integration)
- [ ] **Event Loop Draining**: Refactor main loop in `main.rs` to drain the action channel before drawing. (@pro-executor)

## MEDIUM PRIORITY
- [ ] **Error Display Refinement**: Improve error popup positioning and visibility. (@flash-executor)
- [ ] **Metric Detail View**: Add a side-pane or popup for detailed script metrics. (@flash-executor)

## LOW PRIORITY
- [ ] **Logging Implementation**: Standardized `[<SCOPE>]: <LEVEL> - <MESSAGE>` logging. (@janitor)
- [ ] **Final Performance Audit**: Verify 60fps target. (@analyst)

## ARCHIVE
- [x] **Git History Hygiene**: Reword commits `a37ebec`, `22f8483`, and `a1d7416` to follow standard format. (@janitor)
- [x] Asynchronous Event Loop
- [x] Action System
- [x] Stateful Widgets (TableState refactor)
- [x] Async Networking (fetch_metrics)
- [x] Real-time Updates (Background loop)
- [x] Loading, Error, and Empty States (Initial Implementation)
- [x] Interactive Navigation [FRONTEND] (@flash-executor): Add arrow key support for table navigation and scrolling.
- [x] State Tracking [OTHER] (@janitor): Create and maintain `task_tui_refactor.json` (ephemeral).
- [x] **Stateful Widgets** [FRONTEND] (@flash-executor): Refactor `ui.rs` to use `TableState` and `ListState` for the "Top Scripts" table.
- [x] **Async Networking** [BACKEND] (@flash-executor): Implement non-blocking data fetching in `network.rs` using `reqwest`.
- [x] **Real-time Updates** [BACKEND] (@flash-executor): Replace dummy data with periodic polling/fetching from the API.
- [x] **Asynchronous Event Loop** [BACKEND] (@pro-executor): Decouple UI rendering from data processing using `tokio` tasks and `mpsc` channels.
- [x] **Action System** [BACKEND] (@pro-executor): Implement a centralized `Action` enum to handle state transitions.
