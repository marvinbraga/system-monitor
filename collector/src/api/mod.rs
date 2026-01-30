/// API module for REST and WebSocket endpoints
///
/// This module provides HTTP REST API endpoints and WebSocket support
/// for real-time metrics streaming.
mod rest;
mod routes;
mod websocket;

pub use rest::{start_server, start_server_with_shutdown, ServerConfig};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::storage::MetricsRepository;
use shared::types::{Anomaly, SystemMetrics};

/// Shared application state passed to all HTTP handlers
#[derive(Clone)]
pub struct AppState {
    /// Current system metrics (latest collected data)
    pub current_metrics: Arc<RwLock<Option<SystemMetrics>>>,

    /// Recent anomalies buffer (kept in memory for quick access)
    pub recent_anomalies: Arc<RwLock<Vec<Anomaly>>>,

    /// Database repository for historical data
    pub repository: Arc<MetricsRepository>,
}

impl AppState {
    /// Creates a new AppState instance
    ///
    /// # Arguments
    /// * `current_metrics` - Shared current metrics
    /// * `recent_anomalies` - Shared recent anomalies
    /// * `repository` - Database repository
    pub fn new(
        current_metrics: Arc<RwLock<Option<SystemMetrics>>>,
        recent_anomalies: Arc<RwLock<Vec<Anomaly>>>,
        repository: Arc<MetricsRepository>,
    ) -> Self {
        Self {
            current_metrics,
            recent_anomalies,
            repository,
        }
    }
}
