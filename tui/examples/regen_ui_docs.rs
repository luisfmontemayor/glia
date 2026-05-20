use ratatui::{Terminal, backend::TestBackend};
use tui::app::{App, TimeWindow, Metric};
use tui::network::JobMetrics;
use tui::ui;

fn get_buffer_as_string(terminal: &Terminal<TestBackend>) -> String {
    let buffer = terminal.backend().buffer();
    let mut result = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            result.push_str(buffer.get(x, y).symbol());
        }
        result.push('\n');
    }
    result
}

fn main() {
    let mut app = App::new();
    app.window = TimeWindow::WMax;
    app.jobs = vec![
        JobMetrics {
            started_at: "2023-10-27T10:00:00Z".to_string(),
            program_name: "data_ingest".to_string(),
            user_name: "alice".to_string(),
            wall_time_ms: 1200,
            cpu_time_sec: 1.2,
            cpu_percent: 45.0,
            max_rss_kb: 10240,
            exit_code_int: 0,
        },
        JobMetrics {
            started_at: "2023-10-27T10:15:00Z".to_string(),
            program_name: "model_train".to_string(),
            user_name: "bob".to_string(),
            wall_time_ms: 5000,
            cpu_time_sec: 4.5,
            cpu_percent: 90.0,
            max_rss_kb: 40960,
            exit_code_int: 0,
        },
        JobMetrics {
            started_at: "2023-10-27T10:30:00Z".to_string(),
            program_name: "data_ingest".to_string(),
            user_name: "alice".to_string(),
            wall_time_ms: 1100,
            cpu_time_sec: 1.1,
            cpu_percent: 40.0,
            max_rss_kb: 10000,
            exit_code_int: 1,
        },
    ];
    app.refresh_summaries();

    let width = 120;
    let height = 40;

    // Bar Chart View
    app.blame_mode = false;
    app.metric = Metric::WallTime;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();
    let bar_screenshot = get_buffer_as_string(&terminal);
    println!("---BARCHART_START---");
    println!("{}", bar_screenshot);
    println!("---BARCHART_END---");

    // Line Graph View (Blame Mode)
    app.blame_mode = true;
    app.metric = Metric::CpuPercent;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();
    let line_screenshot = get_buffer_as_string(&terminal);
    println!("---LINEGRAPH_START---");
    println!("{}", line_screenshot);
    println!("---LINEGRAPH_END---");
}
