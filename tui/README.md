# Glia TUI Dashboard

A minimalistic, highly visual terminal user interface for monitoring Glia telemetry data in real-time. Built with Rust and `ratatui`.

## Features
- **Visual-First Design**: All metrics are presented graphically using sparklines or bar charts to provide immediate context without overwhelming the user with raw numbers.
- **Dynamic Time Windows**: Toggle between viewing data from the past 1h, 3h, 6h, 12h, or 24h.
- **Interactive Metric Selection**: Cycle through key metrics one at a time:
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
- `Tab`: Cycle to the next metric.
- `t`: Cycle to the next time window.
- `q` or `Ctrl+C`: Quit the application.