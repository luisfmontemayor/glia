use crate::network::JobMetrics;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimeWindow {
    W1h,
    W3h,
    W6h,
    W12h,
    W24h,
    WMax,
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
    pub jobs: Vec<JobMetrics>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            running: true,
            window: TimeWindow::W1h,
            metric: Metric::WallTime,
            jobs: Vec::new(),
        };

        // Dummy data for visual testing
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1024,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:05:00Z".to_string(),
                wall_time_ms: 200,
                cpu_time_sec: 0.2,
                cpu_percent: 20.0,
                max_rss_kb: 2048,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:10:00Z".to_string(),
                wall_time_ms: 150,
                cpu_time_sec: 0.15,
                cpu_percent: 15.0,
                max_rss_kb: 1536,
                exit_code_int: 1,
            },
            JobMetrics {
                started_at: "2023-10-27T10:15:00Z".to_string(),
                wall_time_ms: 300,
                cpu_time_sec: 0.3,
                cpu_percent: 30.0,
                max_rss_kb: 3072,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:20:00Z".to_string(),
                wall_time_ms: 250,
                cpu_time_sec: 0.25,
                cpu_percent: 25.0,
                max_rss_kb: 2560,
                exit_code_int: 0,
            },
        ];

        app
    }

    pub fn next_window(&mut self) {
        self.window = match self.window {
            TimeWindow::W1h => TimeWindow::W3h,
            TimeWindow::W3h => TimeWindow::W6h,
            TimeWindow::W6h => TimeWindow::W12h,
            TimeWindow::W12h => TimeWindow::W24h,
            TimeWindow::W24h => TimeWindow::WMax,
            TimeWindow::WMax => TimeWindow::W1h,
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

    pub fn previous_metric(&mut self) {
        self.metric = match self.metric {
            Metric::WallTime => Metric::SuccessRate,
            Metric::CpuTime => Metric::WallTime,
            Metric::CpuPercent => Metric::CpuTime,
            Metric::MaxRss => Metric::CpuPercent,
            Metric::Throughput => Metric::MaxRss,
            Metric::SuccessRate => Metric::Throughput,
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
        let mut app = App::new();
        app.window = TimeWindow::W1h;

        app.next_window();
        assert_eq!(app.window, TimeWindow::W3h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W6h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W12h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W24h);
        app.next_window();
        assert_eq!(app.window, TimeWindow::WMax);
        app.next_window();
        assert_eq!(app.window, TimeWindow::W1h);
    }

    #[test]
    fn test_metric_toggle() {
        let mut app = App::new();
        app.metric = Metric::WallTime;

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

    #[test]
    fn test_previous_metric() {
        let mut app = App::new();
        app.metric = Metric::WallTime;

        app.previous_metric();
        assert_eq!(app.metric, Metric::SuccessRate);
        app.previous_metric();
        assert_eq!(app.metric, Metric::Throughput);
        app.previous_metric();
        assert_eq!(app.metric, Metric::MaxRss);
        app.previous_metric();
        assert_eq!(app.metric, Metric::CpuPercent);
        app.previous_metric();
        assert_eq!(app.metric, Metric::CpuTime);
        app.previous_metric();
        assert_eq!(app.metric, Metric::WallTime);
    }
}
