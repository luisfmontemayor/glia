use crate::app::App;
use crate::components::chart::render_metric_chart;
use crate::components::footer::render_footer;
use crate::components::header::render_header;
use crate::components::modal::render_modal;
use crate::components::table::render_top_scripts_table;
use crate::components::tabs::render_tabs;
use crate::theme::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main Body
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
        let right_keybinds = vec![
            ("[/]", "Search"),
            ("[Enter]", "Select"),
            ("[Esc]", "Unselect"),
            ("[s]", "Sort"),
            ("[Arrows]", "Navigate"),
        ];

        let available_width = main_chunks[1].width.saturating_sub(2);
        let mut required_height = 1;
        if available_width > 0 {
            let mut current_line_width = 0;
            for (i, (key, desc)) in right_keybinds.iter().enumerate() {
                let unit_width = (key.chars().count() + 1 + desc.chars().count()) as u16;
                if i == 0 {
                    current_line_width = unit_width;
                } else {
                    if current_line_width + 3 + unit_width <= available_width {
                        current_line_width += 3 + unit_width;
                    } else {
                        required_height += 1;
                        current_line_width = unit_width;
                    }
                }
            }
        }
        let palette_height = required_height + 2;

        let graph_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(palette_height)])
            .split(main_chunks[0]);
        let jobs_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(palette_height)])
            .split(main_chunks[1]);

        let left_text = Line::from(vec![
            Span::styled("[b]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("\u{00a0}Blame Mode"),
        ]);

        let mut right_spans = Vec::new();
        for (i, (key, desc)) in right_keybinds.iter().enumerate() {
            if i > 0 {
                right_spans.push(Span::raw(" |\u{00a0}"));
            }
            right_spans.push(Span::styled(*key, Style::default().add_modifier(Modifier::BOLD)));
            right_spans.push(Span::raw(format!("\u{00a0}{}", desc)));
        }
        let right_text = Line::from(right_spans);

        let palette_style = Style::default().fg(OVERLAY2);
        let palette_block = Block::default()
            .borders(Borders::ALL)
            .border_style(palette_style);

        f.render_widget(
            Paragraph::new(left_text)
                .block(palette_block.clone())
                .style(palette_style)
                .wrap(Wrap { trim: true }),
            graph_split[1],
        );
        f.render_widget(
            Paragraph::new(right_text)
                .block(palette_block)
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

    if app.jobs.is_empty() && !app.is_loading {
        crate::components::modal::render_no_data_modal(f, app, f.size());
    }

    if app.show_detail {
        render_modal(f, app);
    }
}
