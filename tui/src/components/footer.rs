use crate::app::App;
use crate::theme::*;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let text = vec![Line::from(vec![
        Span::styled("[p]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Command Palette | "),
        Span::styled("[q]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ])];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(OVERLAY2)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
