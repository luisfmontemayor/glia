use crate::app::{App, Metric};
use crate::theme::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, Paragraph, Tabs},
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10), // Header
            Constraint::Percentage(10), // Tabs
            Constraint::Percentage(70), // Chart
            Constraint::Percentage(10), // Footer
        ])
        .split(f.size());

    render_header(f, app, chunks[0]);
    render_tabs(f, app, chunks[1]);
    render_metric_chart(f, app, chunks[2]);
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
    let titles = vec![
        "WallTime",
        "CpuTime",
        "CpuPercent",
        "MaxRss",
        "Throughput",
        "SuccessRate",
    ];

    let active_index = match app.metric {
        Metric::WallTime => 0,
        Metric::CpuTime => 1,
        Metric::CpuPercent => 2,
        Metric::MaxRss => 3,
        Metric::Throughput => 4,
        Metric::SuccessRate => 5,
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
    let mut str_data: Vec<String> = Vec::new();
    let mut num_data: Vec<u64> = Vec::new();

    let (y_title, bar_color) = match app.metric {
        Metric::WallTime => {
            for (i, j) in app.jobs.iter().enumerate() {
                str_data.push(format!("#{}", i));
                num_data.push(j.wall_time_ms as u64);
            }
            ("Wall Time (ms)", MAUVE)
        }
        Metric::CpuTime => {
            for (i, j) in app.jobs.iter().enumerate() {
                str_data.push(format!("#{}", i));
                num_data.push((j.cpu_time_sec * 1000.0) as u64);
            }
            ("CPU Time (ms)", GREEN)
        }
        Metric::CpuPercent => {
            for (i, j) in app.jobs.iter().enumerate() {
                str_data.push(format!("#{}", i));
                num_data.push(j.cpu_percent as u64);
            }
            ("CPU Percent (%)", PEACH)
        }
        Metric::MaxRss => {
            for (i, j) in app.jobs.iter().enumerate() {
                str_data.push(format!("#{}", i));
                num_data.push(j.max_rss_kb as u64);
            }
            ("Max RSS (KB)", BLUE)
        }
        Metric::Throughput => {
            for (i, _) in app.jobs.iter().enumerate() {
                str_data.push(format!("#{}", i));
                num_data.push(1);
            }
            ("Throughput", TEAL)
        }
        Metric::SuccessRate => {
            for (i, j) in app.jobs.iter().enumerate() {
                str_data.push(format!("#{}", i));
                num_data.push(if j.exit_code_int == 0 { 100 } else { 0 });
            }
            ("Success Rate (%)", GREEN)
        }
    };

    let chart_data: Vec<(&str, u64)> = str_data
        .iter()
        .zip(num_data.iter())
        .map(|(s, n)| (s.as_str(), *n))
        .collect();

    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(y_title)
                .style(Style::default().fg(TEXT)),
        )
        .data(&chart_data)
        .bar_width(5)
        .bar_gap(1)
        .bar_style(Style::default().fg(bar_color))
        .value_style(Style::default().fg(CRUST).bg(bar_color));

    f.render_widget(barchart, area);
}

fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let text = vec![Line::from(vec![
        Span::styled("[Tab]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Next Metric | "),
        Span::styled("[Shift+Tab]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Prev Metric | "),
        Span::styled("[t]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Time Window | "),
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
