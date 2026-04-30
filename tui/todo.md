# Glia TUI Refactor TODO

## [HIGH PRIORITY] - Foundation & Core Infrastructure
- [ ] **TDD Integration** [OTHER] (@monitor): Ensure all new features are backed by failing tests before implementation.

## [MEDIUM PRIORITY] - Data Integration & Interaction
- [ ] **Implement JobSummary for TableState** [BACKEND] (@flash-executor): This is needed before the TableState refactor.
- [ ] **Implement Error Display in UI** [FRONTEND] (@flash-executor): Show robust feedback for network and IO errors.
- [ ] **Error Handling** [BACKEND] (@pro-executor): Implement a robust error handling strategy for network and IO errors.

## [LOW PRIORITY] - Polish & Quality Assurance
- [ ] **Dynamic Layout** [FRONTEND] (@analyst + @flash-executor): Improve layout responsiveness for smaller terminal windows.
- [ ] **Loading/Empty States** [FRONTEND] (@flash-executor): Add visual feedback for data fetching and empty results.
- [ ] **Logging** [OTHER] (@janitor): Implement a standardized logging format `[<SCOPE>]: <LEVEL> - <MESSAGE>`.
- [ ] **Performance Audit** [OTHER] (@analyst): Ensure the UI remains responsive (60fps target).
- [ ] **Metric Detail View** [FRONTEND] (@flash-executor): (Optional) Add a popup/detail pane for selected scripts.
- [ ] **Clean Up** [OTHER] (@janitor): Remove unused code and ensure theme consistency.

## ARCHIVE
- [x] **Stateful Widgets** [FRONTEND] (@flash-executor): Refactor `ui.rs` to use `TableState` and `ListState` for the "Top Scripts" table.
- [x] **Async Networking** [BACKEND] (@flash-executor): Implement non-blocking data fetching in `network.rs` using `reqwest`.
- [x] **Real-time Updates** [BACKEND] (@flash-executor): Replace dummy data with periodic polling/fetching from the API.
- [x] **Interactive Navigation** [FRONTEND] (@flash-executor): Add arrow key support for table navigation and scrolling.
- [x] **State Tracking** [OTHER] (@janitor): Create and maintain `task_tui_refactor.json` (ephemeral).
- [x] **Asynchronous Event Loop** [BACKEND] (@pro-executor): Decouple UI rendering from data processing using `tokio` tasks and `mpsc` channels.
- [x] **Action System** [BACKEND] (@pro-executor): Implement a centralized `Action` enum to handle state transitions.
