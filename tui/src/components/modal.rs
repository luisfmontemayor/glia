use crate::app::App;
use crate::theme::*;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_modal(f: &mut Frame, app: &App) {
    let area = crate::utils::centered_rect(50, 35, f.size());
    f.render_widget(Clear, area);

    let (title, content) = if let Some(selected) = app.jobs_table_state.row_state.selected() {
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
                format!(" Detail: {} ", prog_name),
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
                " No Selection ".to_string(),
                vec![Line::from("No script selected")],
            )
        }
    } else {
        (
            " No Selection ".to_string(),
            vec![Line::from("No script selected")],
        )
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(SAPPHIRE));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, area);
}

pub fn render_no_data_modal(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let modal_area = crate::utils::centered_rect(60, 20, area);
    f.render_widget(Clear, modal_area);

    let content = vec![
        Line::from("No data available for time window."),
        Line::from("Waiting for updates..."),
    ];

    let p = Paragraph::new(content)
        .style(Style::default().fg(YELLOW))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(YELLOW))
                .padding(ratatui::widgets::Padding::vertical(1)),
        );
    f.render_widget(p, modal_area);
}
