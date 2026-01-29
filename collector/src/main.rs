use collector::api::{start_server, ServerConfig};
use collector::{AnomalyRules, MetricsCollector, MetricsRepository};
use shared::types::SystemMetrics;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

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

    let _api_handle = tokio::spawn(async move {
        if let Err(e) = start_server(server_config, api_metrics, api_anomalies, api_repo).await {
            tracing::error!("API server error: {}", e);
        }
    });

    // Collection loop
    let mut collector = MetricsCollector::new()?;
    let mut detector = AnomalyRules::new(num_cpus::get());
    let mut tick = interval(Duration::from_secs(collection_interval));

    tracing::info!("Starting collection loop");

    loop {
        tick.tick().await;

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
