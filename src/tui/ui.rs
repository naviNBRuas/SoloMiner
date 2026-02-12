use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Gauge, LineGauge, Chart, Dataset, GraphType},
    symbols,
    Frame,
};
use crate::tui::app::{App, CurrentScreen};

pub fn render(f: &mut Frame, app: &mut App, metrics: &crate::telemetry::MetricsSnapshot) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Enhanced header with animated title
    let title_text = if app.animation_frame % 20 < 10 {
        "⛏️  LONELY SOLO MINER ⛏️"
    } else {
        "💎 LONELY SOLO MINER 💎"
    };
    
    let titles = vec![
        "[1] Dashboard".to_string(),
        "[2] Miners".to_string(),
        "[3] Logs".to_string(),
        "[4] Settings".to_string(),
        "[q] Quit".to_string(),
    ];
    
    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title_text)
            .border_style(Style::default().fg(Color::Rgb(255, 215, 0))))
        .select(match app.current_screen {
            CurrentScreen::Main => 0,
            CurrentScreen::Miners => 1,
            CurrentScreen::Logs => 2,
            CurrentScreen::Settings => 3,
        })
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[0]);

    match app.current_screen {
        CurrentScreen::Main => render_main(f, chunks[1], app, metrics),
        CurrentScreen::Miners => render_miners(f, chunks[1], app, metrics),
        CurrentScreen::Logs => render_logs(f, chunks[1], app),
        CurrentScreen::Settings => render_settings(f, chunks[1], app, metrics),
    }

    // Enhanced footer with more information
    let footer_text = format!(
        "⚡ Status: {} | 🔢 Hashes: {} | 🎯 Blocks: {} | ⏱️  Uptime: {} | [r] Reset Stats",
        metrics.status,
        format_number(metrics.total_hashes),
        metrics.blocks_found,
        app.formatted_uptime()
    );
    
    let footer = Paragraph::new(footer_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(footer, chunks[2]);
    
    // Render particles on top of everything
    render_particles(f, app);
}

fn render_main(f: &mut Frame, area: Rect, app: &App, metrics: &crate::telemetry::MetricsSnapshot) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Min(1)
        ])
        .split(chunks[0]);

    // Enhanced hashrate gauge with dynamic coloring
    let hashrate_ratio = ((metrics.hashrate as f64) / 500000.0).min(1.0);
    let hashrate_color = if hashrate_ratio > 0.8 { Color::Green }
                        else if hashrate_ratio > 0.5 { Color::Yellow }
                        else { Color::Red };
    
    let hashrate_gauge = LineGauge::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" ⚡ Hashrate: {} H/s ", format_number(metrics.hashrate)))
            .border_style(Style::default().fg(hashrate_color)))
        .gauge_style(Style::default().fg(hashrate_color))
        .line_set(symbols::line::DOUBLE)
        .ratio(hashrate_ratio);
    f.render_widget(hashrate_gauge, left_chunks[0]);

    // Enhanced CPU gauge
    let cpu_usage = app.system.global_cpu_info().cpu_usage() as u16;
    let cpu_color = if cpu_usage > 80 { Color::Red }
                   else if cpu_usage > 60 { Color::Yellow }
                   else { Color::Green };
    
    let cpu_gauge = Gauge::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" 🖥️  CPU Usage: {}% ", cpu_usage))
            .border_style(Style::default().fg(cpu_color)))
        .gauge_style(Style::default().fg(cpu_color))
        .percent(cpu_usage);
    f.render_widget(cpu_gauge, left_chunks[1]);

    // Enhanced loneliness meter with dynamic text
    let loneliness_level = 99u16;
    let loneliness_text = match loneliness_level {
        95..=100 => "Extremely Lonely 😢",
        80..=94 => "Very Lonely 🥺",
        60..=79 => "Moderately Lonely 😔",
        _ => "Slightly Lonely 😕",
    };
    
    let loneliness_gauge = Gauge::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" 💔 {}: {}% ", loneliness_text, loneliness_level))
            .border_style(Style::default().fg(Color::Magenta)))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(loneliness_level);
    f.render_widget(loneliness_gauge, left_chunks[2]);

    // Enhanced hashrate chart
    let data: Vec<(f64, f64)> = app.hashrate_history
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as f64, val))
        .collect();
    
    let datasets = vec![Dataset::default()
        .name("Hashrate")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data)];
    
    let chart = Chart::new(datasets)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📈 Hashrate Trend ")
            .border_style(Style::default().fg(Color::Blue)))
        .x_axis(ratatui::widgets::Axis::default()
            .bounds([0.0, 100.0])
            .labels(vec!["0".into(), "50".into(), "100".into()]))
        .y_axis(ratatui::widgets::Axis::default()
            .bounds([0.0, app.hashrate_history.iter().fold(0.0f64, |a, &b| a.max(b)) + 1000.0])
            .labels(vec!["0".into(), "Max".into()]));
    f.render_widget(chart, left_chunks[3]);

    // Enhanced info panel
    let mem_info = app.system.total_memory();
    let used_mem = app.system.used_memory();
    let mem_percent = ((used_mem as f64 / mem_info as f64) * 100.0) as u64;
    
    let info_text = vec![
        Line::from(vec![
            Span::raw("💼 Wallet: "),
            Span::styled("L0N3LY-W4LL3T-4DDR355", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("🎯 Difficulty: "),
            Span::styled("0000", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("⏱️  Uptime: "),
            Span::styled(app.formatted_uptime(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("💾 Memory: "),
            Span::styled(format!("{}% ({}/{}) MB", mem_percent, used_mem/1024/1024, mem_info/1024/1024), 
                        Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("🌡️  Temperature: "),
            Span::styled("🔥 Getting Hot", Style::default().fg(Color::Red)),
        ]),
    ];
    
    let info = Paragraph::new(info_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📋 Miner Information ")
            .border_style(Style::default().fg(Color::Green)));
    f.render_widget(info, chunks[1]);
}

fn render_miners(f: &mut Frame, area: Rect, app: &App, metrics: &crate::telemetry::MetricsSnapshot) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);
    
    // Mining instances table
    let instances = vec![
        ("🖥️  CPU #1 (SHA-256)", "🟢 Active", format!("{} H/s", metrics.hashrate / 4)),
        ("🖥️  CPU #2 (SHA-256)", "🟢 Active", format!("{} H/s", metrics.hashrate / 4)),
        ("🖥️  CPU #3 (RandomX)", "🟢 Active", format!("{} H/s", metrics.hashrate / 8)),
        ("🖥️  CPU #4 (RandomX)", "🟢 Active", format!("{} H/s", metrics.hashrate / 8)),
        ("🎮 GPU #1 (SHA-256)", "🟢 Active", format!("{} H/s", metrics.hashrate / 4)),
        ("🎮 GPU #2 (RandomX)", "🟢 Active", format!("{} H/s", metrics.hashrate / 8)),
    ];
    
    let mut instance_lines = vec![
        Line::from(vec![
            Span::styled("Instance", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("     "),
            Span::styled("Status", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("     "),
            Span::styled("Hashrate", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("─".repeat(50)),
    ];
    
    for (name, status, hashrate) in instances {
        let status_color = if status.contains("Active") { Color::Green } else { Color::Red };
        instance_lines.push(Line::from(vec![
            Span::styled(name, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled(status, Style::default().fg(status_color)),
            Span::raw("  "),
            Span::styled(hashrate, Style::default().fg(Color::Yellow)),
        ]));
    }
    
    let miners_table = Paragraph::new(instance_lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🏗️  Mining Instances ")
            .border_style(Style::default().fg(Color::Blue)));
    f.render_widget(miners_table, chunks[0]);
    
    // Performance metrics
    let perf_metrics = vec![
        Line::from(vec![
            Span::raw("Total Cores: "),
            Span::styled(format!("{}", app.system.cpus().len()), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Active Threads: "),
            Span::styled("6", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Avg Efficiency: "),
            Span::styled("92%", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Power Consumption: "),
            Span::styled("🔥 250W", Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("Estimated ROI: "),
            Span::styled("💸 Never", Style::default().fg(Color::Magenta)),
        ]),
    ];
    
    let perf_paragraph = Paragraph::new(perf_metrics)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📊 Performance Metrics ")
            .border_style(Style::default().fg(Color::Green)));
    f.render_widget(perf_paragraph, chunks[1]);
}

fn render_logs(f: &mut Frame, area: Rect, app: &App) {
    let log_entries = vec![
        format!("[{}] [INFO] Lonely Solo Miner v1.0.0 starting up...", get_timestamp()),
        format!("[{}] [INFO] Detected {} CPU cores", get_timestamp(), app.system.cpus().len()),
        format!("[{}] [INFO] Initializing SHA-256 miners (CPU × 4)", get_timestamp()),
        format!("[{}] [INFO] Initializing RandomX miners (CPU × 2)", get_timestamp()),
        format!("[{}] [INFO] Initializing GPU miners (Simulated × 2)", get_timestamp()),
        format!("[{}] [INFO] All mining instances active and lonely", get_timestamp()),
        format!("[{}] [DEBUG] Current hashrate: {} H/s", get_timestamp(), *app.hashrate_history.last().unwrap_or(&0.0) as u64),
        format!("[{}] [WARN] Electricity bill approaching danger levels", get_timestamp()),
        format!("[{}] [INFO] Still no blocks found... maintaining hope", get_timestamp()),
        format!("[{}] [DEBUG] Loneliness level: Maximum", get_timestamp()),
    ];
    
    let log_text: Vec<Line> = log_entries
        .iter()
        .map(|entry| {
            let parts: Vec<&str> = entry.split(']').collect();
            if parts.len() >= 2 {
                let level = if entry.contains("[ERROR]") { Color::Red }
                           else if entry.contains("[WARN]") { Color::Yellow }
                           else if entry.contains("[DEBUG]") { Color::Blue }
                           else { Color::Green };
        {
                    let first_part = parts[0].to_string();
                    let rest_parts = parts[1..].join("]");
                    Line::from(vec![
                        Span::styled(first_part, Style::default().fg(level)),
                        Span::styled("]", Style::default().fg(Color::Gray)),
                        Span::styled(rest_parts, Style::default().fg(Color::Gray)),
                    ])
                }
            } else {
                Line::from(Span::styled(entry, Style::default().fg(Color::Gray)))
            }
        })
        .collect();
    
    let logs = Paragraph::new(log_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📜 Mining Log ")
            .border_style(Style::default().fg(Color::Cyan)))
        .scroll((0, 0));
    f.render_widget(logs, area);
}

// Helper functions
fn format_number(num: u64) -> String {
    if num >= 1_000_000 {
        format!("{:.2}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}

fn get_timestamp() -> String {
    use chrono::Local;
    Local::now().format("%H:%M:%S").to_string()
}

fn render_particles(f: &mut Frame, app: &App) {
    for particle in &app.particles {
        if particle.x >= 0.0 && particle.x < f.size().width as f64 &&
           particle.y >= 0.0 && particle.y < f.size().height as f64 {
            let intensity = (particle.life * 255.0) as u8;
            let color = Color::Rgb(intensity, intensity, 255);
            
            let span = Span::styled(
                particle.char.to_string(),
                Style::default().fg(color)
            );
            
            f.render_widget(
                Paragraph::new(span),
                Rect::new(particle.x as u16, particle.y as u16, 1, 1)
            );
        }
    }
}

fn render_settings(f: &mut Frame, area: Rect, _app: &App, _metrics: &crate::telemetry::MetricsSnapshot) {
    let settings = vec![
        Line::from(vec![
            Span::styled("⚙️  Settings Menu", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("[d] Set Difficulty: "),
            Span::styled("0000", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("[m] Mining Mode: "),
            Span::styled("Performance", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("[w] Wallet Address: "),
            Span::styled("L0N3LY-W4LL3T-4DDR355", Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::raw("[t] Theme: "),
            Span::styled("Dark (Default)", Style::default().fg(Color::Blue)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press corresponding key to modify settings", 
                        Style::default().fg(Color::Gray)),
        ]),
    ];
    
    let settings_paragraph = Paragraph::new(settings)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚙️  Settings ")
            .border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(settings_paragraph, area);
}

