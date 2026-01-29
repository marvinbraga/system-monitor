use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use shared::types::MemoryMetrics;

/// Render memory metrics view
pub fn render_memory_view(frame: &mut Frame, area: Rect, memory: &MemoryMetrics) {
    let block = Block::default()
        .title(" Memory ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Layout: RAM gauge + SWAP gauge + details
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(2), // RAM gauge
            Constraint::Length(2), // SWAP gauge
            Constraint::Min(1),    // Details
        ])
        .split(inner_area);

    // Render RAM usage
    render_ram_usage(frame, chunks[0], memory);

    // Render SWAP usage
    render_swap_usage(frame, chunks[1], memory);

    // Render details
    render_memory_details(frame, chunks[2], memory);
}

fn render_ram_usage(frame: &mut Frame, area: Rect, memory: &MemoryMetrics) {
    let used_gb = memory.used as f64 / (1024.0 * 1024.0 * 1024.0);
    let total_gb = memory.total as f64 / (1024.0 * 1024.0 * 1024.0);

    let color = get_usage_color(memory.usage_percent);
    let label = format!(
        "RAM: {:.2} GB / {:.2} GB ({:.1}%)",
        used_gb, total_gb, memory.usage_percent
    );

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::Black))
        .ratio((memory.usage_percent / 100.0).min(1.0) as f64)
        .label(label);

    frame.render_widget(gauge, area);
}

fn render_swap_usage(frame: &mut Frame, area: Rect, memory: &MemoryMetrics) {
    let used_gb = memory.swap_used as f64 / (1024.0 * 1024.0 * 1024.0);
    let total_gb = memory.swap_total as f64 / (1024.0 * 1024.0 * 1024.0);

    let usage_percent = if memory.swap_total > 0 {
        (memory.swap_used as f64 / memory.swap_total as f64) * 100.0
    } else {
        0.0
    };

    let color = get_usage_color(usage_percent as f32);
    let label = if memory.swap_total > 0 {
        format!(
            "SWAP: {:.2} GB / {:.2} GB ({:.1}%)",
            used_gb, total_gb, usage_percent
        )
    } else {
        "SWAP: Not available".to_string()
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::Black))
        .ratio((usage_percent / 100.0).min(1.0))
        .label(label);

    frame.render_widget(gauge, area);
}

fn render_memory_details(frame: &mut Frame, area: Rect, memory: &MemoryMetrics) {
    let available_gb = memory.available as f64 / (1024.0 * 1024.0 * 1024.0);

    let text = vec![Line::from(vec![
        Span::styled("Available: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{:.2} GB", available_gb),
            Style::default().fg(Color::Green),
        ),
    ])];

    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, area);
}

/// Get color based on usage percentage
fn get_usage_color(usage: f32) -> Color {
    if usage < 50.0 {
        Color::Green
    } else if usage < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}
