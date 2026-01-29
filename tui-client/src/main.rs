mod api_client;
mod config;
mod ui;

use anyhow::{Context, Result};
use api_client::ApiClient;
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use shared::types::{Anomaly, SystemMetrics};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;
use ui::dashboard::{render_dashboard, DashboardState};

/// Application state
struct App {
    /// Current system metrics
    metrics: Option<SystemMetrics>,
    /// List of anomalies
    anomalies: Vec<Anomaly>,
    /// Connection status message
    connection_status: String,
    /// Dashboard UI state
    dashboard_state: DashboardState,
    /// Should quit
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            metrics: None,
            anomalies: Vec::new(),
            connection_status: "Connecting...".to_string(),
            dashboard_state: DashboardState::default(),
            should_quit: false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse configuration
    let config = Config::from_args().context("Failed to parse configuration")?;

    // Setup panic handler to restore terminal
    setup_panic_handler();

    // Initialize terminal
    let mut terminal = setup_terminal().context("Failed to setup terminal")?;

    // Create API client
    let client = ApiClient::new(config.api_url.clone()).context("Failed to create API client")?;

    // Create application state
    let app = Arc::new(Mutex::new(App::default()));

    // Spawn data fetching task
    let app_clone = Arc::clone(&app);
    let client_clone = client;
    let refresh_rate = config.refresh_rate;

    tokio::spawn(async move {
        fetch_data_loop(app_clone, client_clone, refresh_rate).await;
    });

    // Run main event loop
    let result = run_event_loop(&mut terminal, app).await;

    // Restore terminal
    restore_terminal(&mut terminal).context("Failed to restore terminal")?;

    result
}

/// Setup terminal for TUI
fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Failed to create terminal")?;
    Ok(terminal)
}

/// Restore terminal to normal mode
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;
    Ok(())
}

/// Setup panic handler to restore terminal
fn setup_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));
}

/// Main event loop
async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: Arc<Mutex<App>>,
) -> Result<()> {
    loop {
        // Draw UI
        let should_quit = {
            let mut app = app.lock().unwrap();

            // Clone the data we need for rendering
            let metrics = app.metrics.clone();
            let anomalies = app.anomalies.clone();
            let connection_status = app.connection_status.clone();

            terminal
                .draw(|frame| {
                    render_dashboard(
                        frame,
                        &metrics,
                        &anomalies,
                        &connection_status,
                        &mut app.dashboard_state,
                    );
                })
                .context("Failed to draw terminal")?;

            app.should_quit
        };

        if should_quit {
            break;
        }

        // Handle events with timeout
        if event::poll(Duration::from_millis(100)).context("Failed to poll events")? {
            if let Event::Key(key) = event::read().context("Failed to read event")? {
                let mut app = app.lock().unwrap();
                let anomalies_len = app.anomalies.len();

                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.should_quit = true;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.dashboard_state
                            .anomalies_view_state
                            .scroll_up(anomalies_len);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.dashboard_state
                            .anomalies_view_state
                            .scroll_down(anomalies_len);
                    }
                    KeyCode::PageUp => {
                        app.dashboard_state
                            .anomalies_view_state
                            .scroll_page_up(anomalies_len, 10);
                    }
                    KeyCode::PageDown => {
                        app.dashboard_state
                            .anomalies_view_state
                            .scroll_page_down(anomalies_len, 10);
                    }
                    KeyCode::Home => {
                        app.dashboard_state
                            .anomalies_view_state
                            .list_state
                            .select(Some(0));
                    }
                    KeyCode::End => {
                        if anomalies_len > 0 {
                            app.dashboard_state
                                .anomalies_view_state
                                .scroll_to_newest(anomalies_len);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

/// Background task to fetch data from API
async fn fetch_data_loop(app: Arc<Mutex<App>>, client: ApiClient, refresh_rate: Duration) {
    let mut interval = interval(refresh_rate);

    loop {
        interval.tick().await;

        // Fetch current metrics
        match client.get_current_metrics().await {
            Ok(metrics) => {
                let mut app = app.lock().unwrap();
                app.metrics = Some(metrics);
                app.connection_status = "Connected".to_string();
            }
            Err(e) => {
                let mut app = app.lock().unwrap();
                app.connection_status = format!("Error: {}", e);
            }
        }

        // Fetch anomalies (limit to last 100)
        match client.get_anomalies(Some(100)).await {
            Ok(anomalies) => {
                let mut app = app.lock().unwrap();
                app.anomalies = anomalies;
            }
            Err(e) => {
                eprintln!("Failed to fetch anomalies: {}", e);
            }
        }
    }
}
