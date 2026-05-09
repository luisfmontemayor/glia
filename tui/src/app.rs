use crate::network::JobMetrics;
use crate::action::Action;
use crate::table_state::JobsTableState;
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

impl TimeWindow {
    pub fn next(&self) -> Self {
        match self {
            TimeWindow::W1h => TimeWindow::W3h,
            TimeWindow::W3h => TimeWindow::W6h,
            TimeWindow::W6h => TimeWindow::W12h,
            TimeWindow::W12h => TimeWindow::W24h,
            TimeWindow::W24h => TimeWindow::WMax,
            TimeWindow::WMax => TimeWindow::W1h,
        }
    }

    pub fn to_duration(&self) -> Option<std::time::Duration> {
        match self {
            TimeWindow::W1h => Some(std::time::Duration::from_secs(3600)),
            TimeWindow::W3h => Some(std::time::Duration::from_secs(3600 * 3)),
            TimeWindow::W6h => Some(std::time::Duration::from_secs(3600 * 6)),
            TimeWindow::W12h => Some(std::time::Duration::from_secs(3600 * 12)),
            TimeWindow::W24h => Some(std::time::Duration::from_secs(3600 * 24)),
            TimeWindow::WMax => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Metric {
    WallTime,
    CpuTime,
    CpuPercent,
    MaxRss,
    JobStatus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pane {
    Graph,
    Jobs,
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
    pub jobs_table_state: JobsTableState,
    pub error_message: Option<String>,
    pub is_loading: bool,
    pub show_detail: bool,
    pub show_user_lines: bool,
    pub org_name: String,
    pub db_status: bool,
    pub api_status: bool,
    pub focused_pane: Pane,
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
            jobs_table_state: JobsTableState::default(),
            error_message: None,
            is_loading: false,
            show_detail: false,
            show_user_lines: false,
            org_name: std::env::var("GLIA_ORG_NAME").unwrap_or_else(|_| "Unnamed team".to_string()),
            db_status: true,
            api_status: true,
            focused_pane: Pane::Jobs,
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
            Action::ToggleUserLines => self.show_user_lines = !self.show_user_lines,
            Action::SetJobs(jobs) => {
                let all_jobs = jobs;
                if !all_jobs.is_empty() {
                    for _ in 0..6 {
                        let now = std::time::SystemTime::now();
                        let cutoff = self.window.to_duration().and_then(|d| now.checked_sub(d));

                        let matched_indices: Vec<usize> = all_jobs.iter().enumerate().filter_map(|(i, job)| {
                            if let Some(cutoff_time) = cutoff {
                                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&job.started_at) {
                                    let job_time: std::time::SystemTime = dt.into();
                                    if job_time >= cutoff_time { Some(i) } else { None }
                                } else {
                                    Some(i)
                                }
                            } else {
                                Some(i)
                            }
                        }).collect();

                        if !matched_indices.is_empty() || self.window == TimeWindow::WMax {
                            self.jobs = matched_indices.into_iter().map(|i| all_jobs[i].clone()).collect();
                            break;
                        }
                        self.window = self.window.next();
                    }
                } else {
                    self.jobs = Vec::new();
                }

                self.refresh_summaries();
                if self.jobs_table_state.row_state.selected().is_none() && !self.jobs.is_empty() {
                    self.jobs_table_state.row_state.select(Some(0));
                }
                self.error_message = None;
                self.is_loading = false;
                self.api_status = true;
            }
            Action::FetchError(err) => {
                self.error_message = Some(err);
                self.is_loading = false;
                self.api_status = false;
            }
            Action::UpdateHealth(db, api) => {
                self.db_status = db;
                self.api_status = api;
            }
            Action::FocusPane(pane) => {
                self.focused_pane = pane;
            }
            Action::TableFocusRow => {
                self.jobs_table_state.focus_mode = crate::table_state::TableFocusMode::Row;
            }
            Action::TableFocusCell => {
                self.jobs_table_state.focus_mode = crate::table_state::TableFocusMode::Cell;
            }
            Action::TableNextCol => {
                self.jobs_table_state.next_col(5); // 5 columns in total
            }
            Action::TablePrevCol => {
                self.jobs_table_state.prev_col();
            }
            Action::TableSearch(query) => {
                self.jobs_table_state.is_searching = true;
                self.jobs_table_state.search_query = query;
                self.refresh_summaries();
            }
            Action::TableEndSearch => {
                self.jobs_table_state.is_searching = false;
            }
            Action::TableChar(c) => {
                if self.jobs_table_state.is_searching {
                    self.jobs_table_state.search_query.push(c);
                    self.refresh_summaries();
                }
            }
            Action::TableBackspace => {
                if self.jobs_table_state.is_searching {
                    self.jobs_table_state.search_query.pop();
                    self.refresh_summaries();
                }
            }
            Action::TableSort => {
                if self.jobs_table_state.focus_mode == crate::table_state::TableFocusMode::Cell {
                    if let Some(selected) = self.jobs_table_state.selected_col {
                        if self.jobs_table_state.sort_col == Some(selected) {
                            self.jobs_table_state.sort_desc = !self.jobs_table_state.sort_desc;
                        } else {
                            self.jobs_table_state.sort_col = Some(selected);
                            self.jobs_table_state.sort_desc = true;
                        }
                        self.refresh_summaries();
                    }
                }
            }
            _ => {}
        }
    }

    pub fn summarize_jobs(&self) -> Vec<JobSummary> {
        let query = self.jobs_table_state.search_query.to_lowercase();
        let mut map: HashMap<String, (usize, u64, f64, u64)> = HashMap::new();
        for j in &self.jobs {
            if !query.is_empty() && !j.program_name.to_lowercase().contains(&query) {
                continue;
            }
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

        if let Some(col) = self.jobs_table_state.sort_col {
            summaries.sort_by(|a, b| {
                let res = match col {
                    0 => a.program_name.cmp(&b.program_name),
                    1 => a.count.cmp(&b.count),
                    2 => a.avg_wall_time_ms.cmp(&b.avg_wall_time_ms),
                    3 => a.total_cpu_time_sec.partial_cmp(&b.total_cpu_time_sec).unwrap_or(std::cmp::Ordering::Equal),
                    4 => a.max_rss_kb.cmp(&b.max_rss_kb),
                    _ => std::cmp::Ordering::Equal,
                };
                if self.jobs_table_state.sort_desc {
                    res.reverse()
                } else {
                    res
                }
            });
        } else {
            summaries.sort_by(|a, b| {
                b.count
                    .cmp(&a.count)
                    .then_with(|| a.program_name.cmp(&b.program_name))
            });
        }
        summaries
    }

    pub fn next_row(&mut self) {
        let count = self.summarize_jobs().len();
        if count == 0 {
            self.jobs_table_state.row_state.select(None);
            return;
        }
        let i = match self.jobs_table_state.row_state.selected() {
            Some(i) => {
                if i >= count - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.jobs_table_state.row_state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let count = self.summarize_jobs().len();
        if count == 0 {
            self.jobs_table_state.row_state.select(None);
            return;
        }
        let i = match self.jobs_table_state.row_state.selected() {
            Some(i) => {
                if i == 0 { count - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.jobs_table_state.row_state.select(Some(i));
    }

    pub fn next_window(&mut self) {
        self.window = self.window.next();
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
        assert!(app.jobs_table_state.row_state.selected().is_none());
        assert_eq!(app.focused_pane, Pane::Jobs);
        assert!(app.db_status);
        assert!(app.api_status);
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
        app.jobs_table_state.row_state.select(Some(0));

        app.next_row();
        assert_eq!(app.jobs_table_state.row_state.selected(), Some(1));

        for _ in 0..count {
            app.next_row();
        }
        // After count more nexts, we should be at Some(1) again
        assert_eq!(app.jobs_table_state.row_state.selected(), Some(1));

        app.jobs_table_state.row_state.select(Some(0));
        app.previous_row();
        assert_eq!(app.jobs_table_state.row_state.selected(), Some(count - 1));
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

    #[test]
    fn test_job_summary_sorting() {
        let mut app = App::new();
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "zebra".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "alpha".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
        ];

        let summaries = app.summarize_jobs();
        assert_eq!(summaries.len(), 2);
        // Both have count 1. "alpha" should come before "zebra" if sorted ascending by program_name.
        assert_eq!(summaries[0].program_name, "alpha");
        assert_eq!(summaries[1].program_name, "zebra");
    }

    #[test]
    fn test_table_sorting() {
        let mut app = App::new();
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "b".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 200,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "a".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.2,
                cpu_percent: 10.0,
                max_rss_kb: 2000,
                exit_code_int: 0,
            },
        ];

        // Default sort: by count (both 1) then program_name ASC -> "a", "b"
        let s = app.summarize_jobs();
        assert_eq!(s[0].program_name, "a");
        assert_eq!(s[1].program_name, "b");

        // Sort by program_name (col 0) DESC
        app.jobs_table_state.focus_mode = crate::table_state::TableFocusMode::Cell;
        app.jobs_table_state.selected_col = Some(0);
        app.update(Action::TableSort);
        assert_eq!(app.jobs_table_state.sort_col, Some(0));
        assert!(app.jobs_table_state.sort_desc);
        let s = app.summarize_jobs();
        assert_eq!(s[0].program_name, "b");
        assert_eq!(s[1].program_name, "a");

        // Sort by program_name (col 0) ASC
        app.update(Action::TableSort);
        assert!(!app.jobs_table_state.sort_desc);
        let s = app.summarize_jobs();
        assert_eq!(s[0].program_name, "a");
        assert_eq!(s[1].program_name, "b");

        // Sort by avg_wall_time_ms (col 2) DESC
        app.jobs_table_state.selected_col = Some(2);
        app.update(Action::TableSort);
        assert_eq!(app.jobs_table_state.sort_col, Some(2));
        assert!(app.jobs_table_state.sort_desc);
        let s = app.summarize_jobs();
        assert_eq!(s[0].program_name, "b"); // 200ms
        assert_eq!(s[1].program_name, "a"); // 100ms
    }

    #[test]
    fn test_row_to_cell_focus_transition() {
        use crate::table_state::TableFocusMode;
        let mut app = App::new();
        assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Row);
        
        app.update(Action::TableFocusCell);
        assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Cell);
        
        app.update(Action::TableFocusRow);
        assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Row);
    }

    #[test]
    fn test_search_filtering() {
        let mut app = App::new();
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "find_me".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "hide_me".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
        ];
        
        // Initial state: both should be there
        assert_eq!(app.summarize_jobs().len(), 2);
        
        // Apply filter
        app.update(Action::TableSearch("find".to_string()));
        
        // Case-insensitive check
        app.update(Action::TableSearch("FIND".to_string()));
        assert_eq!(app.summarize_jobs().len(), 1);
        let summaries = app.summarize_jobs();
        
        // Case-insensitive check
        app.update(Action::TableSearch("FIND".to_string()));
        assert_eq!(app.summarize_jobs().len(), 1);
        assert_eq!(summaries.len(), 1);
        
        // Case-insensitive check
        app.update(Action::TableSearch("FIND".to_string()));
        assert_eq!(app.summarize_jobs().len(), 1);
        assert_eq!(summaries[0].program_name, "find_me");
        
        // Case-insensitive check
        app.update(Action::TableSearch("FIND".to_string()));
        assert_eq!(app.summarize_jobs().len(), 1);
        
        // Apply filter that matches nothing
        app.update(Action::TableSearch("nothing".to_string()));
        assert_eq!(app.summarize_jobs().len(), 0);
        
        // Clear filter (empty string)
        app.update(Action::TableSearch("".to_string()));
        assert_eq!(app.summarize_jobs().len(), 2);
    }
}
