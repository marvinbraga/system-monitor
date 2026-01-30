/// Example demonstrating time range queries and cleanup
///
/// Run with: cargo run --package collector --example test_range_queries
use chrono::{Duration, Utc};
use collector::MetricsRepository;
use shared::types::{
    Anomaly, AnomalyCategory, AnomalySeverity, CpuMetrics, MemoryMetrics, NetworkMetrics,
    SystemMetrics,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Time Range Queries Test ===\n");

    // Create repository
    let repo = MetricsRepository::new("sqlite::memory:").await?;
    repo.run_migrations().await?;

    // Create metrics at different times
    println!("1. Creating test data...");
    let now = Utc::now();

    for i in 0..5 {
        let timestamp = now - Duration::hours(i);
        let metrics = SystemMetrics {
            timestamp,
            cpu: CpuMetrics {
                global_usage: 40.0 + (i as f32 * 5.0),
                per_core: vec![40.0, 45.0],
                load_avg_1: 1.0,
                load_avg_5: 1.0,
                load_avg_15: 1.0,
            },
            memory: MemoryMetrics {
                total: 16_000_000_000,
                used: 8_000_000_000,
                available: 8_000_000_000,
                usage_percent: 50.0,
                swap_total: 4_000_000_000,
                swap_used: 500_000_000,
            },
            temperatures: vec![],
            disks: vec![],
            usb_devices: vec![],
            network: NetworkMetrics {
                rx_bytes: 1_000_000,
                tx_bytes: 500_000,
                rx_packets: 1000,
                tx_packets: 500,
            },
            gpu: None,
        };
        repo.store_metrics(&metrics).await?;

        if i % 2 == 0 {
            let anomaly = Anomaly {
                id: format!("anomaly-{}", i),
                timestamp,
                severity: AnomalySeverity::Warning,
                category: AnomalyCategory::Cpu,
                message: format!("Test anomaly {}", i),
                metrics: serde_json::json!({"index": i}),
            };
            repo.store_anomaly(&anomaly).await?;
        }
    }
    println!("   ✓ Created 5 metrics and 3 anomalies\n");

    // Query metrics in last 3 hours
    println!("2. Querying metrics from last 3 hours...");
    let start = now - Duration::hours(3);
    let end = now;
    let metrics = repo.get_metrics_range(start, end).await?;
    println!("   ✓ Found {} metrics\n", metrics.len());

    // Query anomalies in last 3 hours
    println!("3. Querying anomalies from last 3 hours...");
    let anomalies = repo.get_anomalies_range(start, end).await?;
    println!("   ✓ Found {} anomalies\n", anomalies.len());

    // Test cleanup (this would delete data older than 2 hours)
    println!("4. Testing cleanup (retention: 2 hours)...");
    let retention_hours = 2;
    let (deleted_metrics, deleted_anomalies) =
        repo.cleanup_old_data(retention_hours as i64 / 24).await?;
    println!(
        "   ✓ Would delete {} metrics and {} anomalies\n",
        deleted_metrics, deleted_anomalies
    );

    // Verify remaining data
    println!("5. Verifying remaining data...");
    let all_metrics = repo.get_recent_metrics(100).await?;
    let all_anomalies = repo.get_recent_anomalies(100).await?;
    println!(
        "   ✓ Remaining: {} metrics, {} anomalies\n",
        all_metrics.len(),
        all_anomalies.len()
    );

    println!("=== Test completed successfully! ===");

    Ok(())
}
