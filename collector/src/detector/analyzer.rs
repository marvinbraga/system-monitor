/// Helper functions for analyzing metrics and calculating deltas
///
/// This module provides utilities for comparing metrics between intervals,
/// calculating rates of change, and classifying anomaly severity.
use shared::types::{AnomalySeverity, SystemMetrics};

/// Represents the delta/change between two metric measurements
#[derive(Debug, Clone)]
pub struct MetricsDelta {
    pub cpu_usage_delta: f32,
    pub memory_usage_delta: f32,
    pub temperature_delta: f32,
    pub swap_delta: i64,
    pub disk_io_delta: f64,
}

/// Calculate the delta between two system metric snapshots
pub fn calculate_delta(current: &SystemMetrics, previous: &SystemMetrics) -> MetricsDelta {
    let cpu_usage_delta = current.cpu.global_usage - previous.cpu.global_usage;
    let memory_usage_delta = current.memory.usage_percent - previous.memory.usage_percent;

    // Calculate max temperature delta
    let current_max_temp = current
        .temperatures
        .iter()
        .map(|t| t.value)
        .fold(0.0f32, f32::max);
    let previous_max_temp = previous
        .temperatures
        .iter()
        .map(|t| t.value)
        .fold(0.0f32, f32::max);
    let temperature_delta = current_max_temp - previous_max_temp;

    // Calculate swap delta
    let swap_delta = current.memory.swap_used as i64 - previous.memory.swap_used as i64;

    // Calculate max disk I/O delta
    let current_max_io = current
        .disks
        .iter()
        .map(|d| d.read_mb + d.write_mb)
        .fold(0.0f64, f64::max);
    let previous_max_io = previous
        .disks
        .iter()
        .map(|d| d.read_mb + d.write_mb)
        .fold(0.0f64, f64::max);
    let disk_io_delta = current_max_io - previous_max_io;

    MetricsDelta {
        cpu_usage_delta,
        memory_usage_delta,
        temperature_delta,
        swap_delta,
        disk_io_delta,
    }
}

/// Calculate the rate of change per second
///
/// # Arguments
/// * `delta` - The change in value
/// * `time_interval_secs` - The time interval in seconds
pub fn calculate_rate(delta: f64, time_interval_secs: f64) -> f64 {
    if time_interval_secs > 0.0 {
        delta / time_interval_secs
    } else {
        0.0
    }
}

/// Classify severity based on value and thresholds
///
/// # Arguments
/// * `value` - The current value to check
/// * `warning_threshold` - The warning level threshold
/// * `critical_threshold` - The critical level threshold
///
/// # Returns
/// * `Some(AnomalySeverity)` if value exceeds a threshold, `None` otherwise
pub fn classify_severity(
    value: f32,
    warning_threshold: f32,
    critical_threshold: f32,
) -> Option<AnomalySeverity> {
    if value >= critical_threshold {
        Some(AnomalySeverity::Critical)
    } else if value >= warning_threshold {
        Some(AnomalySeverity::Warning)
    } else {
        None
    }
}

/// Get the maximum temperature from a metrics snapshot
pub fn get_max_temperature(metrics: &SystemMetrics) -> f32 {
    metrics
        .temperatures
        .iter()
        .map(|t| t.value)
        .fold(0.0f32, f32::max)
}

/// Get the maximum disk usage percentage from a metrics snapshot
pub fn get_max_disk_usage(metrics: &SystemMetrics) -> f32 {
    metrics
        .disks
        .iter()
        .map(|d| d.usage_percent)
        .fold(0.0f32, f32::max)
}

/// Get the maximum disk I/O (read + write) from a metrics snapshot
pub fn get_max_disk_io(metrics: &SystemMetrics) -> f64 {
    metrics
        .disks
        .iter()
        .map(|d| d.read_mb + d.write_mb)
        .fold(0.0f64, f64::max)
}

/// Check if any USB device has a timeout
pub fn has_usb_timeout(metrics: &SystemMetrics) -> bool {
    metrics.usb_devices.iter().any(|usb| usb.has_timeout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use shared::types::{CpuMetrics, MemoryMetrics, Temperature};

    #[test]
    fn test_classify_severity() {
        assert_eq!(classify_severity(50.0, 70.0, 90.0), None);
        assert_eq!(
            classify_severity(75.0, 70.0, 90.0),
            Some(AnomalySeverity::Warning)
        );
        assert_eq!(
            classify_severity(95.0, 70.0, 90.0),
            Some(AnomalySeverity::Critical)
        );
    }

    #[test]
    fn test_calculate_rate() {
        assert_eq!(calculate_rate(100.0, 10.0), 10.0);
        assert_eq!(calculate_rate(50.0, 5.0), 10.0);
        assert_eq!(calculate_rate(100.0, 0.0), 0.0);
    }

    #[test]
    fn test_calculate_delta() {
        let current = SystemMetrics {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                global_usage: 80.0,
                per_core: vec![],
                load_avg_1: 2.0,
                load_avg_5: 1.5,
                load_avg_15: 1.0,
            },
            memory: MemoryMetrics {
                total: 1000,
                used: 800,
                available: 200,
                usage_percent: 80.0,
                swap_total: 1000,
                swap_used: 500,
            },
            temperatures: vec![Temperature {
                sensor: "cpu".to_string(),
                value: 75.0,
                label: "CPU".to_string(),
            }],
            disks: vec![],
            usb_devices: vec![],
            network: shared::types::NetworkMetrics {
                rx_bytes: 0,
                tx_bytes: 0,
                rx_packets: 0,
                tx_packets: 0,
            },
            gpu: None,
        };

        let previous = SystemMetrics {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                global_usage: 40.0,
                per_core: vec![],
                load_avg_1: 1.0,
                load_avg_5: 1.0,
                load_avg_15: 1.0,
            },
            memory: MemoryMetrics {
                total: 1000,
                used: 500,
                available: 500,
                usage_percent: 50.0,
                swap_total: 1000,
                swap_used: 0,
            },
            temperatures: vec![Temperature {
                sensor: "cpu".to_string(),
                value: 50.0,
                label: "CPU".to_string(),
            }],
            disks: vec![],
            usb_devices: vec![],
            network: shared::types::NetworkMetrics {
                rx_bytes: 0,
                tx_bytes: 0,
                rx_packets: 0,
                tx_packets: 0,
            },
            gpu: None,
        };

        let delta = calculate_delta(&current, &previous);
        assert_eq!(delta.cpu_usage_delta, 40.0);
        assert_eq!(delta.memory_usage_delta, 30.0);
        assert_eq!(delta.temperature_delta, 25.0);
        assert_eq!(delta.swap_delta, 500);
    }
}
