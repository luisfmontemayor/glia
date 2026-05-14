use crate::app::App;
use crate::components::header::render_header;
use crate::components::tabs::render_tabs;
use crate::components::footer::render_footer;
use crate::components::modal::render_modal;
use crate::components::table::render_top_scripts_table;
use crate::components::chart::render_metric_chart;
use crate::theme::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
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

    let (graph_area, jobs_area) = if app.show_command_palette {
        let graph_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(2)])
            .split(main_chunks[0]);
        let jobs_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(2)])
            .split(main_chunks[1]);

        let left_text = Line::from(vec![
            Span::styled("[b]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Blame Mode"),
        ]);
        let right_text = Line::from(vec![
            Span::styled("[/]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Search | "),
            Span::styled("[Enter]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Select | "),
            Span::styled("[Esc]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Unselect | "),
            Span::styled("[s]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Sort | "),
            Span::styled("[Arrows]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Navigate"),
        ]);

        let palette_style = Style::default().fg(OVERLAY2);
        
        f.render_widget(
            Paragraph::new(left_text)
                .style(palette_style)
                .wrap(Wrap { trim: true }),
            graph_split[1],
        );
        f.render_widget(
            Paragraph::new(right_text)
                .style(palette_style)
                .wrap(Wrap { trim: true }),
            jobs_split[1],
        );

        (graph_split[0], jobs_split[0])
    } else {
        (main_chunks[0], main_chunks[1])
    };

    render_metric_chart(f, app, graph_area);
    render_top_scripts_table(f, app, jobs_area);

    render_footer(f, app, chunks[3]);

    if app.show_detail {
        render_modal(f, app);
    }
}
