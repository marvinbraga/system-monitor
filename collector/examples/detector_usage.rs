/// Example usage of the anomaly detection system
///
/// This example demonstrates how to use the AnomalyRules detector
/// to identify system anomalies from collected metrics.
///
/// Run with: cargo run --example detector_usage
use chrono::Utc;
use collector::AnomalyRules;
use shared::types::{
    CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics, SystemMetrics, Temperature,
};

fn main() {
    println!("Anomaly Detection System Example\n");
    println!("=================================\n");

    // Initialize the detector with 8 CPUs
    let mut detector = AnomalyRules::new(8);

    // Simulate first measurement (baseline)
    println!("📊 First measurement (baseline):");
    let metrics1 = create_sample_metrics(30.0, 60.0, 55.0, 0, 50.0);
    print_metrics(&metrics1);

    let anomalies1 = detector.check(&metrics1);
    println!("   Anomalies detected: {}\n", anomalies1.len());

    // Simulate second measurement (CPU spike)
    println!("📊 Second measurement (CPU spike):");
    let metrics2 = create_sample_metrics(75.0, 65.0, 58.0, 0, 52.0);
    print_metrics(&metrics2);

    let anomalies2 = detector.check(&metrics2);
    print_anomalies(&anomalies2);

    // Simulate third measurement (memory critical)
    println!("📊 Third measurement (memory critical):");
    let metrics3 = create_sample_metrics(80.0, 96.0, 62.0, 0, 54.0);
    print_metrics(&metrics3);

    let anomalies3 = detector.check(&metrics3);
    print_anomalies(&anomalies3);

    // Simulate fourth measurement (SWAP activation)
    println!("📊 Fourth measurement (SWAP activation):");
    let metrics4 = create_sample_metrics(85.0, 98.0, 65.0, 512, 56.0);
    print_metrics(&metrics4);

    let anomalies4 = detector.check(&metrics4);
    print_anomalies(&anomalies4);

    // Simulate fifth measurement (temperature critical)
    println!("📊 Fifth measurement (temperature critical):");
    let metrics5 = create_sample_metrics(70.0, 90.0, 88.0, 256, 58.0);
    print_metrics(&metrics5);

    let anomalies5 = detector.check(&metrics5);
    print_anomalies(&anomalies5);

    println!("\n✓ Detection completed");
}

/// Create sample metrics for testing
fn create_sample_metrics(
    cpu_usage: f32,
    mem_usage: f32,
    temp: f32,
    swap_used: u64,
    disk_usage: f32,
) -> SystemMetrics {
    SystemMetrics {
        timestamp: Utc::now(),
        cpu: CpuMetrics {
            global_usage: cpu_usage,
            per_core: vec![cpu_usage; 8],
            load_avg_1: 2.0,
            load_avg_5: 1.5,
            load_avg_15: 1.2,
        },
        memory: MemoryMetrics {
            total: 16_000_000_000,
            used: (mem_usage * 160_000_000.0) as u64,
            available: ((100.0 - mem_usage) * 160_000_000.0) as u64,
            usage_percent: mem_usage,
            swap_total: 8_000_000_000,
            swap_used,
        },
        temperatures: vec![
            Temperature {
                sensor: "k10temp".to_string(),
                value: temp,
                label: "Tctl".to_string(),
            },
            Temperature {
                sensor: "nvme".to_string(),
                value: temp - 15.0,
                label: "Composite".to_string(),
            },
        ],
        disks: vec![DiskMetrics {
            name: "nvme0n1p2".to_string(),
            mount_point: "/".to_string(),
            total: 500_000_000_000,
            used: (disk_usage * 5_000_000_000.0) as u64,
            available: ((100.0 - disk_usage) * 5_000_000_000.0) as u64,
            usage_percent: disk_usage,
            read_mb: 50.0,
            write_mb: 30.0,
        }],
        usb_devices: vec![],
        network: NetworkMetrics {
            rx_bytes: 1_000_000,
            tx_bytes: 500_000,
            rx_packets: 1000,
            tx_packets: 500,
        },
        gpu: None,
    }
}

/// Print metrics summary
fn print_metrics(metrics: &SystemMetrics) {
    println!(
        "   CPU: {:.1}% | Memory: {:.1}% | Temp: {:.1}°C | SWAP: {} MB",
        metrics.cpu.global_usage,
        metrics.memory.usage_percent,
        metrics
            .temperatures
            .iter()
            .map(|t| t.value)
            .fold(0.0f32, f32::max),
        metrics.memory.swap_used / 1_000_000
    );
}

/// Print detected anomalies
fn print_anomalies(anomalies: &[shared::types::Anomaly]) {
    if anomalies.is_empty() {
        println!("   ✓ No anomalies detected\n");
    } else {
        println!("   ⚠ Anomalies detected: {}", anomalies.len());
        for anomaly in anomalies {
            let icon = match anomaly.severity {
                shared::types::AnomalySeverity::Critical => "🔴",
                shared::types::AnomalySeverity::Warning => "🟡",
                shared::types::AnomalySeverity::Info => "🔵",
            };
            println!("     {} [{:?}] {}", icon, anomaly.category, anomaly.message);
        }
        println!();
    }
}
