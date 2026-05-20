# Glia TUI Dashboard

A minimalistic, highly visual terminal user interface for monitoring Glia telemetry data in real-time. Built with Rust and `ratatui`.

## Features
- **Interactive Dashboard**: Multi-pane layout showing aggregated metric graphs alongside detailed job lists.
- **Blame Mode**: Track resource usage per user to identify heavy workloads.
- **Advanced Table Navigation**: Navigate through jobs with dedicated Row, Column, and Cell focus modes.
- **Dynamic UI**: Auto-expanding cells for better visibility and prioritized time windows for granular analysis.
- **Visual-First Design**: Metrics are presented graphically using sparklines or bar charts to provide immediate context.
- **Interactive Metric Selection**: Cycle through key metrics:
  - Wall Time
  - CPU Time
  - CPU Percentage
  - Max RSS (Memory)
  - Job Throughput
  - Success Rate

## Running the Dashboard
The TUI connects to the Glia FastAPI backend to fetch telemetry data. Ensure the backend is running before starting the dashboard.

Using `cargo` directly:
```bash
cd tui
cargo run
```

## Keyboard Controls
| Key | Action |
|-----|--------|
| `q` | Quit |
| `Tab` / `Shift+Tab` | Cycle Metrics |
| `t / T` | Cycle Time Windows (1h, 3h, 6h, 12h, 24h) |
| `b` | Toggle Blame Mode (User-based aggregation) |
| `r` | Row Focus Mode (Navigate through jobs/users) |
| `c` | Column Focus Mode (Navigate through metrics) |
| `Enter` | Select Cell (in Row mode) or Show Details |
| `Esc` | Go back (Cell -> Row, Column -> Row) |
| `s` | Sort Table (based on selected Cell or Column) |
| `/` | Search jobs |
