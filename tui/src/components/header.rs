use crate::app::{App, TimeWindow};
use crate::theme::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_header(f: &mut Frame, app: &App, area: Rect) {
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
        Span::raw(" | Time Window [t/T]: "),
        Span::styled(
            window_str,
            Style::default().fg(YELLOW).add_modifier(Modifier::BOLD),
        ),
    ])];

    let main_paragraph = Paragraph::new(main_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Glia TUI ")
                .border_style(Style::default().fg(SAPPHIRE))
                .style(Style::default().fg(TEXT)),
        )
        .wrap(Wrap { trim: true });
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
                .border_style(Style::default().fg(SAPPHIRE))
                .style(Style::default().fg(TEXT)),
        )
        .wrap(Wrap { trim: true });
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
                .border_style(Style::default().fg(SAPPHIRE))
                .style(Style::default().fg(TEXT)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(api_paragraph, header_chunks[2]);
}
