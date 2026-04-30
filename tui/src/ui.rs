use crate::app::{App, Metric, TimeWindow};
use crate::theme::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Cell, Paragraph, Row, Table, Tabs},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10), // Header
            Constraint::Percentage(10), // Tabs
            Constraint::Percentage(70), // Main Body
            Constraint::Percentage(10), // Footer
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
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let window_str = format!("{:?}", app.window);

    let text = vec![Line::from(vec![
        Span::styled(
            "Glia Dashboard",
            Style::default().add_modifier(Modifier::BOLD).fg(SAPPHIRE),
        ),
        Span::raw(" | Window: "),
        Span::styled(window_str, Style::default().fg(YELLOW)),
    ])];

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Status")
            .style(Style::default().fg(TEXT)),
    );
    f.render_widget(paragraph, area);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["WallTime", "CpuTime", "CpuPercent", "MaxRss", "JobStatus"];

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
                .title("Metrics")
                .style(Style::default().fg(TEXT)),
        )
        .select(active_index)
        .style(Style::default().fg(SUBTEXT0))
        .highlight_style(Style::default().fg(PINK).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}

fn render_metric_chart(f: &mut Frame, app: &App, area: Rect) {
    let format_time = |started_at: &str| -> String {
        let parts: Vec<&str> = started_at.split('T').collect();
        if parts.len() == 2 {
            if app.window == TimeWindow::WMax {
                let date_parts: Vec<&str> = parts[0].split('-').collect();
                if date_parts.len() >= 3 {
                    format!("{}-{}", date_parts[1], date_parts[2])
                } else {
                    parts[0].to_string()
                }
            } else if parts[1].len() >= 5 {
                parts[1][0..5].to_string()
            } else {
                parts[1].to_string()
            }
        } else {
            started_at.to_string()
        }
    };

    let mut barchart = BarChart::default()
        .bar_width(5)
        .bar_gap(1)
        .value_style(Style::default().fg(CRUST));

    if app.metric == Metric::JobStatus {
        barchart = barchart
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Job Status (Success: Green | Fail: Red)")
                    .style(Style::default().fg(TEXT)),
            )
            .bar_width(2)
            .bar_gap(0)
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
            Metric::WallTime => ("Wall Time (ms)", MAUVE),
            Metric::CpuTime => ("CPU Time (ms)", GREEN),
            Metric::CpuPercent => ("CPU Percent (%)", PEACH),
            Metric::MaxRss => ("Max RSS (KB)", BLUE),
            _ => ("", BLUE),
        };

        barchart = barchart
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(y_title)
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
}

fn render_top_scripts_table(f: &mut Frame, app: &mut App, area: Rect) {
    let summaries = app.summarize_jobs();

    let rows: Vec<Row> = summaries
        .iter()
        .map(|s| {
            Row::new(vec![
                Cell::from(s.program_name.clone()),
                Cell::from(s.count.to_string()),
                Cell::from(format!("{}ms", s.avg_wall_time_ms)),
                Cell::from(format!("{:.2}s", s.total_cpu_time_sec)),
                Cell::from(format!("{}KB", s.max_rss_kb)),
            ])
            .style(Style::default().fg(TEXT))
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .header(
        Row::new(vec!["Script", "Uses", "Avg Wall", "Total CPU", "Max RSS"])
            .style(Style::default().add_modifier(Modifier::BOLD).fg(LAVENDER))
            .bottom_margin(1),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Top Scripts")
            .style(Style::default().fg(TEXT)),
    )
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(SAPPHIRE));

    f.render_stateful_widget(table, area, &mut app.table_state);
}

fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let text = vec![Line::from(vec![
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
