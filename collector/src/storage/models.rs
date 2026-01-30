use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Database model for system metrics
/// Maps to the metrics table in SQLite
#[derive(Debug, Clone, FromRow)]
pub struct MetricsRow {
    pub id: i64,
    pub timestamp: String, // SQLite stores as TEXT

    // CPU metrics
    pub cpu_global: f32,
    pub cpu_per_core: String, // JSON array
    pub load_avg_1: f64,
    pub load_avg_5: f64,
    pub load_avg_15: f64,

    // Memory metrics
    pub memory_total: i64,
    pub memory_used: i64,
    pub memory_available: i64,
    pub memory_percent: f32,
    pub swap_total: i64,
    pub swap_used: i64,

    // Complex metrics as JSON
    pub temperatures: Option<String>, // JSON array
    pub disks: Option<String>,        // JSON array
    pub usb_devices: Option<String>,  // JSON array
    pub gpu: Option<String>,          // JSON object

    // Network metrics
    pub network_rx: i64,
    pub network_tx: i64,
    pub network_rx_packets: i64,
    pub network_tx_packets: i64,

    pub created_at: String, // SQLite stores as TEXT
}

/// Database model for anomalies
/// Maps to the anomalies table in SQLite
#[derive(Debug, Clone, FromRow)]
pub struct AnomalyRow {
    pub id: i64,
    pub timestamp: String, // SQLite stores as TEXT
    pub severity: String,
    pub category: String,
    pub message: String,
    pub metrics: String,    // JSON object
    pub created_at: String, // SQLite stores as TEXT
}

/// Database model for configuration
/// Maps to the config table in SQLite
#[derive(Debug, Clone, FromRow)]
pub struct ConfigRow {
    pub key: String,
    pub value: String,
    pub updated_at: String, // SQLite stores as TEXT
}

/// Insert model for new metrics
#[derive(Debug, Clone)]
pub struct NewMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_global: f32,
    pub cpu_per_core: String, // JSON serialized
    pub load_avg_1: f64,
    pub load_avg_5: f64,
    pub load_avg_15: f64,
    pub memory_total: i64,
    pub memory_used: i64,
    pub memory_available: i64,
    pub memory_percent: f32,
    pub swap_total: i64,
    pub swap_used: i64,
    pub temperatures: Option<String>, // JSON serialized
    pub disks: Option<String>,        // JSON serialized
    pub usb_devices: Option<String>,  // JSON serialized
    pub gpu: Option<String>,          // JSON serialized
    pub network_rx: i64,
    pub network_tx: i64,
    pub network_rx_packets: i64,
    pub network_tx_packets: i64,
}

/// Insert model for new anomaly
#[derive(Debug, Clone)]
pub struct NewAnomaly {
    pub timestamp: DateTime<Utc>,
    pub severity: String,
    pub category: String,
    pub message: String,
    pub metrics: String, // JSON serialized
}

/// Insert/update model for configuration
#[derive(Debug, Clone)]
pub struct NewConfig {
    pub key: String,
    pub value: String,
}
