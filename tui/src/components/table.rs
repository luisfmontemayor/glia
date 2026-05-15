pub mod table_state;

use crate::app::{App, Pane};
use crate::theme::*;
use crate::utils::centered_rect;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

pub fn render_top_scripts_table(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.focused_pane == Pane::Jobs {
        PINK
    } else {
        TEAL
    };
    let (table_area, error_area) = if let Some(msg) = &app.error_message {
        let text_len = msg.chars().count() as u16;
        let available_width = area.width.saturating_sub(2).max(1);
        let required_height = (text_len.saturating_add(available_width - 1) / available_width) + 2;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(required_height)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    if let Some(ea) = error_area {
        let p = Paragraph::new(app.error_message.as_ref().unwrap().clone())
            .style(Style::default().fg(RED))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Error ")
                    .style(Style::default().fg(RED)),
            );
        f.render_widget(p, ea);
    }

    let summaries = &app.summaries;

    let table_title = if app.jobs_table_state.is_searching {
        format!(
            " Jobs (Search: {}) • [j] ",
            app.jobs_table_state.search_query
        )
    } else {
        " Jobs • [j] ".to_string()
    };

    if summaries.is_empty() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(table_title)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().fg(TEXT));
        f.render_widget(block, table_area);

        if app.is_loading {
            let area = centered_rect(30, 10, table_area);
            f.render_widget(Clear, area);
            f.render_widget(
                Paragraph::new("Loading...")
                    .style(Style::default().fg(YELLOW).add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Center)
                    .wrap(ratatui::widgets::Wrap { trim: true })
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(border_color))
                            .style(Style::default().fg(TEXT)),
                    ),
                area,
            );
        }
        return;
    }

    let focus_cell = app.jobs_table_state.focus_mode
        == crate::components::table::table_state::TableFocusMode::Cell;
    let focus_col = app.jobs_table_state.focus_mode
        == crate::components::table::table_state::TableFocusMode::Column;
    let focus_row = app.jobs_table_state.focus_mode
        == crate::components::table::table_state::TableFocusMode::Row;
    let selected_col = app.jobs_table_state.selected_col;

    let rows: Vec<Row> = summaries
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let cells_content = vec![
                s.program_name.clone(),
                crate::utils::format_with_commas(s.count as u64),
                format!("{:.2}", s.avg_wall_time_ms as f64 / 1000.0),
                format!("{:.2}", s.total_cpu_time_sec),
                crate::utils::format_with_commas(s.max_rss_kb),
            ];

            let row_cells: Vec<Cell> = cells_content
                .into_iter()
                .enumerate()
                .map(|(j, content)| {
                    let is_active_col = selected_col == Some(j);

                    let display_content = if (focus_cell || focus_col) && is_active_col {
                        content
                    } else if content.len() > 21 {
                        format!("{}...", &content[..21])
                    } else {
                        content
                    };

                    let mut style = Style::default();
                    if focus_cell {
                        if app.jobs_table_state.row_state.selected() == Some(i) && is_active_col {
                            style = style.bg(SAPPHIRE).fg(BASE);
                        }
                    } else if focus_col && is_active_col {
                        style = style.bg(SAPPHIRE).fg(BASE);
                    }

                    Cell::from(display_content).style(style)
                })
                .collect();

            Row::new(row_cells).style(Style::default().fg(TEXT))
        })
        .collect();

    let mut constraints = vec![
        Constraint::Min(25),
        Constraint::Length(8),
        Constraint::Length(14),
        Constraint::Length(15),
        Constraint::Length(12),
    ];

    if focus_cell || focus_col {
        if let Some(col) = selected_col {
            if col < constraints.len() {
                let mut max_len = match col {
                    0 => 25,
                    1 => 8,
                    2 => 14,
                    3 => 15,
                    4 => 12,
                    _ => 0,
                };

                if focus_cell {
                    if let Some(row_idx) = app.jobs_table_state.row_state.selected() {
                        if let Some(s) = summaries.get(row_idx) {
                            let content_len = match col {
                                0 => s.program_name.chars().count() as u16,
                                1 => crate::utils::format_with_commas(s.count as u64)
                                    .chars()
                                    .count() as u16,
                                2 => format!("{:.2}", s.avg_wall_time_ms as f64 / 1000.0)
                                    .chars()
                                    .count() as u16,
                                3 => format!("{:.2}", s.total_cpu_time_sec).chars().count() as u16,
                                4 => crate::utils::format_with_commas(s.max_rss_kb)
                                    .chars()
                                    .count() as u16,
                                _ => 0,
                            };
                            max_len = max_len.max(content_len);
                        }
                    }
                } else if focus_col {
                    for s in summaries {
                        let content_len = match col {
                            0 => s.program_name.chars().count() as u16,
                            1 => crate::utils::format_with_commas(s.count as u64)
                                .chars()
                                .count() as u16,
                            2 => format!("{:.2}", s.avg_wall_time_ms as f64 / 1000.0)
                                .chars()
                                .count() as u16,
                            3 => format!("{:.2}", s.total_cpu_time_sec).chars().count() as u16,
                            4 => crate::utils::format_with_commas(s.max_rss_kb)
                                .chars()
                                .count() as u16,
                            _ => 0,
                        };
                        max_len = max_len.max(content_len);
                    }
                }

                constraints[col] = Constraint::Min(max_len);
            }
        }
    }

    let mut header_titles = vec![
        "Name".to_string(),
        "Uses".to_string(),
        "Avg Wall (s)".to_string(),
        "Total CPU (s)".to_string(),
        "Max RSS (KB)".to_string(),
    ];

    if let Some(col) = app.jobs_table_state.sort_col {
        if col < header_titles.len() {
            let indicator = if app.jobs_table_state.sort_desc {
                " ▼"
            } else {
                " ▲"
            };
            header_titles[col].push_str(indicator);
        }
    }

    let header_cells: Vec<Cell> = header_titles
        .into_iter()
        .enumerate()
        .map(|(j, t)| {
            if focus_col && Some(j) == selected_col {
                Cell::from(t).style(
                    Style::default()
                        .fg(DARK_BLUE)
                        .add_modifier(Modifier::REVERSED | Modifier::BOLD),
                )
            } else {
                Cell::from(t).style(Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD))
            }
        })
        .collect();

    let mut table = Table::new(rows, constraints.clone())
        .header(Row::new(header_cells).height(1))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(table_title)
                .border_style(Style::default().fg(border_color))
                .style(Style::default().fg(TEXT)),
        );

    if focus_row {
        table = table.highlight_style(Style::default().bg(SAPPHIRE).fg(BASE));
    }

    let total_rows = summaries.len();
    let offset = app.jobs_table_state.row_state.offset();
    let overhead = 3; // 2 for borders, 1 for header
    let visible_rows_no_indicator = table_area.height.saturating_sub(overhead) as usize;
    let show_indicator = offset + visible_rows_no_indicator < total_rows;

    let (actual_table_area, indicator_area) = if show_indicator {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(table_area);
        (chunks[0], Some(chunks[1]))
    } else {
        (table_area, None)
    };

    f.render_stateful_widget(
        table,
        actual_table_area,
        &mut app.jobs_table_state.row_state,
    );

    if let Some(ia) = indicator_area {
        let indicator = Paragraph::new("▼")
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .style(Style::default().bg(SURFACE1).fg(TEXT));
        f.render_widget(indicator, ia);
    }

    if app.is_loading {
        let area = centered_rect(30, 10, table_area);
        f.render_widget(Clear, area);
        f.render_widget(
            Paragraph::new("Loading...")
                .style(Style::default().fg(YELLOW).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(border_color))
                        .style(Style::default().fg(TEXT)),
                ),
            area,
        );
    }
}
