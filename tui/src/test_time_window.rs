#[cfg(test)]
mod tests {
    use crate::app::{App, TimeWindow};
    use crate::ui;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn should_display_no_data_message() {
        let mut app = App::new();
        app.jobs = vec![];
        app.window = TimeWindow::W1h;
        app.is_loading = false;

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            ui::draw(f, &mut app);
        }).unwrap();

        let buffer = terminal.backend().buffer();
        let expected_message = "No data available for this time window";
        
        let mut found = false;
        for y in 0..buffer.area.height {
            let mut line = String::new();
            for x in 0..buffer.area.width {
                line.push_str(buffer.get(x, y).symbol());
            }
            if line.contains(expected_message) {
                found = true;
                break;
            }
        }

        assert!(found, "Message '{}' not found in the output", expected_message);
    }
}
