#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimeWindow {
    W1h,
    W3h,
    W6h,
    W12h,
    W24h,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Metric {
    WallTime,
    CpuTime,
    CpuPercent,
    MaxRss,
    Throughput,
    SuccessRate,
}

pub struct App {
    pub running: bool,
    pub window: TimeWindow,
    pub metric: Metric,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            window: TimeWindow::W1h,
            metric: Metric::WallTime,
        }
    }

    pub fn next_window(&mut self) {
        self.window = match self.window {
            TimeWindow::W1h => TimeWindow::W3h,
            TimeWindow::W3h => TimeWindow::W6h,
            TimeWindow::W6h => TimeWindow::W12h,
            TimeWindow::W12h => TimeWindow::W24h,
            TimeWindow::W24h => TimeWindow::W1h,
        };
    }

    pub fn next_metric(&mut self) {
        self.metric = match self.metric {
            Metric::WallTime => Metric::CpuTime,
            Metric::CpuTime => Metric::CpuPercent,
            Metric::CpuPercent => Metric::MaxRss,
            Metric::MaxRss => Metric::Throughput,
            Metric::Throughput => Metric::SuccessRate,
            Metric::SuccessRate => Metric::WallTime,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let app = App::new();
        assert_eq!(app.window, TimeWindow::W1h);
        assert_eq!(app.metric, Metric::WallTime);
    }

    #[test]
    fn test_time_window_toggle() {
        let mut app = App {
            running: true,
            window: TimeWindow::W1h,
            metric: Metric::WallTime,
        };
        
        app.next_window();
        assert_eq!(app.window, TimeWindow::W3h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W6h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W12h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W24h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W1h);
    }

    #[test]
    fn test_metric_toggle() {
        let mut app = App {
            running: true,
            window: TimeWindow::W1h,
            metric: Metric::WallTime,
        };

        app.next_metric();
        assert_eq!(app.metric, Metric::CpuTime);
        app.next_metric();
        assert_eq!(app.metric, Metric::CpuPercent);
        app.next_metric();
        assert_eq!(app.metric, Metric::MaxRss);
        app.next_metric();
        assert_eq!(app.metric, Metric::Throughput);
        app.next_metric();
        assert_eq!(app.metric, Metric::SuccessRate);
        app.next_metric();
        assert_eq!(app.metric, Metric::WallTime);
    }
}
