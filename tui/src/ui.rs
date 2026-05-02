use crate::app::{App, Metric, Pane, TimeWindow};
use crate::theme::*;
use std::collections::HashMap;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, Borders, Cell, Chart, Clear, Dataset, GraphType,
        Paragraph, Row, Table, Tabs,
    },
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0), // Main Body
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    render_header(f, app, chunks[0]);
    render_tabs(f, app, chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    render_metric_chart(f, app, main_chunks[0]);
    render_top_scripts_table(f, app, main_chunks[1]);

    render_footer(f, app, chunks[3]);

    if app.show_detail {
        render_detail_popup(f, app);
    }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .split(area);

    let window_str = match app.window {
        TimeWindow::W1h => "1 Hour",
        TimeWindow::W3h => "3 Hours",
        TimeWindow::W6h => "6 Hours",
        TimeWindow::W12h => "12 Hours",
        TimeWindow::W24h => "24 Hours",
        TimeWindow::WMax => "All Time",
    };

    let main_text = vec![Line::from(vec![
        Span::styled(
            format!(" {} ", app.org_name),
            Style::default().add_modifier(Modifier::BOLD).fg(SAPPHIRE),
        ),
        Span::raw(" | Window: "),
        Span::styled(window_str, Style::default().fg(YELLOW)),
    ])];

    let main_paragraph = Paragraph::new(main_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Status ")
            .style(Style::default().fg(TEXT)),
    );
    f.render_widget(main_paragraph, header_chunks[0]);

    let (db_text, db_color) = if app.db_status {
        ("ACTIVE", GREEN)
    } else {
        ("INACTIVE", RED)
    };
    let db_paragraph = Paragraph::new(db_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(db_color).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" DB ")
                .style(Style::default().fg(TEXT)),
        );
    f.render_widget(db_paragraph, header_chunks[1]);

    let (api_text, api_color) = if app.api_status {
        ("ACTIVE", GREEN)
    } else {
        ("INACTIVE", RED)
    };
    let api_paragraph = Paragraph::new(api_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(api_color).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" API ")
                .style(Style::default().fg(TEXT)),
        );
    f.render_widget(api_paragraph, header_chunks[2]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Wall Time", "CPU Time", "CPU Percent", "Max RSS", "Job Status"];

    let active_index = match app.metric {
        Metric::WallTime => 0,
        Metric::CpuTime => 1,
        Metric::CpuPercent => 2,
        Metric::MaxRss => 3,
        Metric::JobStatus => 4,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Metrics ")
                .style(Style::default().fg(TEXT)),
        )
        .select(active_index)
        .style(Style::default().fg(SUBTEXT0))
        .highlight_style(Style::default().fg(PINK).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}

fn render_metric_chart(f: &mut Frame, app: &App, area: Rect) {
    let border_color = if app.focused_pane == Pane::Graph { PINK } else { TEXT };

    let chart_title = match app.metric {
        Metric::WallTime => " Wall Time (ms) ",
        Metric::CpuTime => " CPU Time (ms) ",
        Metric::CpuPercent => " CPU Percent (%) ",
        Metric::MaxRss => " Max RSS (KB) ",
        Metric::JobStatus => " Job Status (Success: Green | Fail: Red) ",
    };

    if app.is_loading && app.jobs.is_empty() {
        let loading = Paragraph::new("Loading...")
            .style(Style::default().fg(YELLOW))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(chart_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            );
        f.render_widget(loading, area);
        return;
    }

    let parse_time = |started_at: &str| -> (String, String, String) {
        let parts: Vec<&str> = started_at.split('T').collect();
        if parts.len() < 2 {
            return ("??:??".to_string(), "????".to_string(), started_at.to_string());
        }
        let date = parts[0];
        let time = parts[1];
        let date_parts: Vec<&str> = date.split('-').collect();
        let mm_dd = if date_parts.len() >= 3 {
            format!("{}-{}", date_parts[1], date_parts[2])
        } else {
            date.to_string()
        };
        let time_parts: Vec<&str> = time.split(':').collect();
        let hh_mm = if time_parts.len() >= 2 {
            format!("{}:{}", time_parts[0], time_parts[1])
        } else {
            time.chars().take(5).collect()
        };
        (hh_mm, mm_dd, date.to_string())
    };

    if app.show_user_lines {
        let mut user_data: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = 0.0f64;

        for j in &app.jobs {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&j.started_at) {
                let x = dt.timestamp() as f64;
                let val = match app.metric {
                    Metric::WallTime => j.wall_time_ms as f64,
                    Metric::CpuTime => (j.cpu_time_sec * 1000.0) as f64,
                    Metric::CpuPercent => j.cpu_percent as f64,
                    Metric::MaxRss => j.max_rss_kb as f64,
                    Metric::JobStatus => {
                        if j.exit_code_int == 0 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                };
                user_data.entry(j.user_name.clone()).or_default().push((x, val));
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                max_y = max_y.max(val);
            }
        }

        if min_x == f64::MAX {
            min_x = 0.0;
            max_x = 1.0;
        }
        if max_x <= min_x {
            max_x = min_x + 1.0;
        }
        if max_y <= 0.0 {
            max_y = 1.0;
        }

        let mut user_names: Vec<_> = user_data.keys().cloned().collect();
        user_names.sort();

        let mut sorted_data = Vec::new();
        for name in &user_names {
            let mut d = user_data.get(name).unwrap().clone();
            d.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            sorted_data.push(d);
        }

        let colors = [PINK, GREEN, BLUE];
        let datasets: Vec<_> = sorted_data
            .iter()
            .enumerate()
            .map(|(i, data)| {
                Dataset::default()
                    .name(user_names[i].as_str())
                    .marker(symbols::Marker::Dot)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(colors[i % colors.len()]))
                    .data(data)
            })
            .collect();

        let x_labels = vec![
            Span::raw(
                chrono::DateTime::from_timestamp(min_x as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M")
                    .to_string(),
            ),
            Span::raw(
                chrono::DateTime::from_timestamp(max_x as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M")
                    .to_string(),
            ),
        ];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(chart_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(TEXT))
                    .bounds([min_x, max_x])
                    .labels(x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Value")
                    .style(Style::default().fg(TEXT))
                    .bounds([0.0, max_y * 1.1])
                    .labels(vec![Span::raw("0"), Span::raw(format!("{:.0}", max_y))]),
            );

        f.render_widget(chart, area);
    } else {
        let is_wmax = app.window == TimeWindow::WMax;
        let (main_area, label_area) = if is_wmax && !app.jobs.is_empty() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(area);
            (chunks[0], Some(chunks[1]))
        } else {
            (area, None)
        };

        let mut barchart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(chart_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            )
            .bar_set(symbols::bar::NINE_LEVELS)
            .value_style(Style::default().fg(CRUST).bg(TEXT));

        let mut labels_hhmm = Vec::new();
        let mut labels_mmdd = Vec::new();
        let mut last_date = String::new();

        if app.metric == Metric::JobStatus {
            barchart = barchart.bar_width(2).bar_gap(1).group_gap(2);
            for j in &app.jobs {
                let success_val = if j.exit_code_int == 0 { 1 } else { 0 };
                let fail_val = if j.exit_code_int != 0 { 1 } else { 0 };

                let (hhmm, mmdd, date) = parse_time(&j.started_at);
                let label = if is_wmax {
                    "".to_string()
                } else if app.window == TimeWindow::WMax {
                    format!("{} {}", mmdd, hhmm)
                } else {
                    hhmm.clone()
                };

                if is_wmax {
                    let is_new_day = !last_date.is_empty() && date != last_date;
                    let hhmm_style = if is_new_day {
                        Style::default().fg(PINK)
                    } else {
                        Style::default().fg(TEXT)
                    };
                    let mut text = hhmm;
                    if is_new_day {
                        text = format!("|{}", text);
                    }
                    labels_hhmm.push(Span::styled(format!("{:^7}", text), hhmm_style));
                    if is_new_day || labels_mmdd.is_empty() {
                        labels_mmdd.push(Span::styled(
                            format!("{:^7}", mmdd),
                            Style::default().fg(YELLOW),
                        ));
                    } else {
                        labels_mmdd.push(Span::raw("       "));
                    }
                    last_date = date;
                }

                let group = BarGroup::default().label(Line::from(label)).bars(&[
                    Bar::default()
                        .value(success_val)
                        .style(Style::default().fg(GREEN)),
                    Bar::default()
                        .value(fail_val)
                        .style(Style::default().fg(RED)),
                ]);
                barchart = barchart.data(group);
            }
        } else {
            let bar_color = match app.metric {
                Metric::WallTime => BLUE,
                Metric::CpuTime => PEACH,
                Metric::CpuPercent => GREEN,
                Metric::MaxRss => RED,
                _ => BLUE,
            };
            barchart = barchart
                .bar_width(5)
                .bar_gap(1)
                .bar_style(Style::default().fg(bar_color));
            for j in &app.jobs {
                let val = match app.metric {
                    Metric::WallTime => j.wall_time_ms as u64,
                    Metric::CpuTime => (j.cpu_time_sec * 1000.0) as u64,
                    Metric::CpuPercent => j.cpu_percent as u64,
                    Metric::MaxRss => j.max_rss_kb as u64,
                    _ => 0,
                };
                let (hhmm, mmdd, date) = parse_time(&j.started_at);
                let label = if is_wmax {
                    "".to_string()
                } else if app.window == TimeWindow::WMax {
                    format!("{} {}", mmdd, hhmm)
                } else {
                    hhmm.clone()
                };

                if is_wmax {
                    let is_new_day = !last_date.is_empty() && date != last_date;
                    let hhmm_style = if is_new_day {
                        Style::default().fg(PINK)
                    } else {
                        Style::default().fg(TEXT)
                    };
                    let mut text = hhmm;
                    if is_new_day {
                        text = format!("|{}", text);
                    }
                    labels_hhmm.push(Span::styled(format!("{:^6}", text), hhmm_style));
                    if is_new_day || labels_mmdd.is_empty() {
                        labels_mmdd.push(Span::styled(
                            format!("{:^6}", mmdd),
                            Style::default().fg(YELLOW),
                        ));
                    } else {
                        labels_mmdd.push(Span::raw("      "));
                    }
                    last_date = date;
                }

                let group = BarGroup::default()
                    .label(Line::from(label))
                    .bars(&[Bar::default().value(val).style(Style::default().fg(bar_color))]);
                barchart = barchart.data(group);
            }
        }
        f.render_widget(barchart, main_area);

        if let Some(la) = label_area {
            let inner_la = Rect {
                x: la.x + 1,
                y: la.y,
                width: la.width.saturating_sub(2),
                height: 2,
            };
            f.render_widget(
                Paragraph::new(vec![Line::from(labels_hhmm), Line::from(labels_mmdd)]),
                inner_la,
            );
        }
    }

    if app.is_loading {
        let area = centered_rect(30, 10, area);
        f.render_widget(Clear, area);
        f.render_widget(
            Paragraph::new("Loading...")
                .style(Style::default().fg(YELLOW).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(YELLOW)),
                ),
            area,
        );
    }
}

fn render_top_scripts_table(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.focused_pane == Pane::Jobs { PINK } else { TEXT };
    let table_area = area;

    let summaries = &app.summaries;

    let table_title = if app.jobs_table_state.is_searching {
        format!(" Jobs (Search: {}) ", app.jobs_table_state.search_query)
    } else {
        " Jobs ".to_string()
    };

    if summaries.is_empty() {
        let msg = if app.is_loading {
            "Loading..."
        } else {
            "No data available. Waiting for updates..."
        };
        let p = Paragraph::new(msg)
            .style(Style::default().fg(YELLOW))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(table_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            );
        f.render_widget(p, table_area);
        return;
    }

    let focus_cell = app.jobs_table_state.focus_mode == crate::table_state::TableFocusMode::Cell;
    let selected_col = app.jobs_table_state.selected_col;

    let rows: Vec<Row> = summaries
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_selected_row = app.jobs_table_state.row_state.selected() == Some(i);
            let cells_content = vec![
                s.program_name.clone(),
                format_with_commas(s.count as u64),
                format!("{:.2}s", s.avg_wall_time_ms as f64 / 1000.0),
                format!("{:.2}s", s.total_cpu_time_sec),
                format!("{}KB", format_with_commas(s.max_rss_kb)),
            ];

            let row_cells: Vec<Cell> = cells_content.into_iter().enumerate().map(|(j, content)| {
                let is_selected_cell = is_selected_row && focus_cell && selected_col == Some(j);
                
                let display_content = if focus_cell && selected_col == Some(j) {
                    content
                } else if content.len() > 8 {
                    format!("{}...", &content[..8])
                } else {
                    content
                };

                let mut style = Style::default();
                if is_selected_cell {
                    style = style.add_modifier(Modifier::REVERSED).fg(SAPPHIRE);
                }
                Cell::from(display_content).style(style)
            }).collect();

            Row::new(row_cells).style(Style::default().fg(TEXT))
        })
        .collect();

    let mut constraints = vec![
        Constraint::Min(20),
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
    ];

    if focus_cell
        && let Some(col) = selected_col
            && col < constraints.len() {
                constraints[col] = Constraint::Min(25);
            }

    let mut table = Table::new(rows, constraints)
        .header(
            Row::new(vec!["Name", "Uses", "Avg Wall (s)", "Total CPU (s)", "Max RSS"])
                .style(Style::default().add_modifier(Modifier::BOLD).fg(LAVENDER))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(table_title)
                .border_style(Style::default().fg(border_color))
                .style(Style::default().fg(TEXT)),
        );

    if !focus_cell {
        table = table.highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(SAPPHIRE));
    }

    f.render_stateful_widget(table, table_area, &mut app.jobs_table_state.row_state);

    if app.is_loading {
        let area = centered_rect(30, 10, table_area);
        f.render_widget(Clear, area);
        f.render_widget(
            Paragraph::new("Loading...")
                .style(Style::default().fg(YELLOW).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).style(Style::default().fg(YELLOW))),
            area,
        );
    }
}

fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let text = vec![Line::from(vec![
        Span::styled("[Enter]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Detail | "),
        Span::styled("[Tab]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Next Metric | "),
        Span::styled("[Shift+Tab]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Prev Metric | "),
        Span::styled("[t]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Time Window | "),
        Span::styled("[j/k]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Navigate | "),
        Span::styled("[q]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ])];

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(OVERLAY2)),
    );
    f.render_widget(paragraph, area);
}

fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (count, c) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_detail_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 35, f.size());
    f.render_widget(Clear, area);
    
    let (title, content) = if let Some(selected) = app.jobs_table_state.row_state.selected() {
        if let Some(summary) = app.summaries.get(selected) {
            let prog_name = &summary.program_name;
            let prog_jobs: Vec<_> = app.jobs.iter().filter(|j| &j.program_name == prog_name).collect();
            
            let successes = prog_jobs.iter().filter(|j| j.exit_code_int == 0).count();
            let failures = prog_jobs.len() - successes;
            let last_exit = prog_jobs.last().map(|j| j.exit_code_int).unwrap_or(0);
            
            let mut unique_users: Vec<_> = prog_jobs.iter().map(|j| &j.user_name).collect();
            unique_users.sort();
            unique_users.dedup();
            let users_str = if unique_users.is_empty() {
                "None".to_string()
            } else {
                unique_users.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            };

            (
                format!(" Detail: {} ", prog_name),
                vec![
                    Line::from(vec![
                        Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(format!("{} Success", successes), Style::default().fg(GREEN)),
                        Span::raw(" | "),
                        Span::styled(format!("{} Failure", failures), Style::default().fg(RED)),
                    ]),
                    Line::from(vec![
                        Span::styled("Last Exit Code: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(last_exit.to_string(), if last_exit == 0 { Style::default().fg(GREEN) } else { Style::default().fg(RED) }),
                    ]),
                    Line::from(vec![
                        Span::styled("Users: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(users_str),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Avg WallTime: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(format!("{} ms", summary.avg_wall_time_ms), Style::default().fg(BLUE)),
                    ]),
                    Line::from(vec![
                        Span::styled("Total CPU: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(format!("{:.2} s", summary.total_cpu_time_sec), Style::default().fg(PEACH)),
                    ]),
                    Line::from(vec![
                        Span::styled("Max RSS: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(format!("{} KB", summary.max_rss_kb), Style::default().fg(RED)),
                    ]),
                ]
            )
        } else {
            (" No Selection ".to_string(), vec![Line::from("No script selected")])
        }
    } else {
        (" No Selection ".to_string(), vec![Line::from("No script selected")])
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(SAPPHIRE));
        
    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);
        
    f.render_widget(paragraph, area);
}
