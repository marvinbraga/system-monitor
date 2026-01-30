use collector::api::{start_server_with_shutdown, ServerConfig};
use collector::{AnomalyRules, MetricsCollector, MetricsRepository};
use shared::types::SystemMetrics;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, timeout, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("Starting System Monitor Collector");

    // Configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./data/system-monitor.db".to_string());
    let collection_interval = std::env::var("COLLECTION_INTERVAL_SECS")
        .unwrap_or_else(|_| "2".to_string())
        .parse::<u64>()
        .unwrap_or(2);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5253".to_string())
        .parse::<u16>()
        .unwrap_or(5253);

    tracing::info!("Database: {}", database_url);
    tracing::info!("Collection interval: {}s", collection_interval);
    tracing::info!("API server: {}:{}", host, port);

    // Initialize database
    let repo = Arc::new(MetricsRepository::new(&database_url).await?);
    tracing::info!("Database initialized");

    // Run migrations
    repo.run_migrations().await?;
    tracing::info!("Database migrations completed");

    // Create shutdown token for coordinated shutdown
    let shutdown_token = CancellationToken::new();

    // Shared state
    let current_metrics: Arc<RwLock<Option<SystemMetrics>>> = Arc::new(RwLock::new(None));
    let recent_anomalies = Arc::new(RwLock::new(Vec::new()));

    // Start API server
    let server_config = ServerConfig {
        host: host.clone(),
        port,
        enable_cors: true,
        cors_origins: vec!["http://localhost:5252".to_string()],
    };

    let api_metrics = Arc::clone(&current_metrics);
    let api_anomalies = Arc::clone(&recent_anomalies);
    let api_repo = Arc::clone(&repo);
    let api_shutdown_token = shutdown_token.clone();

    let api_handle = tokio::spawn(async move {
        if let Err(e) = start_server_with_shutdown(
            server_config,
            api_metrics,
            api_anomalies,
            api_repo,
            api_shutdown_token,
        )
        .await
        {
            tracing::error!("API server error: {}", e);
        }
    });

    // Start collection loop
    let collection_shutdown_token = shutdown_token.clone();
    let collection_repo = Arc::clone(&repo);
    let collection_handle = tokio::spawn(async move {
        run_collection_loop(
            collection_interval,
            collection_repo,
            current_metrics,
            recent_anomalies,
            collection_shutdown_token,
        )
        .await
    });

    // Wait for shutdown signal and coordinate shutdown
    shutdown_signal(shutdown_token.clone()).await;

    tracing::info!("Waiting for tasks to complete (15 second timeout)...");

    // Wait for both tasks to complete with a timeout
    let shutdown_timeout = Duration::from_secs(15);

    let api_result = timeout(shutdown_timeout, api_handle).await;
    match api_result {
        Ok(Ok(())) => tracing::info!("API server task completed"),
        Ok(Err(e)) => tracing::error!("API server task panicked: {}", e),
        Err(_) => tracing::warn!("API server task timed out during shutdown"),
    }

    let collection_result = timeout(shutdown_timeout, collection_handle).await;
    match collection_result {
        Ok(Ok(Ok(()))) => tracing::info!("Collection loop task completed"),
        Ok(Ok(Err(e))) => tracing::error!("Collection loop error: {}", e),
        Ok(Err(e)) => tracing::error!("Collection loop task panicked: {}", e),
        Err(_) => tracing::warn!("Collection loop task timed out during shutdown"),
    }

    // Close database connection
    tracing::info!("Closing database connection...");
    if let Err(e) = repo.close().await {
        tracing::warn!("Error closing database connection: {}", e);
    }

    tracing::info!("Shutdown complete!");

    Ok(())
}

/// Runs the metrics collection loop
///
/// This function collects system metrics at regular intervals, detects anomalies,
/// and stores the data in the database. It responds to the shutdown token for
/// graceful termination.
async fn run_collection_loop(
    collection_interval: u64,
    repo: Arc<MetricsRepository>,
    current_metrics: Arc<RwLock<Option<SystemMetrics>>>,
    recent_anomalies: Arc<RwLock<Vec<shared::types::Anomaly>>>,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    let mut collector = MetricsCollector::new()?;
    let mut detector = AnomalyRules::new(num_cpus::get());
    let mut tick = interval(Duration::from_secs(collection_interval));

    tracing::info!("Starting collection loop");

    loop {
        tokio::select! {
            _ = shutdown_token.cancelled() => {
                tracing::info!("Collection loop received shutdown signal");
                break;
            }
            _ = tick.tick() => {
                // Collect metrics
                let metrics = match collector.collect_all() {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::error!("Failed to collect metrics: {}", e);
                        continue;
                    }
                };

                // Detect anomalies
                let anomalies = detector.check(&metrics);

                // Store in database
                if let Err(e) = repo.store_metrics(&metrics).await {
                    tracing::error!("Failed to store metrics: {}", e);
                }

                for anomaly in &anomalies {
                    if let Err(e) = repo.store_anomaly(anomaly).await {
                        tracing::error!("Failed to store anomaly: {}", e);
                    } else {
                        tracing::warn!("[{:?}] {}", anomaly.severity, anomaly.message);
                    }
                }

                // Update shared state for API
                *current_metrics.write().await = Some(metrics);

                // Keep last 100 anomalies in memory
                let mut anomalies_write = recent_anomalies.write().await;
                anomalies_write.extend(anomalies);
                if anomalies_write.len() > 100 {
                    let len = anomalies_write.len();
                    anomalies_write.drain(0..len - 100);
                }

                tracing::debug!("Metrics collected and stored");
            }
        }
    }

    tracing::info!("Collection loop stopped");
    Ok(())
}

/// Waits for shutdown signal (SIGINT or SIGTERM) and cancels the token
async fn shutdown_signal(shutdown_token: CancellationToken) {
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
            tracing::info!("Received SIGINT (Ctrl+C), initiating graceful shutdown...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, initiating graceful shutdown...");
        },
    }

    // Cancel all tasks
    shutdown_token.cancel();
}
