- [x] too much horizonal padding on text and modal, must be twice as vertical padding. Text is still not synamically centered vertically, but it is horizontally
- [x] there are more x axis vertical ticks than there are bars. That cannot be the case. Also, the values of the barplot dissapeared! 
- [x] there are two rows of x axis values. Why is that and how was this allowed 
- [x] text in modal is not centred and does not wrap when the display is shrunk. Add centring, padding and wrapping. The waiting for data should be on a different line.
- [x] when the no data modal is displayed, the graph should not display the message too. The table pane dissapeared altogether. It should be displayed in blank
- [x] add a shift + t for reversing time windows
- [x] I understand that 1h is the default time window. What happens is: 1h doesn't have data, the modal gets displayed, by default it circles to All Time and then the modal disapears. The modal should only be displayed once the time window consolidates on it's final form
- [x] Display modal centred on the screen overlaid on top of other panes. It will have the only "No data available for time window. Waiting for updates..." sign, not in job or graph panes. Centred on the table with a border of a different colour
- [x] the data point vertical lines (need a better name) are not centred with the bars on the barplot. They are layed out evenly with a greater margin on the right than on the left so they don't line up with the bars. They are too tall too. 
- [x] The bar plot doesn't have a x axis line. Add. Perpendicular to the vertical bar chart graph
- [x] The x axis values are outside of the bar plot pane, they are not within the blue line
- [x] The togglable command palette footers are not bordered. Add a grey border same colour of the app footer
- [x] The text wrapping of the command palette words has an issue: it wraps on a per word basis, not on a per item basis. It currently cuts at arrow navigation, and only navigation got wrapped around. It should wrap around the key bound and the name. A line should not end in |
- [x] The command palette footer box should be proportionally as tall as it's text lines. A box with 1 line worth of leyend shouldn't have as much height as one with 2 lines worth of leyends


# CURRENT ORCHESTRATION
- [x] T1: Implement `GLIA_DATA_POINT_THRESHOLD` in `App` struct (@flash-executor)
- [x] T2: Refine Blame Mode Graph (Centering, markers, markers alignment) (@flash-executor)
- [x] T3: Refine Normal Mode BarChart (Centering, markers, padding) (@flash-executor)
- [x] T4: Implement Pinned Headers for table scroll (@flash-executor)
- [x] T5: Refine Context-aware Scroll Indicator (@flash-executor)
- [x] T6: Optimize Table Header aesthetic (Compactness, unit proximity) (@flash-executor)
- [x] T7: Final QA & todo.md Cleanup (@janitor)

# ARCHIVE

### Final UI & Polish (Refactor T1-T7)
- [x] fix: prevent out-of-bounds panic when rendering high-density charts
- [x] T1: Implement GLIA_DATA_POINT_THRESHOLD in App struct
- [x] T2: Refine Blame Mode Graph (Centering, markers, markers alignment)
- [x] T3: Refine Normal Mode BarChart (Centering, markers, padding)
- [x] T4: Implement Pinned Headers for table scroll
- [x] T5: Refine Context-aware Scroll Indicator
- [x] T6: Optimize Table Header aesthetic (Compactness, unit proximity)
- [x] T7: Final QA & todo.md Cleanup
- [x] single timepoint blame graph should show line in the middle of the x axis. Ensure this is the case for 2 or 3: dynamically center the rows, adding equal space between one another and the end of the row. Also, add a vertical perpendicular line indicating the position of the time point
- [x] make ctrl+c a program quit
- [x] when there are too many rows on table to display, scroll should be implemented, by navigating down it should display. There should be a row space with a small downwards caret in a lng highlighted bar to indicate that there are more records below out of view which are not displayed
- [x] add equal margin border within the graph pane, currently uneven
- [x] add the table units to the space below the header cell. Bring cells and headers closer
- [x] fixed: pressing r/c in cell focus mode correctly transitions back to row/column mode
- [x] fixed: command palette now wraps over 2 lines when text is out of view

### Command Palette Refactor
- [x] move blame mode toggle to command palette toggleable at the bottom of of graph pane
- [x] rename top jobs window to jobs window, referring to the pane where the table is
- [x] make context of a commands pane: a subpane at the foot of each pane which displays the command keybindings. It can be toggled with p for command palette. These can be toggled. When toggled, they add to the panes, and the panes are to expand dynamically to include them. Currently a single row of info. The global command palette at the bottom stays anchored always. It displays p for toggling on the subpanes command palettes and q for quit. In the graph, it should shrink the graph horizontally to fit in the pane and in the table pane it should remove visible rows to fit.
- [x] move next metric and previous metrc info to metrics pane name ("Metrics • [Tab]/[Shift+Tab] ")
- [x] move the time window change command info to time window title ("Time Window • [t]")
- [x] moved focus graph [g] and focus jobs [j] keybindings to the titles of their respective panes

### UI & Feature Polish (Completed)
- [x] b still does not trigger blame mode. I currently only have a single user (single task was run 1000 times). Why does it not display
- [x] selecting column means that column including header get wrapped by slim line outline in the same selection colour as the cell selection
- [x] change cell selection fill to selection outline in same saphire colour
- [x] lower 0 value grey tone of barchart to half the whiteness, should be darker but still grey
- [x] no esc legend to say select up. Enter doesn't actually give details, it selects cell. exit unselects cell
- [x] no blame mode command below signalled below with b and it doesn't work (legend/footer update needed)
- [x] Update README with all current features
- [x] fix: cannot display 1h time window when there's no data there. The "prioritise time windows with data" issue means that in the cycle of time windows (1h, 3h, 6h, 12h, 24h, max), the default app state should land on the smallest time window possible for which there is data. It should still display "no available data" if there is no available data, respecting the flow
- [x] this warning happens when I run cargo run: unused_mut in `src/ui.rs:662:9`
- [x] blame mode doesn't work. When b is toggled, it will change the bar plot for a line plot tracking resource usage (y) across the time scale (x) with each line representing a user
- [x] column mode doesn't seem to work
- [x] fix: selecting a cell still doesn't display the entire long cell name. The selection-less display of the cell is actually longer than that of the selected cell. Column width now expands to display the cell contents.
- [x] when pressing enter, it doesn't actually select the cell. I have to click on the arrows or hjkl to be able to get a cell to be actually selected.
- [x] Task 1: Verify Ratatui v0.28 BarChart features usage (e.g., NINE_LEVELS) (@analyst)
- [x] Task 2: Add Legend to Blame Mode Chart (@flash-executor)
- [x] Task 3: Prioritize time windows with data (auto-cycle TimeWindow on fetch) (@flash-executor)
- [x] Task 4: Render 0 value bars distinctly with a "0" text value (@pro-executor)
- [x] Task 5: Implement table sorting by selected column ('s' key) (@flash-executor)
- [x] Task 6: Implement Column selection mode ('c' key) and Row selection mode ('r' key)
    - [x] State logic: Added `TableFocusMode::Column`, handled `Action::TableFocusCol/Row` in `app.rs`, and bound `c`/`r` keys in `main.rs` (@pro-executor)
    - [x] UI logic: Column Highlighting
        - [x] Micro-task 6.1: Define mode flags (is_row_mode, is_col_mode, is_cell_mode) in `render_top_scripts_table`.
        - [x] Micro-task 6.2: Conditionalize `Table::highlight_style` to only apply in Row mode.
        - [x] Micro-task 6.3: Apply `REVERSED` sapphire style to the cell at `selected_col` in the row loop.
- [x] Task 7: Dynamic cell expansion on focus in Cell mode
    - [x] Micro-task 7.1: Retrieve full content of the active cell before constraint calculation.
    - [x] Micro-task 7.2: Skip truncation (`...`) for the active cell in the rendering loop.
    - [x] Micro-task 7.3: Calculate dynamic `Constraint::Length` for the focused column.
    - [x] Micro-task 7.4: Implement horizontal displacement for final column expansion.

### TUI Refactor Part 3 (Completed)
- [x] prioritise time windows where there is data. If the 1h time window has no data, move to the next up until there's data, respecting the cycling nature of the options. 
- [x] when in cell focus mode, if cell is focused, entire text must be displayed, by displacing cells away to show it's value. All is to be displaded to the right unless it's final column, then it must displace columns to the left and the window must expand dynamically like it is the spec for the error and table windows.
- [x] 0 value bars must be displayed on the graph window with a small square with the value displayed inside, narrower than the bar and just a few pixels taller than the value display
- [x] Implement column selection when clicking c. If cell is focused, pressing c selects it's column. If row is selected, pressing c selects first column. Arrows and h and l move column selection left and right
- [x] Implement rowselection when clicking r. If cell is focused, pressing r selects it's row. If column is selected, pressing r selects first row. Up down arrows and j k should move up and down the row selection
- [x] pressing r when on a cell focus selects it's entire row. pressing c on a cell focus focuses the entire column
- [x] pressing s when column is selected sorts table by it, pressing s when cell is selected means column is sorted, pressing s when row is selected is no op
- [x] Add blame mode, where pressing b renders line charts with different colours for each user for each metric, with x and y axis and a legend like that in the barchart section of https://blog.orhun.dev/ratatui-0-22-0/
- [x] confirm that these barchart features are being used: https://ratatui.rs/highlights/v028/ 
- [x] fix: up arrows up and down / j and k for up and down do not work. 
- [x] too much distance between each column. Up the cropping limit to 21 for job names
- [x] table Headers should not be cropped
- [x] Remove the metric units from the values in each cell in the tables. Values only in the headers
- [x] return original colour to the time window value on app header
- [x] make each window (header, graph, jobs table, etc) it's own colour according to the app's palette

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
- All tests passed successfully (17 passed, 0 failed, 1 ignored).
- Refactor is complete and verified.
