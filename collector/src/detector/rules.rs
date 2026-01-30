/// Anomaly detection rules
///
/// This module implements the AnomalyRules struct that detects various system anomalies
/// by comparing current metrics against previous values and thresholds.
use chrono::Utc;
use shared::types::{Anomaly, AnomalyCategory, AnomalySeverity, SystemMetrics};
use uuid::Uuid;

use super::analyzer::{get_max_disk_io, get_max_disk_usage, get_max_temperature, has_usb_timeout};

/// Threshold constants for anomaly detection
/// These values are based on the original implementation in main.rs
const CPU_SPIKE_THRESHOLD: f32 = 40.0; // % increase
const CPU_CRITICAL_THRESHOLD: f32 = 90.0; // % usage
const MEMORY_SPIKE_THRESHOLD: f32 = 20.0; // % increase
const MEMORY_CRITICAL_THRESHOLD: f32 = 95.0; // % usage
const TEMPERATURE_CRITICAL: f32 = 85.0; // °C
const TEMPERATURE_DROP_THRESHOLD: f32 = 30.0; // °C decrease
const DISK_CRITICAL_THRESHOLD: f32 = 90.0; // % usage
const DISK_IO_HIGH_THRESHOLD: f64 = 500.0; // MB/s
const LOAD_AVG_MULTIPLIER: f64 = 2.0; // times number of CPUs
const GPU_TEMP_CRITICAL: f32 = 90.0; // °C
const GPU_USAGE_CRITICAL: f32 = 95.0; // % usage
const GPU_MEMORY_CRITICAL: f32 = 95.0; // % usage

/// Stores state for anomaly detection and implements detection rules
pub struct AnomalyRules {
    previous_metrics: Option<SystemMetrics>,
    num_cpus: usize,
}

impl AnomalyRules {
    /// Create a new AnomalyRules instance
    ///
    /// # Arguments
    /// * `num_cpus` - Number of CPU cores in the system (used for load average checks)
    pub fn new(num_cpus: usize) -> Self {
        Self {
            previous_metrics: None,
            num_cpus,
        }
    }

    /// Check for anomalies in the current metrics
    ///
    /// This method compares current metrics against previous values and defined thresholds
    /// to detect various types of anomalies.
    ///
    /// # Arguments
    /// * `current` - The current system metrics snapshot
    ///
    /// # Returns
    /// A vector of detected anomalies
    pub fn check(&mut self, current: &SystemMetrics) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // If we have previous metrics, check for changes/spikes
        if let Some(ref previous) = self.previous_metrics {
            self.check_comparative_anomalies(current, previous, &mut anomalies);
        }

        // Check absolute threshold anomalies (don't require previous state)
        self.check_absolute_anomalies(current, &mut anomalies);

        // Store current metrics for next iteration
        self.previous_metrics = Some(current.clone());

        anomalies
    }

    /// Check anomalies that require comparison with previous metrics
    fn check_comparative_anomalies(
        &self,
        current: &SystemMetrics,
        previous: &SystemMetrics,
        anomalies: &mut Vec<Anomaly>,
    ) {
        // CPU spike detection
        let cpu_delta = current.cpu.global_usage - previous.cpu.global_usage;
        if cpu_delta > CPU_SPIKE_THRESHOLD {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Warning,
                AnomalyCategory::Cpu,
                format!(
                    "CPU spike detected: {:.0}% → {:.0}%",
                    previous.cpu.global_usage, current.cpu.global_usage
                ),
                serde_json::json!({
                    "previous": previous.cpu.global_usage,
                    "current": current.cpu.global_usage,
                    "delta": cpu_delta,
                }),
            ));
        }

        // CPU sustained high usage (critical only if both current and previous are high)
        if current.cpu.global_usage > CPU_CRITICAL_THRESHOLD
            && previous.cpu.global_usage > CPU_CRITICAL_THRESHOLD
        {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Critical,
                AnomalyCategory::Cpu,
                format!(
                    "Sustained critical CPU usage: {:.0}%",
                    current.cpu.global_usage
                ),
                serde_json::json!({
                    "usage": current.cpu.global_usage,
                }),
            ));
        }

        // Memory spike detection
        let mem_delta = current.memory.usage_percent - previous.memory.usage_percent;
        if mem_delta > MEMORY_SPIKE_THRESHOLD {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Warning,
                AnomalyCategory::Memory,
                format!(
                    "Memory spike detected: {:.0}% → {:.0}%",
                    previous.memory.usage_percent, current.memory.usage_percent
                ),
                serde_json::json!({
                    "previous": previous.memory.usage_percent,
                    "current": current.memory.usage_percent,
                    "delta": mem_delta,
                }),
            ));
        }

        // Temperature spike detection
        let current_max_temp = get_max_temperature(current);
        let previous_max_temp = get_max_temperature(previous);

        if current_max_temp > TEMPERATURE_CRITICAL && previous_max_temp <= TEMPERATURE_CRITICAL {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Critical,
                AnomalyCategory::Temperature,
                format!("Critical temperature reached: {:.0}°C", current_max_temp),
                serde_json::json!({
                    "temperature": current_max_temp,
                }),
            ));
        }

        // Temperature drop detection (possible sensor issue or sudden cooling)
        let temp_delta = previous_max_temp - current_max_temp;
        if temp_delta > TEMPERATURE_DROP_THRESHOLD {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Warning,
                AnomalyCategory::Temperature,
                format!(
                    "Sudden temperature drop: {:.0}°C → {:.0}°C",
                    previous_max_temp, current_max_temp
                ),
                serde_json::json!({
                    "previous": previous_max_temp,
                    "current": current_max_temp,
                    "delta": temp_delta,
                }),
            ));
        }

        // Swap activation detection (swap was off, now it's on)
        if current.memory.swap_used > 0 && previous.memory.swap_used == 0 {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Warning,
                AnomalyCategory::Memory,
                "SWAP memory activated".to_string(),
                serde_json::json!({
                    "swap_used": current.memory.swap_used,
                }),
            ));
        }
    }

    /// Check anomalies based on absolute thresholds (no previous state needed)
    fn check_absolute_anomalies(&self, current: &SystemMetrics, anomalies: &mut Vec<Anomaly>) {
        // Memory critical threshold
        if current.memory.usage_percent > MEMORY_CRITICAL_THRESHOLD {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Critical,
                AnomalyCategory::Memory,
                format!(
                    "Critical memory usage: {:.0}%",
                    current.memory.usage_percent
                ),
                serde_json::json!({
                    "usage": current.memory.usage_percent,
                }),
            ));
        }

        // Disk critical threshold
        let max_disk_usage = get_max_disk_usage(current);
        if max_disk_usage > DISK_CRITICAL_THRESHOLD {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Warning,
                AnomalyCategory::Disk,
                format!("Critical disk usage: {:.0}%", max_disk_usage),
                serde_json::json!({
                    "usage": max_disk_usage,
                }),
            ));
        }

        // USB timeout detection
        if has_usb_timeout(current) {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Critical,
                AnomalyCategory::Usb,
                "USB timeout detected".to_string(),
                serde_json::json!({
                    "devices": current.usb_devices.iter()
                        .filter(|d| d.has_timeout)
                        .map(|d| &d.id)
                        .collect::<Vec<_>>(),
                }),
            ));
        }

        // High disk I/O detection
        let max_disk_io = get_max_disk_io(current);
        if max_disk_io > DISK_IO_HIGH_THRESHOLD {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Warning,
                AnomalyCategory::Disk,
                format!("High disk I/O: {:.0} MB/s", max_disk_io),
                serde_json::json!({
                    "io_mbs": max_disk_io,
                }),
            ));
        }

        // High load average detection
        let load_threshold = (self.num_cpus as f64) * LOAD_AVG_MULTIPLIER;
        if current.cpu.load_avg_15 > load_threshold {
            anomalies.push(self.create_anomaly(
                AnomalySeverity::Critical,
                AnomalyCategory::Cpu,
                format!("Critical load average: {:.2}", current.cpu.load_avg_15),
                serde_json::json!({
                    "load_avg_15": current.cpu.load_avg_15,
                    "threshold": load_threshold,
                    "num_cpus": self.num_cpus,
                }),
            ));
        }

        // GPU anomalies detection
        if let Some(ref gpu) = current.gpu {
            // GPU temperature critical
            if gpu.temperature > GPU_TEMP_CRITICAL {
                anomalies.push(self.create_anomaly(
                    AnomalySeverity::Critical,
                    AnomalyCategory::Gpu,
                    format!("Critical GPU temperature: {:.0}°C", gpu.temperature),
                    serde_json::json!({
                        "temperature": gpu.temperature,
                        "gpu_name": gpu.name,
                    }),
                ));
            }

            // GPU usage critical
            if gpu.usage_percent > GPU_USAGE_CRITICAL {
                anomalies.push(self.create_anomaly(
                    AnomalySeverity::Warning,
                    AnomalyCategory::Gpu,
                    format!("Critical GPU usage: {:.0}%", gpu.usage_percent),
                    serde_json::json!({
                        "usage": gpu.usage_percent,
                        "gpu_name": gpu.name,
                    }),
                ));
            }

            // GPU memory critical
            if gpu.memory_usage_percent > GPU_MEMORY_CRITICAL {
                anomalies.push(self.create_anomaly(
                    AnomalySeverity::Warning,
                    AnomalyCategory::Gpu,
                    format!("Critical GPU memory usage: {:.0}%", gpu.memory_usage_percent),
                    serde_json::json!({
                        "memory_usage": gpu.memory_usage_percent,
                        "memory_used_mb": gpu.memory_used_mb,
                        "memory_total_mb": gpu.memory_total_mb,
                        "gpu_name": gpu.name,
                    }),
                ));
            }
        }
    }

    /// Helper method to create an anomaly with consistent structure
    fn create_anomaly(
        &self,
        severity: AnomalySeverity,
        category: AnomalyCategory,
        message: String,
        metrics: serde_json::Value,
    ) -> Anomaly {
        Anomaly {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            severity,
            category,
            message,
            metrics,
        }
    }

    /// Reset the state (clear previous metrics)
    pub fn reset(&mut self) {
        self.previous_metrics = None;
    }

    /// Check if there are previous metrics stored
    pub fn has_previous_metrics(&self) -> bool {
        self.previous_metrics.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use shared::types::{CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics, Temperature};

    fn create_test_metrics(
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
                per_core: vec![],
                load_avg_1: 1.0,
                load_avg_5: 1.0,
                load_avg_15: 1.0,
            },
            memory: MemoryMetrics {
                total: 1000,
                used: (mem_usage * 10.0) as u64,
                available: ((100.0 - mem_usage) * 10.0) as u64,
                usage_percent: mem_usage,
                swap_total: 1000,
                swap_used,
            },
            temperatures: vec![Temperature {
                sensor: "cpu".to_string(),
                value: temp,
                label: "CPU".to_string(),
            }],
            disks: vec![DiskMetrics {
                name: "sda".to_string(),
                mount_point: "/".to_string(),
                total: 1000,
                used: (disk_usage * 10.0) as u64,
                available: ((100.0 - disk_usage) * 10.0) as u64,
                usage_percent: disk_usage,
                read_mb: 0.0,
                write_mb: 0.0,
            }],
            usb_devices: vec![],
            network: NetworkMetrics {
                rx_bytes: 0,
                tx_bytes: 0,
                rx_packets: 0,
                tx_packets: 0,
            },
            gpu: None,
        }
    }

    #[test]
    fn test_cpu_spike_detection() {
        let mut rules = AnomalyRules::new(8);

        // First reading - no anomaly
        let metrics1 = create_test_metrics(30.0, 50.0, 60.0, 0, 50.0);
        let anomalies1 = rules.check(&metrics1);
        assert_eq!(anomalies1.len(), 0);

        // Second reading - CPU spike
        let metrics2 = create_test_metrics(75.0, 50.0, 60.0, 0, 50.0);
        let anomalies2 = rules.check(&metrics2);
        assert!(anomalies2
            .iter()
            .any(|a| matches!(a.category, AnomalyCategory::Cpu)));
    }

    #[test]
    fn test_memory_critical() {
        let mut rules = AnomalyRules::new(8);

        let metrics = create_test_metrics(50.0, 96.0, 60.0, 0, 50.0);
        let anomalies = rules.check(&metrics);

        assert!(anomalies.iter().any(|a| {
            matches!(a.category, AnomalyCategory::Memory)
                && matches!(a.severity, AnomalySeverity::Critical)
        }));
    }

    #[test]
    fn test_swap_activation() {
        let mut rules = AnomalyRules::new(8);

        // First reading - no swap
        let metrics1 = create_test_metrics(50.0, 70.0, 60.0, 0, 50.0);
        rules.check(&metrics1);

        // Second reading - swap activated
        let metrics2 = create_test_metrics(50.0, 80.0, 60.0, 500, 50.0);
        let anomalies = rules.check(&metrics2);

        assert!(anomalies.iter().any(|a| {
            matches!(a.category, AnomalyCategory::Memory) && a.message.contains("SWAP")
        }));
    }

    #[test]
    fn test_temperature_critical() {
        let mut rules = AnomalyRules::new(8);

        // First reading - normal temp
        let metrics1 = create_test_metrics(50.0, 50.0, 70.0, 0, 50.0);
        rules.check(&metrics1);

        // Second reading - critical temp
        let metrics2 = create_test_metrics(50.0, 50.0, 90.0, 0, 50.0);
        let anomalies = rules.check(&metrics2);

        assert!(anomalies.iter().any(|a| {
            matches!(a.category, AnomalyCategory::Temperature)
                && matches!(a.severity, AnomalySeverity::Critical)
        }));
    }

    #[test]
    fn test_reset_state() {
        let mut rules = AnomalyRules::new(8);

        let metrics = create_test_metrics(50.0, 50.0, 60.0, 0, 50.0);
        rules.check(&metrics);

        assert!(rules.has_previous_metrics());

        rules.reset();
        assert!(!rules.has_previous_metrics());
    }
}
