/// Collector service library
///
/// This library provides the core functionality for the system monitoring collector,
/// including metrics collection, storage, and anomaly detection.
pub mod api;
pub mod collectors;
pub mod detector;
pub mod storage;

// Re-export commonly used types
pub use api::{start_server, AppState, ServerConfig};
pub use collectors::MetricsCollector;
pub use detector::AnomalyRules;
pub use storage::{AnomalyRow, MetricsRepository, MetricsRow};
