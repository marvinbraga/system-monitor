use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use shared::types::CpuMetrics;

/// Render CPU metrics view
pub fn render_cpu_view(frame: &mut Frame, area: Rect, cpu: &CpuMetrics) {
    let block = Block::default()
        .title(" CPU ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Calculate layout: global gauge + per-core bars + load averages
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Global usage
            Constraint::Min(1),    // Per-core bars
            Constraint::Length(3), // Load averages
        ])
        .split(inner_area);

    // Render global CPU usage
    render_global_usage(frame, chunks[0], cpu.global_usage);

    // Render per-core usage (show first 8 cores if more than 8)
    render_per_core_usage(frame, chunks[1], &cpu.per_core);

    // Render load averages
    render_load_averages(frame, chunks[2], cpu);
}

fn render_global_usage(frame: &mut Frame, area: Rect, usage: f32) {
    let color = get_usage_color(usage);
    let label = format!("Global: {:.1}%", usage);

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::Black))
        .ratio((usage / 100.0).min(1.0) as f64)
        .label(label);

    frame.render_widget(gauge, area);
}

fn render_per_core_usage(frame: &mut Frame, area: Rect, per_core: &[f32]) {
    let max_cores_to_show = 8;
    let cores_to_show = per_core.len().min(max_cores_to_show);

    if cores_to_show == 0 {
        return;
    }

    let constraints: Vec<Constraint> = vec![Constraint::Length(1); cores_to_show];

    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, usage) in per_core.iter().take(cores_to_show).enumerate() {
        let color = get_usage_color(*usage);
        let label = format!("Core {}: {:.1}%", i, usage);

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(color).bg(Color::Black))
            .ratio((*usage / 100.0).min(1.0) as f64)
            .label(label);

        frame.render_widget(gauge, chunks[i]);
    }

    // Show message if there are more cores
    if per_core.len() > max_cores_to_show {
        // This would require additional layout space
    }
}

fn render_load_averages(frame: &mut Frame, area: Rect, cpu: &CpuMetrics) {
    let text = vec![Line::from(vec![
        Span::styled("Load Averages: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("1m: {:.2}", cpu.load_avg_1),
            Style::default().fg(Color::Green),
        ),
        Span::raw("  "),
        Span::styled(
            format!("5m: {:.2}", cpu.load_avg_5),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("  "),
        Span::styled(
            format!("15m: {:.2}", cpu.load_avg_15),
            Style::default().fg(Color::Red),
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
