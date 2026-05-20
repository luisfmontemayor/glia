# Test Specifications

This document maps the automated tests in the Glia TUI project to the functional requirements and constraints they enforce.

## Core Application Logic (`tests/app.rs`, `tests/test_ticks.rs`)

### State Management
- **Action Processing**: Verifies that every `Action` (e.g., `SetJobs`, `NextTimeWindow`, `ToggleBlame`) results in the expected state transition in the `App` struct.
- **Auto-Scaling Windows**: Ensures the `TimeWindow` automatically expands if new job metrics fall outside the current selected window.

### Chart Components
- **Data Point Ticks**: Ensures short vertical lines (`│`) are rendered between the chart bars and the x-axis, perfectly centered with the data points.
- **Panic Protection**: Explicitly verified to handle high-density data without out-of-bounds buffer access.

## UI Rendering & Layout (`tests/smoke.rs`, `tests/chart_layout.rs`)

### Responsive Design
- **Multi-Resolution Support**: Smoke tests render the UI at 80x24, 144x43, and 40x10 to verify stability.
- **Centering Logic**: Verifies that modals and text paragraphs are correctly centered within their parent containers.

### Data Visualization
- **Bar Chart (Normal Mode)**: Verifies the use of high-resolution Unicode blocks (`█`) and distinct rendering for 0-value data points.
- **Line Chart (Blame Mode)**: Verifies that data points are mapped to Braille markers and distinct colors are assigned per user.
- **Label Alignment**: Ensures x-axis labels (HH:MM) are centered under their respective bars and clipped if they would exceed the available width.

## Interactive Components (`tests/table.rs`, `tests/modal.rs`, `tests/tabs.rs`)

### Table Interaction
- **Focus Modes**: Enforces constraints for `Row`, `Column`, and `Cell` selection modes, including visual highlighting (Sapphire + Reversed styles).
- **Sorting & Search**: Verifies that the table data correctly re-orders when sorting by a specific column.

### Modals
- **Aggregated Detail View**: Ensures the detail modal correctly summarizes job data (Total duration, Average time, Max RSS, and User lists) for a selected program name.
- **Dynamic Content**: Verifies that the modal content wraps correctly and handles "No data" states gracefully without rendering empty titles.

## Component Layouts (`tests/header.rs`, `tests/footer.rs`)
- **Status Indicators**: Verifies the DB and API health indicators correctly reflect the `ServiceStatus` state.
- **Legend & Controls**: Ensures the legend in the footer correctly displays keybindings based on the current context (Metric, Mode, and Focus).
