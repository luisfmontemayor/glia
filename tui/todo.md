# TODO
- [ ] consider changing job status in blame mode for something more informative
- [ ] support row selection for chart subsetting by specific job
- [ ] Support clicking for pane selection
- [ ] Ensure there are no red flags on tui code
- [ ] put ticks on blame graph
- [ ] add ticks to the y axis
- [ ] Remove "value" heading from chart - uninformative
- [ ] tab + shift shortcut not labelled
- [ ] y axis and y axis values don't have the same distance as origin.

- [ ] cli gui (TUI Dashboard)
- [ ] Column functions: multi-col sort
- [ ] Focus on job: change graph data only to be specific to a single job, controlled with row selection

# Done
- [x] verify metrics tab legend uses exactly `[Tab / Shift+Tab]` (subagent: Command Legend Formatter)
- [x] update time window legend in the header to exactly `[t / T]` (subagent: Command Legend Formatter)
- [x] update test assertion in `tui/tests/header.rs` to check for `[t / T]` or `t / T` (subagent: Command Legend Formatter)
- [x] update controls table row in `tui/README.md` to `t / T` (subagent: Command Legend Formatter)
- [x] update documentation in `tui/docs/UI.md` to `[t / T]` (subagent: Command Legend Formatter)
- [x] run cargo test to verify all tests pass (subagent: Command Legend Formatter)
- [x] commit changes (subagent: Command Legend Formatter)
- [x] revert axis line alignment changes in chart.rs (subagent: Layout Reverter)
- [x] remove test_axis_line_symmetric_overhang from chart_layout.rs (subagent: Layout Reverter)
- [x] run cargo test --package tui to verify (subagent: Layout Reverter)
- [x] commit changes with 'style(tui/chart): revert axis line alignment with barchart area' (subagent: Layout Reverter)
- [x] fix(tui/app): reset selected column to first when transitioning from Row mode (subagent: Table Transition Fixer)
  - [x] add tests for transition to app.rs tests (subagent: Table Transition Fixer)
  - [x] update app.rs to check previous focus mode and force selection of first column when transitioning from Row (subagent: Table Transition Fixer)
- [x] if row selected and press c -> first column selected. If r clicked and column selected: first row in view (if scrolled) selected. if enter pressed in any moment: first cell in selected col or selected row selected. only 3 selection modes: row, column or cell for now. default is first row selected
- [x] Fix TUI bugs: Blame mode, Column mode, Cell expansion, Enter key selection, Unused mut warning.
- [x] UI/UX: Lower 0 value grey tone of barchart to half the whiteness.
- [x] UI/UX: Improve TUI legend/footer (ESC to select up, 'b' command, etc.).
- [x] Documentation: Update TUI README with current features.
- [x] cli gui (TUI Dashboard)
- [x] Create `tui/tui-design.md` detailing the color palette, functional mappings, and layout aesthetics.
- [x] Phase 1: Scaffold TUI project with Ratatui and Tokio.
- [x] Phase 2: Implement route mapping and data models.
- [x] Phase 3: Implement core logic and visuals (Sparklines, metric switching).
- [x] Phase 4: Polish (Panic hooks, formatting).
- [x] Phase 5: Add stacked job status chart and top scripts table.


