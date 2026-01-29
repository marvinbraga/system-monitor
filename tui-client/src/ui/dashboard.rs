use chrono::Local;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use shared::types::{Anomaly, SystemMetrics};

use super::{
    anomalies_view::{render_anomalies_view, AnomaliesViewState},
    cpu_view::render_cpu_view,
    disk_view::render_disk_view,
    memory_view::render_memory_view,
};

/// Main dashboard state
#[derive(Default)]
pub struct DashboardState {
    pub anomalies_view_state: AnomaliesViewState,
}

/// Render the main dashboard
pub fn render_dashboard(
    frame: &mut Frame,
    metrics: &Option<SystemMetrics>,
    anomalies: &[Anomaly],
    connection_status: &str,
    state: &mut DashboardState,
) {
    let area = frame.size();
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
        ])
        .split(area);

    // Render header
    render_header(frame, chunks[0], metrics, connection_status);

    // Main content area
    let main_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Left side (CPU, Memory, Disks)
            Constraint::Percentage(40), // Right side (Anomalies, Temps)
        ])
        .split(chunks[1]);

    // Left side layout
    let left_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(12), // CPU
            Constraint::Length(8),  // Memory
            Constraint::Min(10),    // Disks
        ])
        .split(main_chunks[0]);

    // Right side layout
    let right_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Temperatures
            Constraint::Min(10),   // Anomalies
        ])
        .split(main_chunks[1]);

    if let Some(metrics) = metrics {
        // Render CPU view
        render_cpu_view(frame, left_chunks[0], &metrics.cpu);

        // Render Memory view
        render_memory_view(frame, left_chunks[1], &metrics.memory);

        // Render Disks view
        render_disk_view(frame, left_chunks[2], &metrics.disks);

        // Render Temperatures view
        render_temperatures_view(frame, right_chunks[0], metrics);
    } else {
        // Show loading/error state
        render_no_data(frame, main_chunks[0]);
    }

    // Render Anomalies view
    render_anomalies_view(
        frame,
        right_chunks[1],
        anomalies,
        &mut state.anomalies_view_state,
    );
}

fn render_header(
    frame: &mut Frame,
    area: Rect,
    metrics: &Option<SystemMetrics>,
    connection_status: &str,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let (status_color, status_text) = if connection_status == "Connected" {
        (Color::Green, connection_status)
    } else {
        (Color::Red, connection_status)
    };

    let timestamp = if let Some(metrics) = metrics {
        let local_time = metrics.timestamp.with_timezone(&Local);
        local_time.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        "No data".to_string()
    };

    let header_text = vec![Line::from(vec![
        Span::styled(
            " System Monitor TUI ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled("Status: ", Style::default().fg(Color::Gray)),
        Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled("Updated: ", Style::default().fg(Color::Gray)),
        Span::styled(timestamp, Style::default().fg(Color::White)),
        Span::raw("  |  "),
        Span::styled("Press 'q' to quit", Style::default().fg(Color::DarkGray)),
    ])];

    let paragraph = Paragraph::new(header_text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_temperatures_view(frame: &mut Frame, area: Rect, metrics: &SystemMetrics) {
    let block = Block::default()
        .title(" Temperatures ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if metrics.temperatures.is_empty() {
        let text = vec![Line::from(Span::styled(
            "No temperature sensors detected",
            Style::default().fg(Color::Gray),
        ))];
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, inner_area);
        return;
    }

    let max_temps_to_show = inner_area.height as usize;
    let temps: Vec<Line> = metrics
        .temperatures
        .iter()
        .take(max_temps_to_show)
        .map(|temp| {
            let color = get_temperature_color(temp.value);
            Line::from(vec![
                Span::styled(
                    format!("{:20}", temp.label),
                    Style::default().fg(Color::Gray),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{:.1}°C", temp.value),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(temps);
    frame.render_widget(paragraph, inner_area);
}

fn render_no_data(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "No data available",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Check if the server is running",
            Style::default().fg(Color::Gray),
        )),
    ];

    let block = Block::default()
        .title(" System Monitor ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn get_temperature_color(temp: f32) -> Color {
    if temp < 50.0 {
        Color::Green
    } else if temp < 70.0 {
        Color::Yellow
    } else if temp < 85.0 {
        Color::LightRed
    } else {
        Color::Red
    }
}
