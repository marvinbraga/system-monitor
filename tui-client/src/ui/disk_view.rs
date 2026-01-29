use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use shared::types::DiskMetrics;

/// Render disk metrics view
pub fn render_disk_view(frame: &mut Frame, area: Rect, disks: &[DiskMetrics]) {
    let block = Block::default()
        .title(" Disks ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if disks.is_empty() {
        let text = vec![Line::from(Span::styled(
            "No disks detected",
            Style::default().fg(Color::Gray),
        ))];
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, inner_area);
        return;
    }

    // Create constraints for each disk (3 lines per disk: name, gauge, I/O)
    let max_disks_to_show = (inner_area.height / 3).min(disks.len() as u16) as usize;
    let mut constraints = vec![];

    for _ in 0..max_disks_to_show {
        constraints.push(Constraint::Length(3));
    }

    if constraints.is_empty() {
        return;
    }

    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    for (i, disk) in disks.iter().take(max_disks_to_show).enumerate() {
        render_single_disk(frame, chunks[i], disk);
    }
}

fn render_single_disk(frame: &mut Frame, area: Rect, disk: &DiskMetrics) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Disk name and mount point
            Constraint::Length(1), // Usage gauge
            Constraint::Length(1), // I/O stats
        ])
        .split(area);

    // Disk name and mount point
    let name_text = vec![Line::from(vec![
        Span::styled(&disk.name, Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(
            format!("({})", disk.mount_point),
            Style::default().fg(Color::Gray),
        ),
    ])];
    let name_paragraph = Paragraph::new(name_text);
    frame.render_widget(name_paragraph, chunks[0]);

    // Usage gauge
    let used_gb = disk.used as f64 / (1024.0 * 1024.0 * 1024.0);
    let total_gb = disk.total as f64 / (1024.0 * 1024.0 * 1024.0);

    let color = get_usage_color(disk.usage_percent);
    let label = format!(
        "{:.1} GB / {:.1} GB ({:.1}%)",
        used_gb, total_gb, disk.usage_percent
    );

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::Black))
        .ratio((disk.usage_percent / 100.0).min(1.0) as f64)
        .label(label);

    frame.render_widget(gauge, chunks[1]);

    // I/O stats
    let io_text = vec![Line::from(vec![
        Span::styled("I/O: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("R: {:.2} MB", disk.read_mb),
            Style::default().fg(Color::Green),
        ),
        Span::raw("  "),
        Span::styled(
            format!("W: {:.2} MB", disk.write_mb),
            Style::default().fg(Color::Blue),
        ),
    ])];

    let io_paragraph = Paragraph::new(io_text);
    frame.render_widget(io_paragraph, chunks[2]);
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
