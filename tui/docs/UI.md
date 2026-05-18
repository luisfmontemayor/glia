# Glia TUI Dashboard

## Overview
The Glia TUI Dashboard is a terminal-based interface designed to provide real-time monitoring and historical analysis of job metrics within the Glia ecosystem. Its primary aim is to give developers and operators a high-level view of system performance, job success rates, and resource utilization across different time windows.

## Current Features
- **Real-time Health Status**: Displays the connectivity status of the Database and API.
- **Metric Selection**: Toggle between various metrics:
    - **Wall Time**: Total elapsed time for jobs.
    - **CPU Time**: Total CPU time consumed.
    - **CPU Percent**: Percentage of CPU utilized.
    - **Max RSS**: Maximum memory resident set size.
    - **Job Status**: Success/Failure tracking.
- **Time Windows**: Filter data by 1h, 3h, 6h, 12h, 24h, or All Time.
- **Job Summaries**: A table showing aggregated metrics by program name.
- **Dual Visualization Modes**:
    - **Bar Chart**: Ideal for comparing individual job metrics chronologically.
    - **Line Graph (Blame Mode)**: Best for tracking metrics per user over time.

## Dashboard Components

### 1. Header & Status
The top section displays the team name, current time window, and the health status of the backend services (DB and API).

### 2. Metric Tabs
Allows quick switching between different performance metrics using `[Tab]` and `[Shift+Tab]`.

### 3. Main Body (Split View)
- **Left Pane (Visualization)**: Renders either a BarChart or a Line Graph depending on the mode.
- **Right Pane (Jobs Table)**: Lists aggregated statistics for each program, including usage count, average wall time, total CPU time, and peak memory usage.

### 4. Footnote / Command Palette
The bottom bar provides quick access to common actions and shows the keybindings for navigation and mode toggling.

## Visual Inspection

### Bar Chart View
This view provides a clear chronological breakdown of job metrics. In this mode, individual job results are displayed as bars, making it easy to identify outliers and chronological trends.

```text
┌ Glia TUI ────────────────────────────────────────────────────────────────────────────────────────┐┌ DB ────┐┌ API ───┐
│Glia Base Team  | Time Window [t]: All Time                                                       ││ ACTIVE ││ ACTIVE │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘└────────┘└────────┘
┌ Metrics • [Tab]/[Shift+Tab] ─────────────────────────────────────────────────────────────────────────────────────────┐
│ Wall Time │ CPU Time │ CPU Percent │ Max RSS │ Job Status                                                            │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
┌ Wall Time (ms) • [g] ────────────────────────────────────┐┌ Jobs • [j] ──────────────────────────────────────────────┐
│                    █████████████████                     ││Name                      Uses    Avg Wall Total C Max RSS│
│                    █████████████████                     ││data_ingest               2       1.15     2.30    10,240 │
│                    █████████████████                     ││model_train               1       5.00     4.50    40,960 │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    ██────────────────█                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│                    █████████████████                     ││                                                          │
│ ▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅  █████████████████  ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁  ││                                                          │
│ █████████████████  █████████████████  █████████████████  ││                                                          │
│ █████████████████  █████████████████  █████████████████  ││                                                          │
│ █████████████████  █████████████████  █████████████████  ││                                                          │
│ █████████████████  █████████████████  █████████████████  ││                                                          │
│ █████████████████  █████████████████  █████████████████  ││                                                          │
│ ██████1200███████  ██████5000███████  ██████1100███████  ││                                                          │
│ 10-27 10:00        10:15              10:30              ││                                                          │
└──────────────────────────────────────────────────────────┘└──────────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│[p] Command Palette | [q] Quit                                                                                        │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### Line Graph (Blame Mode)
Blame mode overlays metrics for different users on a single chart. This is particularly useful for identifying which users or processes are consuming the most resources or experiencing the most failures.

```text
┌ Glia TUI ────────────────────────────────────────────────────────────────────────────────────────┐┌ DB ────┐┌ API ───┐
│Glia Base Team  | Time Window [t]: All Time                                                       ││ ACTIVE ││ ACTIVE │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘└────────┘└────────┘
┌ Metrics • [Tab]/[Shift+Tab] ─────────────────────────────────────────────────────────────────────────────────────────┐
│ Wall Time │ CPU Time │ CPU Percent │ Max RSS │ Job Status                                                            │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
┌ CPU Percent (%) • [g] ───────────────────────────────────┐┌ Jobs • [j] ──────────────────────────────────────────────┐
│                                                          ││Name                      Uses    Avg Wall Total C Max RSS│
│ 90│Value                                         ┌─────┐ ││data_ingest               2       1.15     2.30    10,240 │
│   │                                              │alice│ ││model_train               1       5.00     4.50    40,960 │
│   │                          ⠂                   │bob  │ ││                                                          │
│   │                                              └─────┘ ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │             ⠤⠤⠤⣀⣀⣀⣀⣀                                 ││                                                          │
│   │                     ⠉⠉⠉⠉⠉⠑⠒⠒⠒⠒⠢⠤⠤⠤⠤⢄⣀⣀⡀              ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │                                                      ││                                                          │
│   │             ⡄            ⡄            ⡄              ││                                                          │
│ 0 │             ⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡇         Time ││                                                          │
│   └───────────────────────────────────────────────────── ││                                                          │
│                 10:00     10:15     10:30                ││                                                          │
│                                                          ││                                                          │
└──────────────────────────────────────────────────────────┘└──────────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│[p] Command Palette | [q] Quit                                                                                        │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```
