- [ ] fix: up arrows up and down / j and k for up and down do not work. 
- [ ] too much distance between each column. Up the cropping limit to 21 for job names
- [ ] table Headers should not be cropped
- [ ] Remove the metric units from the values in each cell in the tables. Values only in the headers
- [ ] return original colour to the time window value on app header
- [ ] make each window (header, graph, jobs table, etc) it's own colour according to the app's palette

# Glia TUI Refactor TODO

## HIGH PRIORITY
*All high priority tasks are complete. See ARCHIVE.*

## MEDIUM PRIORITY
*All medium priority tasks are complete. See ARCHIVE.*

## LOW PRIORITY
*All low priority tasks are complete. See ARCHIVE.*

## ARCHIVE

### TUI Refactor Part 2 (Completed)
- [x] Add a search bar for names of scripts in the jobs table 
- [x] The "all time" x axis reads 04-09 04-09 04-09 04-09 01-01 01-01 04-09 04-09 04-09 04-09 01-01 00:16. They make no sense (the postgres data is always the same)
- [x] Make sure that the 1h displays the past hour. If nothing has been logged in the past hour, then it should be empty. Same with all the other time metrics.
- [x] Ensure that, when displaying times that cross day boundaries, that the x axis label space height is doubled and a vertical line of some colour that matches the colour palette. The x axis should display a time of the day for everything but all time which should be in dates and if multiple points per date then time and below it immediatelly should be the date (2 line tall x axis value)
- [x] Ensure that there are docs regarding the colour palette, design philosophy as outlined in tui-design and applicable to the app.
- [x] for the time metrics, add the unit in the header of the table job section, not next to each value. Put them in seconds all of them in the table and display decimals for ms
- [x] add db and api status to the status bar.
- [x] add on ability to focus on each pane. Like LG, add a key binding to each (j for jobs, g for graph) and add arrow and hjkl navigation. rows on table should only be focused if one clicks on it 
- [x] when cell focused, one can use hjkl or arrows to navigate between cells. Fn + arrow or hjkl lets you jump to the beginning end top bottom, make sure that's comlpiant with all keyboards and OSs max win and linux
- [x] when a row is in focus, enter focuses on a cell, escape moves up to the row. 
- [x] truncate values longer than 8 characters, focusing on cell expands it, expanding to the right
- [x] Ensure the table has, like the error pane, an ability to expand dynamically. If values are expanded and it would push values out of view, expand the table value horizontally
- [x] Introduce user line graph for graph, which across the time points, it displays which users are using most of what resource variable.
- [x] Give the focused pane / window a different colour from the rest
- [x] add an env var to name the current organisation to name it in the status bar in the mise.toml. If not, call it "Unnamed team"
- [x] Verify that test coverage is as high as it can be for backend and for front end actions

### TUI Refactor Part 1 (Completed)
- [x] add support for ñ and other non-ansii characters in latin alphabet languages for now 
- [x] **Layout Responsiveness Fix**: Change fixed percentages to fixed lengths (Constraint::Length(3)) for Header, Tabs, and Footer in `ui.rs` to support 80x24 terminals. (@flash-executor)
- [x] **Summaries Caching**: Add `summaries` field to `App` and implement `refresh_summaries()` to avoid recomputing every frame. (@pro-executor for logic, @flash-executor for UI integration)
- [x] **Event Loop Draining**: Refactor main loop in `main.rs` to drain the action channel before drawing. (@pro-executor)
- [x] **Bar Chart Rendering:** Replace basic rectangles with Unicode partial blocks (`▏`, `▎`, `▍`, `▌`, `▋`, `▊`, `▉`, `█`) to achieve sub-cell precision and a high-resolution look.
- [x] **Chart Spacing & Color:** Ensure at least 1 character of "breathing room" between bars. Apply a functional color strategy (e.g., Blue/Orange for primary metrics) ensuring a 3:1 contrast ratio.
- [x] **Chart Axes & Values:** Implement a visible Y-axis that enforces a strict "Zero Baseline". Right-align numeric values on the Y-axis or display them directly on the bars for scannability.
- [x] **X-Axis Time Formatting:** Fix the X-axis logic (currently showing nonsensical dates like "04-09, 01-01"). Format time ranges logically with legible left-aligned labels.
- [x] **Job Table Sorting:** Stabilize top jobs ranking: Sort by `uses` (descending), then by `job name` (alphabetical) to prevent rows from shifting wildly on each poll.
- [x] **Job Details Modal:** Redesign the detail modal (Enter key): make it smaller and centered on the screen. Fetch and display comprehensive DB details for the entry instead of duplicating the jobs table info.
- [x] **Formatting:** Display human-readable metric names in the metrics header, matching the main graph pane titles.
- [x] **Formatting:** Add thousands separators (e.g., `,`) for large numbers in the jobs table.
- [x] **Layout:** Add space padding to window borders/titles for better "Bento-style" aesthetics.
- [x] **Naming:** Rename "Top Script" title to "Jobs" and change the table column "Script" to "Name".
- [x] **Consistency:** Update TUI titles to be consistent with the `spinup` interface.
- [x] **Logging Implementation**: Standardized `[<SCOPE>]: <LEVEL> - <MESSAGE>` logging. (@janitor)
- [x] **Final Performance Audit**: Verify 60fps target. (@analyst)
- [x] Fix DB Loading (Investigation & Diagnostics)
- [x] Error Display Refinement: Improve error popup positioning and visibility. (@flash-executor)
- [x] Metric Detail View: Add a side-pane or popup for detailed script metrics. (@flash-executor)
- [x] **Final Git Hygiene Audit & Cleanup** (@janitor)
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

## QA Audit
- All tests passed successfully (16 passed, 0 failed, 1 ignored).
- Refactor is complete and verified.
