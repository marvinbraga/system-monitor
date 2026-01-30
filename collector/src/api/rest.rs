/// REST API server startup and configuration
///
/// This module handles server initialization, configuration, and graceful shutdown.
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::{header, Method};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};

use shared::types::{Anomaly, SystemMetrics};

use super::{routes, AppState};
use crate::storage::MetricsRepository;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host address to bind to (e.g., "0.0.0.0" or "127.0.0.1")
    pub host: String,

    /// Port number to listen on
    pub port: u16,

    /// Enable CORS (Cross-Origin Resource Sharing)
    pub enable_cors: bool,

    /// Allowed CORS origins (if CORS is enabled)
    pub cors_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            cors_origins: vec![
                "http://localhost:3000".to_string(),
                "http://127.0.0.1:3000".to_string(),
            ],
        }
    }
}

/// Starts the REST API and WebSocket server
///
/// This function initializes the Axum server with all routes, middleware,
/// and graceful shutdown handling. It creates its own internal shutdown
/// signal handler that listens for SIGINT/SIGTERM.
///
/// For coordinated shutdown with other components, use `start_server_with_shutdown`
/// which accepts an external `CancellationToken`.
///
/// # Arguments
/// * `config` - Server configuration
/// * `current_metrics` - Shared current system metrics
/// * `recent_anomalies` - Shared recent anomalies buffer
/// * `repository` - Database repository for historical data
///
/// # Returns
/// Result indicating success or error with message
pub async fn start_server(
    config: ServerConfig,
    current_metrics: Arc<RwLock<Option<SystemMetrics>>>,
    recent_anomalies: Arc<RwLock<Vec<Anomaly>>>,
    repository: Arc<MetricsRepository>,
) -> anyhow::Result<()> {
    // Create an internal token for backward compatibility
    let shutdown_token = CancellationToken::new();
    let token_clone = shutdown_token.clone();

    // Spawn a task that listens for OS signals and cancels the token
    tokio::spawn(async move {
        shutdown_signal().await;
        token_clone.cancel();
    });

    start_server_with_shutdown(config, current_metrics, recent_anomalies, repository, shutdown_token).await
}

/// Starts the REST API and WebSocket server with an external shutdown token
///
/// This function initializes the Axum server with all routes, middleware,
/// and graceful shutdown handling using an externally provided `CancellationToken`.
/// This allows coordinated shutdown across multiple components.
///
/// # Arguments
/// * `config` - Server configuration
/// * `current_metrics` - Shared current system metrics
/// * `recent_anomalies` - Shared recent anomalies buffer
/// * `repository` - Database repository for historical data
/// * `shutdown_token` - External cancellation token for coordinated shutdown
///
/// # Returns
/// Result indicating success or error with message
pub async fn start_server_with_shutdown(
    config: ServerConfig,
    current_metrics: Arc<RwLock<Option<SystemMetrics>>>,
    recent_anomalies: Arc<RwLock<Vec<Anomaly>>>,
    repository: Arc<MetricsRepository>,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    info!("Starting REST API server...");

    // Create shared application state
    let state = AppState::new(current_metrics, recent_anomalies, repository);

    // Build CORS layer if enabled
    let cors_layer = if config.enable_cors {
        info!("CORS enabled for origins: {:?}", config.cors_origins);

        let mut cors = CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(Duration::from_secs(3600));

        // Add allowed origins
        for origin in &config.cors_origins {
            cors = cors.allow_origin(
                origin
                    .parse::<axum::http::HeaderValue>()
                    .map_err(|e| anyhow::anyhow!("Invalid CORS origin '{}': {}", origin, e))?,
            );
        }

        Some(cors)
    } else {
        info!("CORS disabled");
        None
    };

    // Build tracing/logging layer
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // Create router with routes
    let mut app = routes::create_router(state);

    // Add tracing layer
    app = app.layer(trace_layer);

    // Add CORS layer if enabled
    if let Some(cors) = cors_layer {
        app = app.layer(cors);
    }

    // Parse socket address
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid host:port combination: {}", e))?;

    info!("Server listening on http://{}", addr);
    info!("Health check: http://{}/health", addr);
    info!("WebSocket endpoint: ws://{}/ws", addr);
    info!("API base URL: http://{}/api/v1", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;

    // Start server with graceful shutdown using external token
    // Use cancelled_owned() to get an owned future that satisfies the 'static requirement
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_token.cancelled_owned())
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    info!("Server shut down gracefully");
    Ok(())
}

/// Handles graceful shutdown signal
///
/// Waits for CTRL+C (SIGINT) or SIGTERM signal to initiate shutdown
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received SIGINT (Ctrl+C), initiating graceful shutdown...");
        },
        _ = terminate => {
            info!("Received SIGTERM, initiating graceful shutdown...");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(config.enable_cors);
        assert!(!config.cors_origins.is_empty());
    }

    #[test]
    fn test_socket_addr_parsing() {
        let addr = "127.0.0.1:8080";
        let parsed: Result<SocketAddr, _> = addr.parse();
        assert!(parsed.is_ok());
    }
}
