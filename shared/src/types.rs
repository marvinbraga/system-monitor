use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Main system metrics structure containing all collected data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub temperatures: Vec<Temperature>,
    pub disks: Vec<DiskMetrics>,
    pub usb_devices: Vec<UsbDevice>,
    pub network: NetworkMetrics,
}

/// CPU metrics including global usage, per-core usage, and load averages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub global_usage: f32,
    pub per_core: Vec<f32>,
    pub load_avg_1: f64,
    pub load_avg_5: f64,
    pub load_avg_15: f64,
}

/// Memory metrics including RAM and swap usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
}

/// Temperature sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperature {
    pub sensor: String,
    pub value: f32,
    pub label: String,
}

/// Disk metrics including space usage and I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub read_mb: f64,
    pub write_mb: f64,
}

/// USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub id: String,
    pub manufacturer: String,
    pub product: String,
    pub has_timeout: bool,
}

/// Network metrics including bytes and packets transferred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub severity: AnomalySeverity,
    pub category: AnomalyCategory,
    pub message: String,
    pub metrics: serde_json::Value,
}

/// Severity level of an anomaly
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Info,
    Warning,
    Critical,
}

/// Category of system component where anomaly was detected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyCategory {
    Cpu,
    Memory,
    Temperature,
    Disk,
    Usb,
    Network,
    System,
}
