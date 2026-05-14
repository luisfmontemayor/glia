use crate::app::{App, Metric, Pane, TimeWindow};
use crate::components::header::render_header;
use crate::components::tabs::render_tabs;
use crate::components::footer::render_footer;
use crate::components::modal::render_modal;
use crate::components::table::render_top_scripts_table;
use crate::theme::*;
use crate::utils::centered_rect;
use std::collections::HashMap;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, Borders, Cell, Chart, Clear, Dataset, GraphType,
        LegendPosition, Paragraph, Row, Table, Tabs,
    },
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
                .wrap(ratatui::widgets::Wrap { trim: true }),
            graph_split[1],
        );
        f.render_widget(
            Paragraph::new(right_text)
                .style(palette_style)
                .wrap(ratatui::widgets::Wrap { trim: true }),
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

fn render_metric_chart(f: &mut Frame, app: &App, area: Rect) {
    let border_color = if app.focused_pane == Pane::Graph { PINK } else { BLUE };

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
            return ("??:??".to_string(), "????".to_string(), started_at.to_string());
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
                Metric::JobStatus => if j.exit_code_int == 0 { 1.0 } else { 0.0 },
            };

            user_data.entry(j.user_name.clone()).or_default().push((x, val));
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
                    .data(&marker_line)
            );
        }

        let mut x_labels = Vec::new();
        if is_low_density {
            x_labels.push(Span::raw(""));
            for (_, j) in &job_points {
                let (hhmm, _, _) = parse_time(&j.started_at);
                x_labels.push(Span::raw(hhmm));
            }
            x_labels.push(Span::raw(""));
        } else {
            x_labels.push(Span::raw(
                chrono::DateTime::from_timestamp(min_x as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M")
                    .to_string(),
            ));
            x_labels.push(Span::raw(
                chrono::DateTime::from_timestamp(max_x as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M")
                    .to_string(),
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

        f.render_widget(chart, area);
    } else {
        let is_wmax = app.window == TimeWindow::WMax;
        let n_jobs = app.jobs.len();
        let is_low_density = n_jobs > 0 && n_jobs < app.data_point_threshold;
        let group_size = if app.metric == Metric::JobStatus { 2 } else { 1 };

        let (main_area, label_area) = if is_wmax && !app.jobs.is_empty() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(area);
            (chunks[0], Some(chunks[1]))
        } else if !is_wmax && !app.jobs.is_empty() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(area);
            (chunks[0], Some(chunks[1]))
        } else {
            (area, None)
        };

        let available_width = main_area.width.saturating_sub(4);
        let (b_width, b_gap, g_gap) = if is_low_density {
            let bg = 1;
            let gg = if group_size > 1 { 2 } else { 1 };
            let overhead = n_jobs as u16 * (group_size as u16 - 1) * bg + n_jobs.saturating_sub(1) as u16 * gg;
            let bw = (available_width.saturating_sub(overhead) / (n_jobs as u16 * group_size as u16)).max(1).min(20);
            (bw, bg, gg)
        } else if app.metric == Metric::JobStatus {
            (2, 1, 2)
        } else {
            (5, 1, 1)
        };

        let group_width = group_size as u16 * b_width + (group_size as u16 - 1) * b_gap;
        let total_content_width = (n_jobs as u16 * group_width) + (n_jobs.saturating_sub(1) as u16 * g_gap);
        let side_padding = available_width.saturating_sub(total_content_width) / 2;

        let chart_block = Block::default()
            .borders(Borders::ALL)
            .title(chart_title)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().fg(TEXT))
            .padding(ratatui::widgets::Padding {
                left: 1 + side_padding,
                right: 1 + side_padding,
                top: 0,
                bottom: 0,
            });

        let inner_area = chart_block.inner(main_area);
        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(inner_area);
        
        f.render_widget(chart_block, main_area);

        let mut barchart = BarChart::default()
            .bar_set(symbols::bar::NINE_LEVELS)
            .value_style(Style::default().fg(CRUST).bg(TEXT))
            .bar_width(b_width)
            .bar_gap(b_gap)
            .group_gap(g_gap);

        let mut labels_hhmm = Vec::new();
        let mut labels_mmdd = Vec::new();
        let mut labels_normal = Vec::new();
        let mut markers = Vec::new();
        let mut last_date = String::new();

        if app.metric == Metric::JobStatus {
            barchart = barchart.max(8);
            for (i, j) in app.jobs.iter().enumerate() {
                let s_render_val = if j.exit_code_int == 0 { 8 } else { 1 };
                let s_text = if j.exit_code_int == 0 { "1".to_string() } else { "0".to_string() };
                let s_style = if j.exit_code_int == 0 { Style::default().fg(GREEN) } else { Style::default().fg(SURFACE1) };

                let f_render_val = if j.exit_code_int != 0 { 8 } else { 1 };
                let f_text = if j.exit_code_int != 0 { "1".to_string() } else { "0".to_string() };
                let f_style = if j.exit_code_int != 0 { Style::default().fg(RED) } else { Style::default().fg(SURFACE1) };

                let (hhmm, mmdd, date) = parse_time(&j.started_at);
                
                if is_wmax {
                    let is_new_day = !last_date.is_empty() && date != last_date;
                    let hhmm_style = if is_new_day { Style::default().fg(PINK) } else { Style::default().fg(TEXT) };
                    let mut text = hhmm;
                    if is_new_day { text = format!("|{}", text); }
                    labels_hhmm.push(Span::styled(format!("{:^width$}", text, width = group_width as usize), hhmm_style));
                    if is_new_day || labels_mmdd.is_empty() {
                        labels_mmdd.push(Span::styled(format!("{:^width$}", mmdd, width = group_width as usize), Style::default().fg(YELLOW)));
                    } else {
                        labels_mmdd.push(Span::raw(" ".repeat(group_width as usize)));
                    }
                    if i < n_jobs - 1 {
                        labels_hhmm.push(Span::raw(" ".repeat(g_gap as usize)));
                        labels_mmdd.push(Span::raw(" ".repeat(g_gap as usize)));
                    }
                    last_date = date;
                } else {
                    let label = if app.window == TimeWindow::WMax { format!("{} {}", mmdd, hhmm) } else { hhmm };
                    labels_normal.push(Span::raw(format!("{:^width$}", label, width = group_width as usize)));
                    if i < n_jobs - 1 {
                        labels_normal.push(Span::raw(" ".repeat(g_gap as usize)));
                    }
                }

                // Marker
                let center = group_width / 2;
                markers.push(Span::raw(" ".repeat(center as usize)));
                markers.push(Span::styled("│", Style::default().fg(SURFACE2)));
                markers.push(Span::raw(" ".repeat((group_width - center - 1) as usize)));
                if i < n_jobs - 1 {
                    markers.push(Span::raw(" ".repeat(g_gap as usize)));
                }

                let group = BarGroup::default().bars(&[
                    Bar::default().value(s_render_val).text_value(s_text).style(s_style),
                    Bar::default().value(f_render_val).text_value(f_text).style(f_style),
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

            let mut max_val = 0;
            for j in &app.jobs {
                let v = match app.metric {
                    Metric::WallTime => j.wall_time_ms as u64,
                    Metric::CpuTime => (j.cpu_time_sec * 1000.0) as u64,
                    Metric::CpuPercent => j.cpu_percent as u64,
                    Metric::MaxRss => j.max_rss_kb as u64,
                    _ => 0,
                };
                if v > max_val { max_val = v; }
            }

            barchart = barchart.bar_style(Style::default().fg(bar_color)).max(max_val.max(8));

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
                let style = if orig_val == 0 { Style::default().fg(SURFACE1) } else { Style::default().fg(bar_color) };

                let (hhmm, mmdd, date) = parse_time(&j.started_at);
                
                if is_wmax {
                    let is_new_day = !last_date.is_empty() && date != last_date;
                    let hhmm_style = if is_new_day { Style::default().fg(PINK) } else { Style::default().fg(TEXT) };
                    let mut text = hhmm;
                    if is_new_day { text = format!("|{}", text); }
                    labels_hhmm.push(Span::styled(format!("{:^width$}", text, width = group_width as usize), hhmm_style));
                    if is_new_day || labels_mmdd.is_empty() {
                        labels_mmdd.push(Span::styled(format!("{:^width$}", mmdd, width = group_width as usize), Style::default().fg(YELLOW)));
                    } else {
                        labels_mmdd.push(Span::raw(" ".repeat(group_width as usize)));
                    }
                    if i < n_jobs - 1 {
                        labels_hhmm.push(Span::raw(" ".repeat(g_gap as usize)));
                        labels_mmdd.push(Span::raw(" ".repeat(g_gap as usize)));
                    }
                    last_date = date;
                } else {
                    let label = if app.window == TimeWindow::WMax { format!("{} {}", mmdd, hhmm) } else { hhmm };
                    labels_normal.push(Span::raw(format!("{:^width$}", label, width = group_width as usize)));
                    if i < n_jobs - 1 {
                        labels_normal.push(Span::raw(" ".repeat(g_gap as usize)));
                    }
                }

                // Marker
                let center = group_width / 2;
                markers.push(Span::raw(" ".repeat(center as usize)));
                markers.push(Span::styled("│", Style::default().fg(SURFACE2)));
                markers.push(Span::raw(" ".repeat((group_width - center - 1) as usize)));
                if i < n_jobs - 1 {
                    markers.push(Span::raw(" ".repeat(g_gap as usize)));
                }

                let group = BarGroup::default().bars(&[Bar::default().value(render_val).text_value(text_val).style(style)]);
                barchart = barchart.data(group);
            }
        }
        f.render_widget(barchart, inner_chunks[0]);
        f.render_widget(Paragraph::new(Line::from(markers)), inner_chunks[1]);

        if let Some(la) = label_area {
            let max_width = la.width.saturating_sub(2 + side_padding);
            let inner_la = Rect {
                x: la.x + 2 + side_padding,
                y: la.y,
                width: total_content_width.min(max_width),
                height: la.height,
            };
            if is_wmax {
                f.render_widget(Paragraph::new(vec![Line::from(labels_hhmm), Line::from(labels_mmdd)]), inner_la);
            } else {
                f.render_widget(Paragraph::new(Line::from(labels_normal)), inner_la);
            }
        }
    }

    if app.is_loading {
        let area = centered_rect(30, 10, area);
        f.render_widget(Clear, area);
        f.render_widget(
            Paragraph::new("Loading...")
                .style(Style::default().fg(YELLOW).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
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
