/// Example demonstrating the REST API and WebSocket server
///
/// This example shows how to:
/// - Initialize the metrics repository
/// - Set up shared state for current metrics and anomalies
/// - Configure and start the API server
/// - Simulate metrics collection and anomaly detection
///
/// Run with: cargo run --example api_server
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use collector::{MetricsCollector, MetricsRepository, ServerConfig};
use shared::types::{Anomaly, AnomalyCategory, AnomalySeverity, SystemMetrics};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,collector=debug")
        .init();

    println!("Starting System Monitor API Server Example");
    println!("==========================================\n");

    // Initialize database
    let database_url = "sqlite:./metrics_example.db";
    let repo = Arc::new(MetricsRepository::new(database_url).await?);

    println!("Database connected: {}", database_url);

    // Run migrations
    repo.run_migrations().await?;
    println!("Database migrations completed");

    // Create shared state
    let current_metrics: Arc<RwLock<Option<SystemMetrics>>> = Arc::new(RwLock::new(None));
    let recent_anomalies: Arc<RwLock<Vec<Anomaly>>> = Arc::new(RwLock::new(Vec::new()));

    println!("\nShared state initialized");

    // Configure server
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        enable_cors: true,
        cors_origins: vec![
            "http://localhost:3000".to_string(),
            "http://127.0.0.1:3000".to_string(),
        ],
    };

    println!("\nServer configuration:");
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  CORS: enabled");

    // Clone state for background task
    let metrics_clone = current_metrics.clone();
    let anomalies_clone = recent_anomalies.clone();
    let repo_clone = repo.clone();

    // Spawn background task to simulate metrics collection
    tokio::spawn(async move {
        let mut collector = match MetricsCollector::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[Background] Failed to initialize collector: {}", e);
                return;
            }
        };
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        println!("\n[Background] Metrics collector started (collecting every 5 seconds)");

        loop {
            interval.tick().await;

            // Collect metrics
            match collector.collect_all() {
                Ok(metrics) => {
                    println!("[Background] Collected metrics at {}", metrics.timestamp);

                    // Update current metrics
                    {
                        let mut current = metrics_clone.write().await;
                        *current = Some(metrics.clone());
                    }

                    // Store in database
                    if let Err(e) = repo_clone.store_metrics(&metrics).await {
                        eprintln!("[Background] Failed to store metrics: {}", e);
                    } else {
                        println!("[Background] Metrics stored in database");
                    }

                    // Simulate anomaly detection
                    if metrics.cpu.global_usage > 70.0 {
                        let anomaly = Anomaly {
                            id: uuid::Uuid::new_v4().to_string(),
                            timestamp: chrono::Utc::now(),
                            severity: AnomalySeverity::Warning,
                            category: AnomalyCategory::Cpu,
                            message: format!(
                                "High CPU usage detected: {:.2}%",
                                metrics.cpu.global_usage
                            ),
                            metrics: serde_json::json!({
                                "cpu_usage": metrics.cpu.global_usage,
                                "load_avg": metrics.cpu.load_avg_1,
                            }),
                        };

                        println!("[Background] Anomaly detected: {}", anomaly.message);

                        // Add to recent anomalies
                        {
                            let mut recent = anomalies_clone.write().await;
                            recent.push(anomaly.clone());
                            // Keep only last 100 anomalies
                            if recent.len() > 100 {
                                recent.remove(0);
                            }
                        }

                        // Store in database
                        if let Err(e) = repo_clone.store_anomaly(&anomaly).await {
                            eprintln!("[Background] Failed to store anomaly: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Background] Failed to collect metrics: {}", e);
                }
            }
        }
    });

    // Wait a moment before starting server to collect some initial data
    println!("\nWaiting 2 seconds for initial metrics collection...");
    sleep(Duration::from_secs(2)).await;

    println!("\n===========================================");
    println!("Server starting...");
    println!("===========================================\n");
    println!("Available endpoints:");
    println!("  GET  http://{}:{}/health", config.host, config.port);
    println!(
        "  GET  http://{}:{}/api/v1/metrics/current",
        config.host, config.port
    );
    println!(
        "  GET  http://{}:{}/api/v1/metrics/history?start=&end=&limit=",
        config.host, config.port
    );
    println!(
        "  GET  http://{}:{}/api/v1/anomalies?start=&end=&severity=&limit=",
        config.host, config.port
    );
    println!(
        "  GET  http://{}:{}/api/v1/anomalies/:id",
        config.host, config.port
    );
    println!(
        "  GET  http://{}:{}/api/v1/system/info",
        config.host, config.port
    );
    println!("  WS   ws://{}:{}/ws\n", config.host, config.port);
    println!("Press Ctrl+C to stop the server\n");

    // Start server (this will block until shutdown)
    collector::start_server(config, current_metrics, recent_anomalies, repo).await?;

    println!("\nServer stopped");
    Ok(())
}
