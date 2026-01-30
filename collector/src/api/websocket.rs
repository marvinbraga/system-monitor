/// WebSocket handler for real-time metrics streaming
///
/// Provides WebSocket connection handling for streaming system metrics
/// to connected clients in real-time.
use axum::extract::ws::{Message, WebSocket};
use serde_json::json;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};

use super::AppState;

/// Handles a WebSocket connection
///
/// This function is called when a WebSocket connection is successfully upgraded.
/// It sends the current system metrics to the client every 2 seconds until
/// the connection is closed or an error occurs.
///
/// # Arguments
/// * `socket` - The WebSocket connection
/// * `state` - Shared application state containing current metrics
pub async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("New WebSocket connection established");

    // Create a ticker that fires every 2 seconds
    let mut ticker = interval(Duration::from_secs(2));

    loop {
        tokio::select! {
            // Send metrics every 2 seconds
            _ = ticker.tick() => {
                // Read current metrics
                let metrics = {
                    let metrics_guard = state.current_metrics.read().await;
                    metrics_guard.clone()
                };

                // Prepare message
                let message = match metrics {
                    Some(m) => {
                        // Serialize metrics to JSON
                        match serde_json::to_string(&json!({
                            "type": "metrics",
                            "data": m,
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                        })) {
                            Ok(json_str) => Message::Text(json_str),
                            Err(e) => {
                                error!("Failed to serialize metrics: {}", e);
                                continue;
                            }
                        }
                    }
                    None => {
                        // No metrics available yet
                        debug!("No metrics available for WebSocket client");
                        match serde_json::to_string(&json!({
                            "type": "info",
                            "message": "No metrics available yet",
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                        })) {
                            Ok(json_str) => Message::Text(json_str),
                            Err(e) => {
                                error!("Failed to serialize info message: {}", e);
                                continue;
                            }
                        }
                    }
                };

                // Send message to client
                if let Err(e) = socket.send(message).await {
                    warn!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }

            // Handle incoming messages from client
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) => {
                        info!("WebSocket client closed connection");
                        break;
                    }
                    Some(Ok(Message::Ping(data))) => {
                        debug!("Received ping, sending pong");
                        if let Err(e) = socket.send(Message::Pong(data)).await {
                            warn!("Failed to send pong: {}", e);
                            break;
                        }
                    }
                    Some(Ok(Message::Text(text))) => {
                        // Log any text messages from client (could be used for commands)
                        debug!("Received text message from client: {}", text);
                    }
                    Some(Ok(Message::Binary(_))) => {
                        debug!("Received binary message from client (ignored)");
                    }
                    Some(Ok(Message::Pong(_))) => {
                        debug!("Received pong from client");
                    }
                    Some(Err(e)) => {
                        warn!("Error receiving WebSocket message: {}", e);
                        break;
                    }
                    None => {
                        info!("WebSocket connection closed by client");
                        break;
                    }
                }
            }
        }
    }

    info!("WebSocket connection closed");
}

#[cfg(test)]
mod tests {
    use super::*;

    use shared::types::{CpuMetrics, MemoryMetrics, NetworkMetrics, SystemMetrics};

    fn create_test_metrics() -> SystemMetrics {
        SystemMetrics {
            timestamp: chrono::Utc::now(),
            cpu: CpuMetrics {
                global_usage: 50.0,
                per_core: vec![45.0, 55.0, 48.0, 52.0],
                load_avg_1: 1.5,
                load_avg_5: 1.2,
                load_avg_15: 1.0,
            },
            memory: MemoryMetrics {
                total: 16_000_000_000,
                used: 8_000_000_000,
                available: 8_000_000_000,
                usage_percent: 50.0,
                swap_total: 4_000_000_000,
                swap_used: 0,
            },
            temperatures: vec![],
            disks: vec![],
            usb_devices: vec![],
            network: NetworkMetrics {
                rx_bytes: 1_000_000,
                tx_bytes: 500_000,
                rx_packets: 10_000,
                tx_packets: 5_000,
            },
            gpu: None,
        }
    }

    #[tokio::test]
    async fn test_message_serialization() {
        let metrics = create_test_metrics();
        let json_result = serde_json::to_string(&json!({
            "type": "metrics",
            "data": metrics,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        assert!(json_result.is_ok());
    }
}
