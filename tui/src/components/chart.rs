use crate::app::{App, Metric, Pane};
use crate::theme::*;
use crate::utils::centered_rect;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::Span,
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, Borders, Chart, Clear, Dataset, GraphType,
        LegendPosition, Paragraph,
    },
};
use std::collections::HashMap;

pub fn render_metric_chart(f: &mut Frame, app: &App, area: Rect) {
    let border_color = if app.focused_pane == Pane::Graph {
        PINK
    } else {
        BLUE
    };

    let chart_title = match app.metric {
        Metric::WallTime => " Wall Time (ms) • [g] ",
        Metric::CpuTime => " CPU Time (ms) • [g] ",
        Metric::CpuPercent => " CPU Percent (%) • [g] ",
        Metric::MaxRss => " Max RSS (KB) • [g] ",
        Metric::JobStatus => " Job Success/Failure • [g] ",
    };

    if app.jobs.is_empty() {
        let msg = if app.is_loading {
            "Loading..."
        } else {
            "No data available for this time window"
        };
        let p = Paragraph::new(msg)
            .style(Style::default().fg(YELLOW))
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(chart_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT)),
            );
        f.render_widget(p, area);
        return;
    }

    let parse_time = |started_at: &str| -> (String, String, String) {
        let parts: Vec<&str> = started_at.split('T').collect();
        if parts.len() < 2 {
            return (
                "??:??".to_string(),
                "????".to_string(),
                started_at.to_string(),
            );
        }
        let date = parts[0];
        let time = parts[1];
        let date_parts: Vec<&str> = date.split('-').collect();
        let mm_dd = if date_parts.len() >= 3 {
            format!("{}-{}", date_parts[1], date_parts[2])
        } else {
            date.to_string()
        };
        let time_parts: Vec<&str> = time.split(':').collect();
        let hh_mm = if time_parts.len() >= 2 {
            format!("{}:{}", time_parts[0], time_parts[1])
        } else {
            time.chars().take(5).collect()
        };
        (hh_mm, mm_dd, date.to_string())
    };

    if app.blame_mode {
        let n_jobs = app.jobs.len();
        let is_low_density = n_jobs > 0 && n_jobs < app.data_point_threshold;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1), // Second line of dual-line labels
            ])
            .split(area);
        let chart_area = chunks[0];
        let labels_area = chunks[1];

        let mut job_points = Vec::new();
        for j in &app.jobs {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&j.started_at) {
                job_points.push((dt.timestamp() as f64, j));
            }
        }
        job_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut user_data: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
        let mut max_y = 0.0f64;
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;

        for (i, (ts, j)) in job_points.iter().enumerate() {
            let x = if is_low_density {
                (i + 1) as f64 / (n_jobs + 1) as f64
            } else {
                *ts
            };

            let val = match app.metric {
                Metric::WallTime => j.wall_time_ms as f64,
                Metric::CpuTime => (j.cpu_time_sec * 1000.0) as f64,
                Metric::CpuPercent => j.cpu_percent as f64,
                Metric::MaxRss => j.max_rss_kb as f64,
                Metric::JobStatus => {
                    if j.exit_code_int == 0 {
                        1.0
                    } else {
                        0.0
                    }
                }
            };

            user_data
                .entry(j.user_name.clone())
                .or_default()
                .push((x, val));
            max_y = max_y.max(val);
            if !is_low_density {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            }
        }

        if is_low_density {
            min_x = 0.0;
            max_x = 1.0;
        } else {
            if min_x == f64::MAX {
                min_x = 0.0;
                max_x = 1.0;
            } else if max_x == min_x {
                min_x -= 1800.0;
                max_x += 1800.0;
            } else if n_jobs <= 3 {
                let padding = (max_x - min_x) * 0.20;
                min_x -= padding;
                max_x += padding;
            }
        }

        if max_y <= 0.0 {
            max_y = 1.0;
        }

        let mut user_names: Vec<_> = user_data.keys().cloned().collect();
        user_names.sort();

        let mut sorted_data = Vec::new();
        for name in &user_names {
            let mut d = user_data.get(name).unwrap().clone();
            d.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            sorted_data.push(d);
        }

        let mut marker_line = Vec::new();
        if is_low_density {
            let marker_height = max_y * 0.05;
            for i in 0..n_jobs {
                let x = (i + 1) as f64 / (n_jobs + 1) as f64;
                marker_line.push((x, marker_height));
                marker_line.push((x, 0.0));
                if i + 1 < n_jobs {
                    let next_x = (i + 2) as f64 / (n_jobs + 1) as f64;
                    marker_line.push((next_x, 0.0));
                }
            }
        } else if n_jobs <= 3 {
            for data in &sorted_data {
                for &(x, _) in data {
                    marker_line.push((x, max_y * 1.1));
                    marker_line.push((x, 0.0));
                }
            }
        }

        let colors = [PINK, GREEN, BLUE, YELLOW, SAPPHIRE];
        let mut datasets: Vec<_> = sorted_data
            .iter()
            .enumerate()
            .map(|(i, data)| {
                Dataset::default()
                    .name(user_names[i].as_str())
                    .marker(ratatui::symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(colors[i % colors.len()]))
                    .data(data)
            })
            .collect();

        if !marker_line.is_empty() {
            datasets.push(
                Dataset::default()
                    .marker(ratatui::symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(OVERLAY0))
                    .data(&marker_line),
            );
        }

        let mut x_labels = Vec::new();
        if is_low_density {
            x_labels.push(Span::styled("", Style::default().fg(YELLOW)));
            for (_, j) in &job_points {
                let (hhmm, _, _) = parse_time(&j.started_at);
                x_labels.push(Span::styled(hhmm, Style::default().fg(YELLOW)));
            }
            x_labels.push(Span::styled("", Style::default().fg(YELLOW)));
        } else {
            x_labels.push(Span::styled(
                chrono::DateTime::from_timestamp(min_x as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M")
                    .to_string(),
                Style::default().fg(YELLOW),
            ));
            x_labels.push(Span::styled(
                chrono::DateTime::from_timestamp(max_x as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M")
                    .to_string(),
                Style::default().fg(YELLOW),
            ));
        }

        let chart = Chart::new(datasets)
            .legend_position(Some(LegendPosition::TopRight))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(chart_title)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().fg(TEXT))
                    .padding(ratatui::widgets::Padding::uniform(1)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(TEXT))
                    .bounds([min_x, max_x])
                    .labels(x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Value")
                    .style(Style::default().fg(TEXT))
                    .bounds([0.0, max_y * 1.1])
                    .labels(vec![Span::raw("0"), Span::raw(format!("{:.0}", max_y))]),
            );

        f.render_widget(chart, chart_area);

        let mut last_date = String::new();
        if is_low_density {
            let y_axis_width = format!("{:.0}", max_y).len() as u16 + 4;
            let data_area_width = chart_area.width.saturating_sub(y_axis_width + 4);
            for (i, (_, j)) in job_points.iter().enumerate() {
                let (_, mmdd, date) = parse_time(&j.started_at);
                let x_offset = ((i + 1) as f32 / (n_jobs + 1) as f32 * data_area_width as f32) as u16;
                let label_x = chart_area.x + y_axis_width + x_offset.saturating_sub(2);
                if label_x < labels_area.right() && date != last_date {
                    f.render_widget(
                        Paragraph::new(mmdd).style(Style::default().fg(YELLOW)),
                        Rect::new(label_x, labels_area.y, 5, 1)
                    );
                }
                last_date = date;
            }
        } else if !job_points.is_empty() {
            let y_axis_width = format!("{:.0}", max_y).len() as u16 + 4;
            let (_, min_mmdd, _) = parse_time(&job_points.first().unwrap().1.started_at);
            
            f.render_widget(
                Paragraph::new(min_mmdd).style(Style::default().fg(YELLOW)),
                Rect::new(chart_area.x + y_axis_width, labels_area.y, 5, 1)
            );
        }
    } else {
        let n_jobs = app.jobs.len();
        let is_low_density = n_jobs > 0 && n_jobs < app.data_point_threshold;
        let group_size = if app.metric == Metric::JobStatus {
            2
        } else {
            1
        };

        let mut max_val = 0;
        if app.metric == Metric::JobStatus {
            max_val = 8;
        } else {
            for j in &app.jobs {
                let v = match app.metric {
                    Metric::WallTime => j.wall_time_ms as u64,
                    Metric::CpuTime => (j.cpu_time_sec * 1000.0) as u64,
                    Metric::CpuPercent => j.cpu_percent as u64,
                    Metric::MaxRss => j.max_rss_kb as u64,
                    _ => 0,
                };
                if v > max_val {
                    max_val = v;
                }
            }
        }

        let available_width = area.width.saturating_sub(4);
        let (b_width, b_gap, g_gap) = if is_low_density {
            let bg = 1;
            let gg = if group_size > 1 { 2 } else { 1 };
            let overhead_per_group = group_size as u16 * bg + gg;
            let total_overhead = if n_jobs > 0 {
                n_jobs as u16 * overhead_per_group - bg - gg
            } else {
                0
            };
            let bw = (available_width.saturating_sub(total_overhead)
                / (n_jobs as u16 * group_size as u16).max(1))
                .max(1)
                .min(20);
            (bw, bg, gg)
        } else if app.metric == Metric::JobStatus {
            (2, 1, 2)
        } else {
            (5, 1, 1)
        };

        let chart_block = Block::default()
            .borders(Borders::ALL)
            .title(chart_title)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().fg(TEXT))
            .padding(ratatui::widgets::Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            });

        let inner_area = chart_block.inner(area);
        f.render_widget(chart_block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1), // Axis line
                Constraint::Length(2), // Dual line labels
            ])
            .split(inner_area);
        let chart_area = chunks[0];
        let axis_area = chunks[1];
        let labels_area = chunks[2];

        f.render_widget(
            Paragraph::new(symbols::line::HORIZONTAL.repeat(axis_area.width as usize))
                .style(Style::default().fg(TEXT)),
            axis_area,
        );

        let mut barchart = BarChart::default()
            .bar_set(symbols::bar::NINE_LEVELS)
            .value_style(Style::default().fg(CRUST).bg(TEXT))
            .bar_width(b_width)
            .bar_gap(b_gap)
            .group_gap(g_gap);

        if app.metric == Metric::JobStatus {
            barchart = barchart.max(8);
            let mut last_date = String::new();
            for (i, j) in app.jobs.iter().enumerate() {
                let s_render_val = if j.exit_code_int == 0 { 8 } else { 1 };
                let s_text = if j.exit_code_int == 0 {
                    "1".to_string()
                } else {
                    "0".to_string()
                };
                let s_style = if j.exit_code_int == 0 {
                    Style::default().fg(GREEN)
                } else {
                    Style::default().fg(SURFACE1)
                };

                let f_render_val = if j.exit_code_int != 0 { 8 } else { 1 };
                let f_text = if j.exit_code_int != 0 {
                    "1".to_string()
                } else {
                    "0".to_string()
                };
                let f_style = if j.exit_code_int != 0 {
                    Style::default().fg(RED)
                } else {
                    Style::default().fg(SURFACE1)
                };

                let (hhmm, mmdd, date) = parse_time(&j.started_at);
                let is_new_day = date != last_date;
                
                let group_width = group_size as u16 * b_width + (group_size as u16).saturating_sub(1) * b_gap;
                let group_x = chart_area.x + i as u16 * (group_width + g_gap);
                let label_x = group_x + (group_width.saturating_sub(5) / 2);

                if label_x < chart_area.right() {
                    f.render_widget(
                        Paragraph::new(hhmm).style(Style::default().fg(YELLOW)),
                        Rect::new(label_x, labels_area.y, 5.min(chart_area.right().saturating_sub(label_x)), 1)
                    );
                    if is_new_day {
                        f.render_widget(
                            Paragraph::new(mmdd).style(Style::default().fg(YELLOW)),
                            Rect::new(label_x, labels_area.y + 1, 5.min(chart_area.right().saturating_sub(label_x)), 1)
                        );
                    }
                }
                last_date = date;

                let group = BarGroup::default()
                    .bars(&[
                    Bar::default()
                        .value(s_render_val)
                        .text_value(s_text)
                        .style(s_style),
                    Bar::default()
                        .value(f_render_val)
                        .text_value(f_text)
                        .style(f_style),
                ]);
                barchart = barchart.data(group);
            }
        } else {
            let bar_color = match app.metric {
                Metric::WallTime => MAUVE,
                Metric::CpuTime => SAPPHIRE,
                Metric::CpuPercent => PEACH,
                Metric::MaxRss => LAVENDER,
                _ => TEXT,
            };

            barchart = barchart
                .bar_style(Style::default().fg(bar_color))
                .max(max_val.max(8));

            let mut last_date = String::new();
            for (i, j) in app.jobs.iter().enumerate() {
                let orig_val = match app.metric {
                    Metric::WallTime => j.wall_time_ms as u64,
                    Metric::CpuTime => (j.cpu_time_sec * 1000.0) as u64,
                    Metric::CpuPercent => j.cpu_percent as u64,
                    Metric::MaxRss => j.max_rss_kb as u64,
                    _ => 0,
                };
                let render_val = if orig_val == 0 { 1 } else { orig_val };
                let text_val = format!("{}", orig_val);
                let style = if orig_val == 0 {
                    Style::default().fg(SURFACE1)
                } else {
                    Style::default().fg(bar_color)
                };

                let (hhmm, mmdd, date) = parse_time(&j.started_at);
                let is_new_day = date != last_date;
                
                let group_width = group_size as u16 * b_width + (group_size as u16).saturating_sub(1) * b_gap;
                let group_x = chart_area.x + i as u16 * (group_width + g_gap);
                let label_x = group_x + (group_width.saturating_sub(5) / 2);

                if label_x < chart_area.right() {
                    f.render_widget(
                        Paragraph::new(hhmm).style(Style::default().fg(YELLOW)),
                        Rect::new(label_x, labels_area.y, 5.min(chart_area.right().saturating_sub(label_x)), 1)
                    );
                    if is_new_day {
                        f.render_widget(
                            Paragraph::new(mmdd).style(Style::default().fg(YELLOW)),
                            Rect::new(label_x, labels_area.y + 1, 5.min(chart_area.right().saturating_sub(label_x)), 1)
                        );
                    }
                }
                last_date = date;

                let group = BarGroup::default()
                    .bars(&[Bar::default()
                        .value(render_val)
                        .text_value(text_val)
                        .style(style)]);
                barchart = barchart.data(group);
            }
        }
        f.render_widget(barchart, chart_area);
    }

    if app.is_loading {
        let area = centered_rect(30, 10, area);
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
