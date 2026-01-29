/// Database storage layer for the collector service
///
/// This module provides SQLite-based persistence for system metrics and anomalies.
/// It includes:
/// - Database migrations for schema setup
/// - Data models for database rows
/// - Repository pattern for data access
pub mod models;
pub mod repository;

// Re-export commonly used types
pub use models::{AnomalyRow, ConfigRow, MetricsRow, NewAnomaly, NewConfig, NewMetrics};
pub use repository::MetricsRepository;
