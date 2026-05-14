use crate::app::{App, Metric};
use crate::theme::*;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Tabs},
};

pub fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
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
                .title(" Metrics • [Tab]/[Shift+Tab] ")
                .border_style(Style::default().fg(LAVENDER))
                .style(Style::default().fg(TEXT)),
        )
        .select(active_index)
        .style(Style::default().fg(SUBTEXT0))
        .highlight_style(Style::default().fg(PINK).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}
