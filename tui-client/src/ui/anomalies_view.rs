use chrono::Local;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use shared::types::{Anomaly, AnomalySeverity};

/// State for the anomalies view (for scrolling)
pub struct AnomaliesViewState {
    pub list_state: ListState,
}

impl Default for AnomaliesViewState {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { list_state }
    }
}

impl AnomaliesViewState {
    pub fn scroll_up(&mut self, anomalies_count: usize) {
        if anomalies_count == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    anomalies_count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn scroll_down(&mut self, anomalies_count: usize) {
        if anomalies_count == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= anomalies_count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn scroll_page_up(&mut self, anomalies_count: usize, page_size: usize) {
        if anomalies_count == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => i.saturating_sub(page_size),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn scroll_page_down(&mut self, anomalies_count: usize, page_size: usize) {
        if anomalies_count == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => (i + page_size).min(anomalies_count - 1),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn scroll_to_newest(&mut self, anomalies_count: usize) {
        if anomalies_count > 0 {
            self.list_state.select(Some(anomalies_count - 1));
        }
    }
}

/// Render anomalies view
pub fn render_anomalies_view(
    frame: &mut Frame,
    area: Rect,
    anomalies: &[Anomaly],
    state: &mut AnomaliesViewState,
) {
    let block = Block::default()
        .title(format!(" Anomalies ({}) ", anomalies.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    if anomalies.is_empty() {
        let list = List::new(vec![ListItem::new(Line::from(Span::styled(
            "No anomalies detected",
            Style::default().fg(Color::Gray),
        )))])
        .block(block);

        frame.render_widget(list, area);
        return;
    }

    // Create list items from anomalies (newest first)
    let items: Vec<ListItem> = anomalies
        .iter()
        .rev() // Reverse to show newest first
        .map(|anomaly| {
            let severity_color = get_severity_color(&anomaly.severity);
            let severity_symbol = get_severity_symbol(&anomaly.severity);
            let category = format!("{:?}", anomaly.category);

            // Format timestamp to local time
            let local_time = anomaly.timestamp.with_timezone(&Local);
            let time_str = local_time.format("%H:%M:%S").to_string();

            let line = Line::from(vec![
                Span::styled(
                    severity_symbol,
                    Style::default()
                        .fg(severity_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(time_str, Style::default().fg(Color::Gray)),
                Span::raw(" "),
                Span::styled(
                    format!("[{}]", category),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(" "),
                Span::styled(&anomaly.message, Style::default().fg(Color::White)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state.list_state);
}

fn get_severity_color(severity: &AnomalySeverity) -> Color {
    match severity {
        AnomalySeverity::Info => Color::Blue,
        AnomalySeverity::Warning => Color::Yellow,
        AnomalySeverity::Critical => Color::Red,
    }
}

fn get_severity_symbol(severity: &AnomalySeverity) -> &'static str {
    match severity {
        AnomalySeverity::Info => "ℹ",
        AnomalySeverity::Warning => "⚠",
        AnomalySeverity::Critical => "✖",
    }
}
