use crate::app::{App, JobSummary};
use crate::table_state::TableFocusMode;
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    Terminal,
};
use crate::ui::render_top_scripts_table;
use crate::theme::SAPPHIRE;

#[test]
fn should_highlight_column_in_column_mode() {
    let mut app = App::new();
    app.summaries = vec![
        JobSummary {
            program_name: "test1".to_string(),
            count: 10,
            avg_wall_time_ms: 100,
            total_cpu_time_sec: 1.0,
            max_rss_kb: 1000,
        },
        JobSummary {
            program_name: "test2".to_string(),
            count: 20,
            avg_wall_time_ms: 200,
            total_cpu_time_sec: 2.0,
            max_rss_kb: 2000,
        },
    ];
    app.jobs_table_state.focus_mode = TableFocusMode::Column;
    app.jobs_table_state.selected_col = Some(1); // "Uses" column

    let backend = TestBackend::new(100, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_top_scripts_table(f, &mut app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    
    // Column 1 is "Uses". 
    // The columns are: Name (Min 25), Uses (Length 8), ...
    // Let's find where "Uses" column starts.
    // In ui.rs:
    // let mut constraints = vec![
    //    Constraint::Min(25),
    //    Constraint::Length(8),
    //    ...
    // ]
    // On a 100 width terminal, Min(25) will probably take more than 25 if other constraints are fixed.
    // Total fixed length = 8 + 14 + 15 + 12 = 49.
    // Remaining = 100 - 49 = 51. So Min(25) will take 51.
    // Column 0: 0..51
    // Column 1: 51..59
    
    // We expect cells in Column 1 to have SAPPHIRE foreground and REVERSED modifier.
    // Rows start after header and margin. Header is 1 row + 1 bottom margin = 2 rows.
    // Block border takes 1 row.
    // So data rows start at y=2 or 3.
    
    let mut highlighted_cells = Vec::new();
    for y in 0..20 {
        for x in 0..100 {
            let cell = buffer.get(x, y);
            if cell.style().fg == Some(SAPPHIRE) && cell.style().add_modifier.contains(Modifier::REVERSED) {
                highlighted_cells.push((x, y, cell.symbol().to_string()));
            }
        }
    }

    if !highlighted_cells.is_empty() {
        println!("Highlighted cells: {:?}", highlighted_cells);
    }

    let highlighted_cells_in_col1 = highlighted_cells.iter().filter(|(x, _, _)| *x >= 51 && *x < 59).count();
    assert!(highlighted_cells_in_col1 > 0, "No cells in column 1 were highlighted in column mode");
    }
