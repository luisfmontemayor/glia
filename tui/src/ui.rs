use crate::app::{App, Metric, Pane, TimeWindow};
use crate::theme::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs},
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
    let window_str = match app.window {
        TimeWindow::W1h => "1 Hour",
        TimeWindow::W3h => "3 Hours",
        TimeWindow::W6h => "6 Hours",
        TimeWindow::W12h => "12 Hours",
        TimeWindow::W24h => "24 Hours",
        TimeWindow::WMax => "All Time",
    };

    let db_color = if app.db_status { GREEN } else { RED };
    let api_color = if app.api_status { GREEN } else { RED };

    let text = vec![Line::from(vec![
        Span::styled(
            format!(" {} ", app.org_name),
            Style::default().add_modifier(Modifier::BOLD).fg(SAPPHIRE),
        ),
        Span::raw(" | Window: "),
        Span::styled(window_str, Style::default().fg(YELLOW)),
        Span::raw(" | DB: "),
        Span::styled("●", Style::default().fg(db_color)),
        Span::raw(" | API: "),
        Span::styled("●", Style::default().fg(api_color)),
    ])];

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Status ")
            .style(Style::default().fg(TEXT)),
    );
    f.render_widget(paragraph, area);
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

    let chart_title = if app.metric == Metric::JobStatus {
        " Job Status (Success: Green | Fail: Red) "
    } else {
        match app.metric {
            Metric::WallTime => " Wall Time (ms) ",
            Metric::CpuTime => " CPU Time (ms) ",
            Metric::CpuPercent => " CPU Percent (%) ",
            Metric::MaxRss => " Max RSS (KB) ",
            _ => " Metrics ",
        }
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

    let format_time = |started_at: &str| -> String {
        // Expected format: 2023-10-27T10:30:00Z or similar ISO8601
        let parts: Vec<&str> = started_at.split('T').collect();
        if parts.len() < 2 {
            return started_at.chars().take(10).collect();
        }
        let date = parts[0]; // YYYY-MM-DD
        let time = parts[1]; // HH:MM:SS...

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
            time.to_string()
        };

        if app.window == TimeWindow::WMax {
            format!("{} {}", mm_dd, hh_mm)
        } else {
            hh_mm
        }
    };

    let mut barchart = BarChart::default()
        .bar_width(5)
        .bar_gap(1)
        .bar_set(symbols::bar::NINE_LEVELS)
        .value_style(Style::default().fg(CRUST).bg(TEXT));

    if app.metric == Metric::JobStatus {
        barchart = barchart
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Job Status (Success: Green | Fail: Red) ")
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            )
            .bar_width(2)
            .bar_gap(1)
            .group_gap(2);

        for j in &app.jobs {
            let label = format_time(&j.started_at);
            let success_val = if j.exit_code_int == 0 { 1 } else { 0 };
            let fail_val = if j.exit_code_int != 0 { 1 } else { 0 };

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
        let (y_title, bar_color) = match app.metric {
            Metric::WallTime => (" Wall Time (ms) ", BLUE),
            Metric::CpuTime => (" CPU Time (ms) ", PEACH),
            Metric::CpuPercent => (" CPU Percent (%) ", GREEN),
            Metric::MaxRss => (" Max RSS (KB) ", RED),
            _ => (" Metrics ", BLUE),
        };

        barchart = barchart
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(y_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            )
            .bar_style(Style::default().fg(bar_color));

        for j in &app.jobs {
            let label = format_time(&j.started_at);
            let val = match app.metric {
                Metric::WallTime => j.wall_time_ms as u64,
                Metric::CpuTime => (j.cpu_time_sec * 1000.0) as u64,
                Metric::CpuPercent => j.cpu_percent as u64,
                Metric::MaxRss => j.max_rss_kb as u64,
                _ => 0,
            };
            let group = BarGroup::default()
                .label(Line::from(label))
                .bars(&[Bar::default()
                    .value(val)
                    .style(Style::default().fg(bar_color))]);
            barchart = barchart.data(group);
        }
    }

    f.render_widget(barchart, area);

    if app.is_loading {
        let area = centered_rect(30, 10, area);
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

fn render_top_scripts_table(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.focused_pane == Pane::Jobs { PINK } else { TEXT };
    let (table_area, error_area) = if let Some(msg) = &app.error_message {
        let text_len = msg.chars().count() as u16;
        let available_width = area.width.saturating_sub(2).max(1);
        let required_height = (text_len.saturating_add(available_width - 1) / available_width) + 2;
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(required_height)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    if let Some(ea) = error_area {
        let p = Paragraph::new(app.error_message.as_ref().unwrap().clone())
            .style(Style::default().fg(RED))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Error ")
                    .style(Style::default().fg(RED)),
            );
        f.render_widget(p, ea);
    }

    let summaries = &app.summaries;

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
                    .title(" Jobs ")
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            );
        f.render_widget(p, table_area);
        return;
    }

    let rows: Vec<Row> = summaries
        .iter()
        .map(|s| {
            Row::new(vec![
                Cell::from(s.program_name.clone()),
                Cell::from(format_with_commas(s.count as u64)),
                Cell::from(format!("{:.2}s", s.avg_wall_time_ms as f64 / 1000.0)),
                Cell::from(format!("{:.2}s", s.total_cpu_time_sec)),
                Cell::from(format!("{}KB", format_with_commas(s.max_rss_kb))),
            ])
            .style(Style::default().fg(TEXT))
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(20),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .header(
        Row::new(vec!["Name", "Uses", "Avg Wall (s)", "Total CPU (s)", "Max RSS"])
            .style(Style::default().add_modifier(Modifier::BOLD).fg(LAVENDER))
            .bottom_margin(1),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Jobs ")
            .border_style(Style::default().fg(border_color))
            .style(Style::default().fg(TEXT)),
    )
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(SAPPHIRE));

    f.render_stateful_widget(table, table_area, &mut app.table_state);

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
    let mut count = 0;
    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
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
    
    let (title, content) = if let Some(selected) = app.table_state.selected() {
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
