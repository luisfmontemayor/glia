use crate::network::JobMetrics;
use crate::action::Action;
use ratatui::widgets::TableState;
use std::collections::HashMap;

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
    JobStatus,
}

#[derive(Debug, Clone)]
pub struct JobSummary {
    pub program_name: String,
    pub count: usize,
    pub avg_wall_time_ms: u64,
    pub total_cpu_time_sec: f64,
    pub max_rss_kb: u64,
}

pub struct App {
    pub running: bool,
    pub window: TimeWindow,
    pub metric: Metric,
    pub jobs: Vec<JobMetrics>,
    pub summaries: Vec<JobSummary>,
    pub table_state: TableState,
    pub error_message: Option<String>,
    pub is_loading: bool,
    pub show_detail: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            running: true,
            window: TimeWindow::W1h,
            metric: Metric::WallTime,
            jobs: Vec::new(),
            summaries: Vec::new(),
            table_state: TableState::default(),
            error_message: None,
            is_loading: false,
            show_detail: false,
        };
        app.refresh_summaries();
        app
    }

pub fn refresh_summaries(&mut self) {
    self.summaries = self.summarize_jobs();
}

pub fn update(&mut self, action: Action) {
    match action {
        Action::Quit => self.running = false,
        Action::NextWindow => self.next_window(),
        Action::NextMetric => self.next_metric(),
        Action::PreviousMetric => self.previous_metric(),
        Action::NextRow => self.next_row(),
        Action::PreviousRow => self.previous_row(),
        Action::FetchMetrics => self.is_loading = true,
            Action::ToggleDetail => self.show_detail = !self.show_detail,
            Action::SetJobs(jobs) => {
                self.jobs = jobs;
                self.refresh_summaries();
                if self.table_state.selected().is_none() && !self.jobs.is_empty() {
                    self.table_state.select(Some(0));
                }
                self.error_message = None;
                self.is_loading = false;
            }
            Action::FetchError(err) => {
                self.error_message = Some(err);
                self.is_loading = false;
            }
            _ => {}
        }
    }

    pub fn summarize_jobs(&self) -> Vec<JobSummary> {
        let mut map: HashMap<String, (usize, u64, f64, u64)> = HashMap::new();
        for j in &self.jobs {
            let entry = map.entry(j.program_name.clone()).or_insert((0, 0, 0.0, 0));
            entry.0 += 1;
            entry.1 += j.wall_time_ms as u64;
            entry.2 += j.cpu_time_sec as f64;
            entry.3 = entry.3.max(j.max_rss_kb as u64);
        }

        let mut summaries: Vec<JobSummary> = map
            .into_iter()
            .map(|(name, (count, wall, cpu, rss))| JobSummary {
                program_name: name,
                count,
                avg_wall_time_ms: wall / count as u64,
                total_cpu_time_sec: cpu,
                max_rss_kb: rss,
            })
            .collect();
        summaries.sort_by(|a, b| b.count.cmp(&a.count));
        summaries
    }

    pub fn next_row(&mut self) {
        let count = self.summarize_jobs().len();
        if count == 0 {
            self.table_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= count - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let count = self.summarize_jobs().len();
        if count == 0 {
            self.table_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 { count - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
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
            Metric::MaxRss => Metric::JobStatus,
            Metric::JobStatus => Metric::WallTime,
        };
    }

    pub fn previous_metric(&mut self) {
        self.metric = match self.metric {
            Metric::WallTime => Metric::JobStatus,
            Metric::CpuTime => Metric::WallTime,
            Metric::CpuPercent => Metric::CpuTime,
            Metric::MaxRss => Metric::CpuPercent,
            Metric::JobStatus => Metric::MaxRss,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_quit_action() {
        let mut app = App::new();
        app.update(Action::Quit);
        assert!(!app.running, "App should stop running after Quit action");
    }

    #[test]
    fn test_app_initialization() {
        let app = App::new();
        assert_eq!(app.window, TimeWindow::W1h);
        assert_eq!(app.metric, Metric::WallTime);
        assert!(app.table_state.selected().is_none());
    }

    #[test]
    fn test_job_summary_aggregation() {
        let mut app = App::new();
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "data_proc".to_string(),
                user_name: "alice".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:10:00Z".to_string(),
                program_name: "data_proc".to_string(),
                user_name: "alice".to_string(),
                wall_time_ms: 200,
                cpu_time_sec: 0.2,
                cpu_percent: 15.0,
                max_rss_kb: 2000,
                exit_code_int: 1,
            },
        ];
        
        let summaries = app.summarize_jobs();
        assert_eq!(summaries.len(), 1);
        let s = &summaries[0];
        assert_eq!(s.program_name, "data_proc");
        assert_eq!(s.count, 2);
        assert_eq!(s.avg_wall_time_ms, 150);
        assert!((s.total_cpu_time_sec - 0.3).abs() < 1e-6);
        assert_eq!(s.max_rss_kb, 2000);
    }

    #[test]
    fn test_table_navigation_wrap_around() {
        let mut app = App::new();
        // Add some jobs so we have something to navigate
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "job1".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:10:00Z".to_string(),
                program_name: "job2".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 200,
                cpu_time_sec: 0.2,
                cpu_percent: 15.0,
                max_rss_kb: 2000,
                exit_code_int: 0,
            },
        ];
        let count = app.summarize_jobs().len();
        app.table_state.select(Some(0));

        app.next_row();
        assert_eq!(app.table_state.selected(), Some(1));

        for _ in 0..count {
            app.next_row();
        }
        // After count more nexts, we should be at Some(1) again
        assert_eq!(app.table_state.selected(), Some(1));

        app.table_state.select(Some(0));
        app.previous_row();
        assert_eq!(app.table_state.selected(), Some(count - 1));
    }

    #[test]
    fn test_app_loading_state() {
        let mut app = App::new();
        // This will fail initially because is_loading doesn't exist yet
        // and update doesn't handle Action::FetchMetrics to set it to true.
        assert!(!app.is_loading, "Initial state should not be loading");

        app.update(Action::FetchMetrics);
        assert!(app.is_loading, "Should be loading after FetchMetrics");

        app.update(Action::SetJobs(vec![]));
        assert!(!app.is_loading, "Should not be loading after SetJobs");

        app.update(Action::FetchMetrics);
        assert!(app.is_loading, "Should be loading after FetchMetrics again");

        app.update(Action::FetchError("error".to_string()));
        assert!(!app.is_loading, "Should not be loading after FetchError");
    }

    #[test]
    fn test_toggle_detail() {
        let mut app = App::new();
        assert!(!app.show_detail);
        app.update(Action::ToggleDetail);
        assert!(app.show_detail);
        app.update(Action::ToggleDetail);
        assert!(!app.show_detail);
    }
}
