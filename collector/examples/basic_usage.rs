/// Example demonstrating basic usage of the MetricsRepository
///
/// Run with: cargo run --package collector --example basic_usage
use chrono::Utc;
use collector::MetricsRepository;
use shared::types::{
    Anomaly, AnomalyCategory, AnomalySeverity, CpuMetrics, DiskMetrics, MemoryMetrics,
    NetworkMetrics, SystemMetrics, Temperature, UsbDevice,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MetricsRepository Basic Usage Example ===\n");

    // Create repository with in-memory database
    println!("1. Creating repository...");
    let repo = MetricsRepository::new("sqlite::memory:").await?;
    println!("   ✓ Repository created\n");

    // Run migrations
    println!("2. Running migrations...");
    repo.run_migrations().await?;
    println!("   ✓ Migrations completed\n");

    // Create sample metrics
    println!("3. Creating sample metrics...");
    let metrics = SystemMetrics {
        timestamp: Utc::now(),
        cpu: CpuMetrics {
            global_usage: 45.5,
            per_core: vec![40.0, 50.0, 45.0, 48.0],
            load_avg_1: 1.5,
            load_avg_5: 1.2,
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
        temperatures: vec![
            Temperature {
                sensor: "CPU".to_string(),
                value: 65.0,
                label: "Core 0".to_string(),
            },
            Temperature {
                sensor: "CPU".to_string(),
                value: 67.0,
                label: "Core 1".to_string(),
            },
        ],
        disks: vec![DiskMetrics {
            name: "sda1".to_string(),
            mount_point: "/".to_string(),
            total: 500_000_000_000,
            used: 250_000_000_000,
            available: 250_000_000_000,
            usage_percent: 50.0,
            read_mb: 100.5,
            write_mb: 50.2,
        }],
        usb_devices: vec![UsbDevice {
            id: "usb-001".to_string(),
            manufacturer: "Logitech".to_string(),
            product: "USB Keyboard".to_string(),
            has_timeout: false,
        }],
        network: NetworkMetrics {
            rx_bytes: 1_000_000,
            tx_bytes: 500_000,
            rx_packets: 1000,
            tx_packets: 500,
        },
    };
    println!("   ✓ Sample metrics created\n");

    // Store metrics
    println!("4. Storing metrics...");
    let metrics_id = repo.store_metrics(&metrics).await?;
    println!("   ✓ Metrics stored with ID: {}\n", metrics_id);

    // Create sample anomaly
    println!("5. Creating sample anomaly...");
    let anomaly = Anomaly {
        id: "anomaly-001".to_string(),
        timestamp: Utc::now(),
        severity: AnomalySeverity::Warning,
        category: AnomalyCategory::Cpu,
        message: "CPU usage exceeded 80%".to_string(),
        metrics: serde_json::json!({
            "cpu_usage": 85.5,
            "threshold": 80.0
        }),
    };
    println!("   ✓ Sample anomaly created\n");

    // Store anomaly
    println!("6. Storing anomaly...");
    let anomaly_id = repo.store_anomaly(&anomaly).await?;
    println!("   ✓ Anomaly stored with ID: {}\n", anomaly_id);

    // Retrieve recent metrics
    println!("7. Retrieving recent metrics...");
    let recent_metrics = repo.get_recent_metrics(10).await?;
    println!("   ✓ Retrieved {} metrics\n", recent_metrics.len());

    // Retrieve recent anomalies
    println!("8. Retrieving recent anomalies...");
    let recent_anomalies = repo.get_recent_anomalies(10).await?;
    println!("   ✓ Retrieved {} anomalies\n", recent_anomalies.len());

    // Store and retrieve config
    println!("9. Testing configuration storage...");
    repo.set_config("collection_interval", "5").await?;
    let config_value = repo.get_config("collection_interval").await?;
    println!(
        "   ✓ Config stored and retrieved: {:?}\n",
        config_value.unwrap()
    );

    println!("=== Example completed successfully! ===");

    Ok(())
}
