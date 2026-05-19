use crate::app::App;
use crate::theme::*;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_modal(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let modal_area = crate::utils::centered_rect(50, 40, area);
    f.render_widget(Clear, modal_area);

    let (title, content) = if app.jobs.is_empty() {
        let msg = if app.is_loading {
            "Loading..."
        } else {
            "No data available for this time window"
        };
        (None, vec![Line::from(msg)])
    } else if let Some(selected) = app.jobs_table_state.row_state.selected() {
        if let Some(summary) = app.summaries.get(selected) {
            let prog_name = &summary.program_name;
            let prog_jobs: Vec<_> = app
                .jobs
                .iter()
                .filter(|j| &j.program_name == prog_name)
                .collect();

            let successes = prog_jobs.iter().filter(|j| j.exit_code_int == 0).count();
            let failures = prog_jobs.len() - successes;
            let last_exit = prog_jobs.last().map(|j| j.exit_code_int).unwrap_or(0);

            let mut unique_users: Vec<_> = prog_jobs.iter().map(|j| &j.user_name).collect();
            unique_users.sort();
            unique_users.dedup();
            let users_str = if unique_users.is_empty() {
                "None".to_string()
            } else {
                unique_users
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            (
                Some(format!(" Detail: {} ", prog_name)),
                vec![
                    Line::from(vec![
                        Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(format!("{} Success", successes), Style::default().fg(GREEN)),
                        Span::raw(" | "),
                        Span::styled(format!("{} Failure", failures), Style::default().fg(RED)),
                    ]),
                    Line::from(vec![
                        Span::styled(
                            "Last Exit Code: ",
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            last_exit.to_string(),
                            if last_exit == 0 {
                                Style::default().fg(GREEN)
                            } else {
                                Style::default().fg(RED)
                            },
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("Users: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(users_str),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(
                            "Avg WallTime: ",
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{} ms", summary.avg_wall_time_ms),
                            Style::default().fg(BLUE),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("Total CPU: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(
                            format!("{:.2} s", summary.total_cpu_time_sec),
                            Style::default().fg(PEACH),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("Max RSS: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(
                            format!("{} KB", summary.max_rss_kb),
                            Style::default().fg(RED),
                        ),
                    ]),
                ],
            )
        } else {
            (
                Some(" No Selection ".to_string()),
                vec![Line::from("No script selected")],
            )
        }
    } else {
        (
            Some(" No Selection ".to_string()),
            vec![Line::from("No script selected")],
        )
    };

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(SAPPHIRE))
        .style(Style::default().bg(CRUST));

    if let Some(t) = title {
        block = block.title(t);
    }

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, modal_area);
}

pub fn render_no_data_modal(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let modal_height_pct = 20;
    let modal_area = crate::utils::centered_rect(50, modal_height_pct, area);

    f.render_widget(Clear, modal_area);

    let content = vec![
        Line::from("No data available for time window."),
        Line::from("Waiting for updates..."),
    ];

    // Center 2 lines of text vertically:
    // Available height = modal_area.height - 2 (borders)
    let inner_height = modal_area.height.saturating_sub(2);
    let v_pad = inner_height.saturating_sub(2) / 2;
    let h_pad = 2; // Fixed small horizontal padding

    let p = Paragraph::new(content)
        .style(Style::default().fg(YELLOW).bg(CRUST))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(YELLOW))
                .style(Style::default().bg(CRUST))
                .padding(ratatui::widgets::Padding::new(h_pad, h_pad, v_pad, v_pad)),
        );
    f.render_widget(p, modal_area);
}
