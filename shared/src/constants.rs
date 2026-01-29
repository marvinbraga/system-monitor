/// Default threshold values for anomaly detection
// CPU Thresholds
pub const CPU_USAGE_WARNING: f32 = 70.0;
pub const CPU_USAGE_CRITICAL: f32 = 90.0;
pub const CPU_LOAD_AVG_WARNING: f64 = 4.0;
pub const CPU_LOAD_AVG_CRITICAL: f64 = 8.0;

// Memory Thresholds
pub const MEMORY_USAGE_WARNING: f32 = 80.0;
pub const MEMORY_USAGE_CRITICAL: f32 = 95.0;
pub const SWAP_USAGE_WARNING: f32 = 50.0;
pub const SWAP_USAGE_CRITICAL: f32 = 80.0;

// Temperature Thresholds (Celsius)
pub const TEMPERATURE_WARNING: f32 = 75.0;
pub const TEMPERATURE_CRITICAL: f32 = 90.0;

// Disk Thresholds
pub const DISK_USAGE_WARNING: f32 = 80.0;
pub const DISK_USAGE_CRITICAL: f32 = 95.0;
pub const DISK_IO_WARNING_MB: f64 = 100.0;
pub const DISK_IO_CRITICAL_MB: f64 = 500.0;

// Network Thresholds (bytes per second)
pub const NETWORK_RX_WARNING: u64 = 100_000_000; // 100 MB/s
pub const NETWORK_RX_CRITICAL: u64 = 500_000_000; // 500 MB/s
pub const NETWORK_TX_WARNING: u64 = 100_000_000; // 100 MB/s
pub const NETWORK_TX_CRITICAL: u64 = 500_000_000; // 500 MB/s

// USB Device Thresholds
pub const USB_TIMEOUT_WARNING: bool = true;

// Collection Intervals (seconds)
pub const DEFAULT_COLLECTION_INTERVAL: u64 = 5;
pub const FAST_COLLECTION_INTERVAL: u64 = 1;
pub const SLOW_COLLECTION_INTERVAL: u64 = 60;

// Database Settings
pub const DEFAULT_RETENTION_DAYS: u32 = 30;
pub const MAX_ANOMALY_RECORDS: usize = 10000;

// Network Settings
pub const DEFAULT_API_PORT: u16 = 8080;
pub const DEFAULT_WEBSOCKET_PORT: u16 = 8081;
pub const MAX_WEBSOCKET_CLIENTS: usize = 100;

// TUI Settings
pub const DEFAULT_REFRESH_RATE_MS: u64 = 1000;
pub const MIN_REFRESH_RATE_MS: u64 = 100;
pub const MAX_REFRESH_RATE_MS: u64 = 10000;
