use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};
use crate::app::{App, Metric};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(f.size());

    render_header(f, app, chunks[0]);
    render_metric_chart(f, app, chunks[1]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let window_str = format!("{:?}", app.window);
    let metric_str = format!("{:?}", app.metric);
    
    let text = vec![
        Line::from(vec![
            Span::styled("Glia Dashboard", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("Window: "),
            Span::styled(window_str, Style::default().fg(Color::Yellow)),
            Span::raw(" | Metric: "),
            Span::styled(metric_str, Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::styled("[Tab]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Next Metric | "),
            Span::styled("[t]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Next Window | "),
            Span::styled("[q]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(paragraph, area);
}

fn render_metric_chart(f: &mut Frame, app: &App, area: Rect) {
    let data: Vec<u64> = match app.metric {
        Metric::WallTime => app.jobs.iter().map(|j| j.wall_time_ms as u64).collect(),
        Metric::CpuTime => app.jobs.iter().map(|j| (j.cpu_time_sec * 1000.0) as u64).collect(),
        Metric::CpuPercent => app.jobs.iter().map(|j| j.cpu_percent as u64).collect(),
        Metric::MaxRss => app.jobs.iter().map(|j| j.max_rss_kb as u64).collect(),
        Metric::Throughput => {
            app.jobs.iter().map(|_| 1u64).collect()
        },
        Metric::SuccessRate => {
            app.jobs.iter().map(|j| if j.exit_code_int == 0 { 100u64 } else { 0u64 }).collect()
        }
    };

    let sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(format!("{:?}", app.metric)))
        .data(&data)
        .style(Style::default().fg(Color::Green));
    
    f.render_widget(sparkline, area);
}
