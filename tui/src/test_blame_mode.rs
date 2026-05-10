#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::network::JobMetrics;
    use crate::ui;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn should_render_line_chart_in_blame_mode() {
        let mut app = App::new();
        app.blame_mode = true;
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "job1".to_string(),
                user_name: "alice".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:05:00Z".to_string(),
                program_name: "job2".to_string(),
                user_name: "bob".to_string(),
                wall_time_ms: 200,
                cpu_time_sec: 0.2,
                cpu_percent: 15.0,
                max_rss_kb: 2000,
                exit_code_int: 0,
            },
        ];

        let backend = TestBackend::new(100, 50);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            ui::draw(f, &mut app);
        }).unwrap();

        let buffer = terminal.backend().buffer();
        
        let mut has_braille = false;
        let mut has_bar = false;

        for cell in buffer.content() {
            let symbol = cell.symbol();
            if symbol.chars().any(|c| (0x2800..=0x28FF).contains(&(c as u32))) {
                has_braille = true;
            }
            if symbol.contains('█') {
                has_bar = true;
            }
        }

        assert!(has_braille, "Buffer should contain braille characters (line chart)");
        assert!(!has_bar, "Buffer should not contain bar characters (█)");
    }
}
