# Glia TUI Design Specifications

This document outlines the visual language, color palette, and layout philosophy of the Glia TUI dashboard.

## Color Palette: Catppuccin Macchiato

Glia utilizes the **Catppuccin Macchiato** color scheme to provide a high-contrast, yet easy-on-the-eyes experience for long-term monitoring.

| Name | Hex | Usage |
| :--- | :--- | :--- |
| **Rosewater** | `#f4dbd6` | Accents |
| **Flamingo** | `#f0c1c0` | Accents |
| **Pink** | `#f5bdd6` | **Active Elements**: Tab highlighting, focused metric. |
| **Mauve** | `#c6a0f6` | Accents |
| **Red** | `#ed8796` | **Alerts/Critical**: Errors, failures, Max RSS metric. |
| **Maroon** | `#ee99a0` | Accents |
| **Peach** | `#f5a97f` | **Time Metrics**: CPU Time metrics. |
| **Yellow** | `#eed49f` | **Status/Wait**: Time Window selection, Loading indicators, Warnings. |
| **Green** | `#a6da95` | **Success/Health**: Job success, CPU Percent metric. |
| **Teal** | `#8bd5ca` | Accents |
| **Sky** | `#91d7e3` | Accents |
| **Sapphire** | `#7dc4e4` | **Primary Brand**: Header title ("Glia"), Selection highlighting, Popup borders. |
| **Blue** | `#8aadf4` | **Performance Metrics**: Wall Time metrics. |
| **Lavender** | `#b7bdf8` | **Metadata**: Table headers. |
| **Text** | `#cad3f5` | Standard labels and block borders. |
| **Subtext0** | `#a5adcb` | Inactive elements (e.g., non-selected tabs). |
| **Overlay2** | `#939ab7` | Secondary information (Footer keybinds). |
| **Crust** | `#181926` | Background contrast for high-density charts. |

## Functional Mapping

The colors in Glia are not merely aesthetic; they provide immediate semantic feedback:

*   **Red (`RED`)**: Indicates failures in the Job Status chart, errors in the status bar, and critical resource consumption (Max RSS).
*   **Green (`GREEN`)**: Indicates successful job completion and healthy CPU utilization percentages.
*   **Blue/Peach (`BLUE`/`PEACH`)**: Distinguishes between different time-based metrics (Wall Time vs. CPU Time) to prevent cognitive overload when switching tabs.
*   **Yellow (`YELLOW`)**: Used for the "Loading..." state and for non-critical status changes like shifting the Time Window.
*   **Pink (`PINK`)**: Exclusively reserved for the active focus state of the TUI navigation (active tabs).
*   **Sapphire (`SAPPHIRE`)**: Used for brand identity and to denote the "Active Pane" or "Selected Item" in tables.

## Layout Aesthetics

### "Bento-style" Philosophy
The dashboard follows a **Bento-style** layout, inspired by modern UI design where information is compartmentalized into discrete, well-defined blocks.
*   **Structured Chunks**: The screen is divided into Header, Metrics Selection, Visualization, Data Table, and Footer.
*   **Clear Boundaries**: Every functional area is enclosed in a bordered block, ensuring that high-density data remains readable and distinct.

### Visualization & Rendering
*   **Zero-Baseline Graphs**: All bar charts are rendered with a strict zero-baseline to ensure accurate relative comparison of job performance.
*   **Sub-cell Rendering**: Utilizing `ratatui`'s `symbols::bar::NINE_LEVELS`, Glia achieves high-resolution vertical bars that provide smoother transitions and more precise data representation than standard character blocks.
*   **Dynamic Spacing**: Charts use a 1-cell gap between bars and a 2-cell gap between groups (e.g., in the Job Status view) to maintain visual breathing room.
*   **Responsive Popups**: Detail views use a centered-rect calculation to overlay information without losing the context of the background dashboard.
