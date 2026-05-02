# Glia TUI Architecture

The Glia Terminal User Interface (TUI) is a terminal application built in Rust. It uses the `ratatui` library for the interface and `tokio` to run tasks in the background. It connects to the Glia backend API to show job metrics, resource usage, and system status.

## General Structure

The application passes data in one direction. It operates in three steps:
1. **Events**: User keyboard inputs and background network requests create `Action` messages.
2. **State Updates**: The central `App` state receives these `Action` messages and updates its internal data.
3. **UI Rendering**: The `ui::draw` function reads the current `App` state and draws the interface on the screen.

## Core Components

### 1. Main Event Loop (`main.rs`)
This is the entry point. It configures the terminal and starts the asynchronous `tokio` system.
- **Channels**: It uses `tokio::sync::mpsc::unbounded_channel` to pass `Action` messages between different background tasks.
- **Tasks**:
  - **Tick Task**: Creates an `Action::Tick` at regular intervals to maintain timing.
  - **Key Event Task**: Waits for keyboard input using `crossterm::event::poll` and sends an `Action::Key` message.
  - **Fetch Task**: Creates an `Action::FetchMetrics` at regular intervals to request new data from the backend.
  - **Main Loop**: Reads messages from the channel. It applies the necessary logic, updates the `App` state, and redraws the screen.

### 2. Application State (`app.rs`)
The `App` struct contains all the data for the application. It stores the business logic data and the current state of the interface.
- **State Fields**:
  - `jobs`: A list of `JobMetrics` retrieved from the backend.
  - `summaries`: Grouped `JobSummary` data used to display the table.
  - `window` and `metric`: Store the current time window setting (such as 1 Hour or All Time) and the selected resource metric (such as Wall Time or CPU).
  - `jobs_table_state`: A `JobsTableState` struct that stores which row or cell is selected and the current search text.
  - `focused_pane`: Stores whether the Graph or the Jobs Table currently has keyboard focus.
- **`update(action: Action)`**: The function that changes the application state when it receives an action.

### 3. Action System (`action.rs`)
This file defines the `Action` enum. The enum lists all the events that the application recognizes.
- **Examples**: `Quit`, `Tick`, `SetJobs(Vec<JobMetrics>)`, `ToggleDetail`, `NextWindow`, `TableSearch(String)`.
- **Purpose**: It separates the code that creates an event (like a key press) from the code that processes it. This structure makes testing easier.

### 4. UI Rendering (`ui.rs`)
This module reads the `App` state and draws the `ratatui` components. It does not store any data itself.
- **Layout Constraints**: Divides the screen into sections: Header, Tabs, Main Body (Graph and Table), and Footer.
- **`render_metric_chart`**: Draws the charts. Based on the settings in `App`, it displays a standard Bar Chart, a Bar Chart with day boundaries for all-time data, or a Line Graph for specific user metrics.
- **`render_top_scripts_table`**: Draws the data table using the `Table` component and `jobs_table_state`. It handles text truncation, row and cell highlighting, and search text display.

### 5. Networking (`network.rs`)
This module handles network requests.
- **`fetch_metrics`**: An asynchronous function that uses `reqwest` to request data from `localhost:8000/telemetry`. It parses the JSON response into `JobMetrics` structs and sends them to the main loop as an `Action::SetJobs` message.

### 6. Table State (`table_state.rs`)
This module manages the state of the interface specifically for the jobs table.
- **`JobsTableState`**: Stores the `TableState` for row selection, the `selected_col` for column selection, the `TableFocusMode` to track if a row or cell is selected, and the `search_query` string.

## Data Flow Example

1. A background timer runs and sends an `Action::FetchMetrics` message to the channel.
2. The Main Loop (`main.rs`) receives the message and starts the `network::fetch_metrics` network request.
3. The network request finishes and sends an `Action::SetJobs(jobs)` message.
4. The Main Loop passes the action to `App::update`. This function filters the jobs by the `TimeWindow` setting and stores them in `App::jobs`. It then runs `refresh_summaries()` to calculate the table data.
5. The application calls `terminal.draw`, which runs `ui::draw(f, &mut app)`. The UI module reads the updated `App::summaries` and draws the new table and graph on the screen.