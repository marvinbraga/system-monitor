/// Route handlers for REST API endpoints
///
/// Provides HTTP handlers for metrics, anomalies, system info, and health checks.
use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use super::{websocket, AppState};

/// Creates the main application router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // API routes
        .route("/api/v1/metrics/current", get(get_current_metrics))
        .route("/api/v1/metrics/history", get(get_metrics_history))
        .route("/api/v1/anomalies", get(get_anomalies))
        .route("/api/v1/anomalies/:id", get(get_anomaly_by_id))
        .route("/api/v1/system/info", get(get_system_info))
        // Health check
        .route("/health", get(health_check))
        // WebSocket endpoint
        .route("/ws", get(websocket_handler))
        .with_state(state)
}

// ============================================================================
// Handler Functions
// ============================================================================

/// GET /api/v1/metrics/current
/// Returns the current system metrics
async fn get_current_metrics(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let metrics = state.current_metrics.read().await;

    match metrics.as_ref() {
        Some(m) => Ok(Json(json!({
            "status": "success",
            "data": m,
        }))),
        None => Ok(Json(json!({
            "status": "success",
            "data": null,
            "message": "No metrics collected yet",
        }))),
    }
}

/// Query parameters for metrics history endpoint
#[derive(Debug, Deserialize)]
struct HistoryQuery {
    /// Start timestamp in RFC3339 format (optional, defaults to 1 hour ago)
    start: Option<String>,
    /// End timestamp in RFC3339 format (optional, defaults to now)
    end: Option<String>,
    /// Maximum number of records to return (optional, defaults to 100)
    limit: Option<i64>,
}

/// GET /api/v1/metrics/history?start=&end=&limit=
/// Returns historical metrics within a time range
async fn get_metrics_history(
    State(state): State<AppState>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Parse timestamps or use defaults
    let end = match query.end {
        Some(end_str) => DateTime::parse_from_rfc3339(&end_str)
            .map_err(|e| AppError::BadRequest(format!("Invalid end timestamp: {}", e)))?
            .with_timezone(&Utc),
        None => Utc::now(),
    };

    let start = match query.start {
        Some(start_str) => DateTime::parse_from_rfc3339(&start_str)
            .map_err(|e| AppError::BadRequest(format!("Invalid start timestamp: {}", e)))?
            .with_timezone(&Utc),
        None => end - Duration::hours(1),
    };

    // Validate time range
    if start > end {
        return Err(AppError::BadRequest(
            "Start timestamp must be before end timestamp".to_string(),
        ));
    }

    // Fetch metrics from database
    let metrics = if let Some(limit) = query.limit {
        // If limit is specified, get recent metrics
        state
            .repository
            .get_recent_metrics(limit)
            .await
            .map_err(|e| {
                error!("Failed to fetch recent metrics: {}", e);
                AppError::DatabaseError(e.to_string())
            })?
    } else {
        // Otherwise, get metrics in time range
        state
            .repository
            .get_metrics_range(start, end)
            .await
            .map_err(|e| {
                error!("Failed to fetch metrics range: {}", e);
                AppError::DatabaseError(e.to_string())
            })?
    };

    Ok(Json(json!({
        "status": "success",
        "data": {
            "metrics": metrics,
            "count": metrics.len(),
            "start": start.to_rfc3339(),
            "end": end.to_rfc3339(),
        },
    })))
}

/// Query parameters for anomalies endpoint
#[derive(Debug, Deserialize)]
struct AnomaliesQuery {
    /// Start timestamp in RFC3339 format (optional, defaults to 24 hours ago)
    start: Option<String>,
    /// End timestamp in RFC3339 format (optional, defaults to now)
    end: Option<String>,
    /// Filter by severity: info, warning, critical (optional)
    severity: Option<String>,
    /// Maximum number of records to return (optional, defaults to 100)
    limit: Option<i64>,
}

/// GET /api/v1/anomalies?start=&end=&severity=&limit=
/// Returns anomalies list with optional filtering
async fn get_anomalies(
    State(state): State<AppState>,
    Query(query): Query<AnomaliesQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Parse timestamps or use defaults
    let end = match query.end {
        Some(end_str) => DateTime::parse_from_rfc3339(&end_str)
            .map_err(|e| AppError::BadRequest(format!("Invalid end timestamp: {}", e)))?
            .with_timezone(&Utc),
        None => Utc::now(),
    };

    let start = match query.start {
        Some(start_str) => DateTime::parse_from_rfc3339(&start_str)
            .map_err(|e| AppError::BadRequest(format!("Invalid start timestamp: {}", e)))?
            .with_timezone(&Utc),
        None => end - Duration::hours(24),
    };

    // Validate time range
    if start > end {
        return Err(AppError::BadRequest(
            "Start timestamp must be before end timestamp".to_string(),
        ));
    }

    // Fetch anomalies from database
    let mut anomalies = if let Some(limit) = query.limit {
        state
            .repository
            .get_recent_anomalies(limit)
            .await
            .map_err(|e| {
                error!("Failed to fetch recent anomalies: {}", e);
                AppError::DatabaseError(e.to_string())
            })?
    } else {
        state
            .repository
            .get_anomalies_range(start, end)
            .await
            .map_err(|e| {
                error!("Failed to fetch anomalies range: {}", e);
                AppError::DatabaseError(e.to_string())
            })?
    };

    // Filter by severity if specified
    if let Some(severity_str) = query.severity {
        let severity_lower = severity_str.to_lowercase();
        anomalies.retain(|a| {
            let a_severity = format!("{:?}", a.severity).to_lowercase();
            a_severity == severity_lower
        });
    }

    Ok(Json(json!({
        "status": "success",
        "data": {
            "anomalies": anomalies,
            "count": anomalies.len(),
            "start": start.to_rfc3339(),
            "end": end.to_rfc3339(),
        },
    })))
}

/// GET /api/v1/anomalies/:id
/// Get specific anomaly by ID
async fn get_anomaly_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Check in-memory recent anomalies first
    let recent = state.recent_anomalies.read().await;
    if let Some(anomaly) = recent.iter().find(|a| a.id == id) {
        return Ok(Json(json!({
            "status": "success",
            "data": anomaly,
        })));
    }

    // If not found in memory, we could query the database by parsing the ID
    // For now, return not found
    Err(AppError::NotFound(format!(
        "Anomaly with id {} not found",
        id
    )))
}

/// Response structure for system info
#[derive(Debug, Serialize)]
struct SystemInfo {
    hostname: String,
    os: String,
    kernel_version: String,
    uptime: u64,
    cpu_count: usize,
}

/// GET /api/v1/system/info
/// Returns system information
async fn get_system_info() -> Result<Json<serde_json::Value>, AppError> {
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let info = SystemInfo {
        hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
        os: System::name().unwrap_or_else(|| "unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
        uptime: System::uptime(),
        cpu_count: sys.cpus().len(),
    };

    Ok(Json(json!({
        "status": "success",
        "data": info,
    })))
}

/// GET /health
/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    let has_metrics = state.current_metrics.read().await.is_some();

    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "metrics_available": has_metrics,
    }))
}

/// GET /ws
/// WebSocket upgrade endpoint
async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| websocket::handle_socket(socket, state))
}

// ============================================================================
// Error Handling
// ============================================================================

/// Application error types
#[derive(Debug)]
enum AppError {
    BadRequest(String),
    NotFound(String),
    DatabaseError(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "status": "error",
            "message": message,
        }));

        (status, body).into_response()
    }
}
