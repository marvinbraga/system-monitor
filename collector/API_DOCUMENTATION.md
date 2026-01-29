# Collector API Documentation

The collector service provides a REST API and WebSocket server for accessing system metrics and anomalies.

## Quick Start

### Running the Example Server

```bash
cargo run --example api_server
```

This will start the server on `http://127.0.0.1:8080` with:
- Background metrics collection every 5 seconds
- Automatic anomaly detection
- All API endpoints enabled
- WebSocket support for real-time streaming

## API Endpoints

### Health Check

**GET** `/health`

Returns server health status and indicates if metrics are available.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-29T12:00:00Z",
  "metrics_available": true
}
```

---

### Current Metrics

**GET** `/api/v1/metrics/current`

Returns the most recent system metrics.

**Response:**
```json
{
  "status": "success",
  "data": {
    "timestamp": "2024-01-29T12:00:00Z",
    "cpu": {
      "global_usage": 45.2,
      "per_core": [42.1, 48.3, 44.5, 46.0],
      "load_avg_1": 1.5,
      "load_avg_5": 1.2,
      "load_avg_15": 1.0
    },
    "memory": {
      "total": 16000000000,
      "used": 8000000000,
      "available": 8000000000,
      "usage_percent": 50.0,
      "swap_total": 4000000000,
      "swap_used": 0
    },
    "temperatures": [...],
    "disks": [...],
    "usb_devices": [...],
    "network": {...}
  }
}
```

---

### Metrics History

**GET** `/api/v1/metrics/history?start=&end=&limit=`

Returns historical metrics within a time range.

**Query Parameters:**
- `start` (optional): Start timestamp in RFC3339 format (defaults to 1 hour ago)
- `end` (optional): End timestamp in RFC3339 format (defaults to now)
- `limit` (optional): Maximum number of records to return (overrides time range)

**Example:**
```bash
curl "http://localhost:8080/api/v1/metrics/history?limit=10"
curl "http://localhost:8080/api/v1/metrics/history?start=2024-01-29T10:00:00Z&end=2024-01-29T12:00:00Z"
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "metrics": [...],
    "count": 10,
    "start": "2024-01-29T11:00:00Z",
    "end": "2024-01-29T12:00:00Z"
  }
}
```

---

### Anomalies List

**GET** `/api/v1/anomalies?start=&end=&severity=&limit=`

Returns detected anomalies with optional filtering.

**Query Parameters:**
- `start` (optional): Start timestamp in RFC3339 format (defaults to 24 hours ago)
- `end` (optional): End timestamp in RFC3339 format (defaults to now)
- `severity` (optional): Filter by severity: `info`, `warning`, or `critical`
- `limit` (optional): Maximum number of records to return

**Example:**
```bash
curl "http://localhost:8080/api/v1/anomalies?severity=warning&limit=5"
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "anomalies": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "timestamp": "2024-01-29T12:00:00Z",
        "severity": "Warning",
        "category": "Cpu",
        "message": "High CPU usage detected: 75.50%",
        "metrics": {
          "cpu_usage": 75.5,
          "load_avg": 2.1
        }
      }
    ],
    "count": 1,
    "start": "2024-01-28T12:00:00Z",
    "end": "2024-01-29T12:00:00Z"
  }
}
```

---

### Specific Anomaly

**GET** `/api/v1/anomalies/:id`

Returns details for a specific anomaly by ID.

**Example:**
```bash
curl "http://localhost:8080/api/v1/anomalies/123e4567-e89b-12d3-a456-426614174000"
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "timestamp": "2024-01-29T12:00:00Z",
    "severity": "Warning",
    "category": "Cpu",
    "message": "High CPU usage detected: 75.50%",
    "metrics": {...}
  }
}
```

---

### System Information

**GET** `/api/v1/system/info`

Returns general system information.

**Response:**
```json
{
  "status": "success",
  "data": {
    "hostname": "my-computer",
    "os": "Linux",
    "kernel_version": "6.14.0-37-generic",
    "uptime": 123456,
    "cpu_count": 4
  }
}
```

---

## WebSocket Streaming

**WS** `/ws`

Connect to this endpoint for real-time metrics streaming.

**Connection:**
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};
```

**Message Format:**
```json
{
  "type": "metrics",
  "data": {
    "timestamp": "2024-01-29T12:00:00Z",
    "cpu": {...},
    "memory": {...},
    ...
  },
  "timestamp": "2024-01-29T12:00:00Z"
}
```

Messages are sent every **2 seconds** with the latest system metrics.

---

## Server Configuration

### Basic Configuration

```rust
use collector::{ServerConfig, start_server, AppState, MetricsRepository};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize database
    let repo = Arc::new(MetricsRepository::new("sqlite:./metrics.db").await?);
    repo.run_migrations().await?;

    // Create shared state
    let current_metrics = Arc::new(RwLock::new(None));
    let recent_anomalies = Arc::new(RwLock::new(Vec::new()));

    // Configure server
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        enable_cors: true,
        cors_origins: vec!["http://localhost:3000".to_string()],
    };

    // Start server
    start_server(config, current_metrics, recent_anomalies, repo).await?;

    Ok(())
}
```

### Configuration Options

```rust
pub struct ServerConfig {
    /// Host address (e.g., "0.0.0.0" or "127.0.0.1")
    pub host: String,

    /// Port number
    pub port: u16,

    /// Enable CORS
    pub enable_cors: bool,

    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
}
```

**Default Configuration:**
```rust
let config = ServerConfig::default();
// host: "127.0.0.1"
// port: 8080
// enable_cors: true
// cors_origins: ["http://localhost:3000", "http://127.0.0.1:3000"]
```

---

## Error Handling

All endpoints return errors in a consistent format:

```json
{
  "status": "error",
  "message": "Description of what went wrong"
}
```

**HTTP Status Codes:**
- `200 OK` - Successful request
- `400 Bad Request` - Invalid query parameters or request format
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server-side error (database, serialization, etc.)

---

## Testing with curl

```bash
# Health check
curl http://localhost:8080/health

# Get current metrics
curl http://localhost:8080/api/v1/metrics/current

# Get last 10 metrics records
curl "http://localhost:8080/api/v1/metrics/history?limit=10"

# Get anomalies from last 24 hours
curl http://localhost:8080/api/v1/anomalies

# Get critical anomalies only
curl "http://localhost:8080/api/v1/anomalies?severity=critical"

# Get system info
curl http://localhost:8080/api/v1/system/info
```

---

## Testing WebSocket with websocat

```bash
# Install websocat
cargo install websocat

# Connect to WebSocket endpoint
websocat ws://localhost:8080/ws
```

You'll receive metrics updates every 2 seconds.

---

## Architecture

### State Management

The API uses shared state through `Arc<RwLock<T>>` for thread-safe access:

- **current_metrics**: Latest collected metrics (updated by collector)
- **recent_anomalies**: In-memory buffer of recent anomalies (max 100)
- **repository**: Database access for historical data

### Middleware

- **CORS**: Configurable cross-origin resource sharing
- **Tracing**: Request/response logging with `tower-http`
- **Graceful Shutdown**: Handles SIGINT (Ctrl+C) and SIGTERM

### WebSocket Behavior

- Sends metrics every 2 seconds
- Handles ping/pong automatically
- Cleans up on client disconnect
- Concurrent message handling with `tokio::select!`

---

## Production Deployment

### Binding to All Interfaces

```rust
let config = ServerConfig {
    host: "0.0.0.0".to_string(),  // Listen on all interfaces
    port: 8080,
    enable_cors: true,
    cors_origins: vec!["https://your-domain.com".to_string()],
};
```

### Environment Variables

```bash
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="8080"
export DATABASE_URL="sqlite:./metrics.db"
```

### Reverse Proxy (nginx)

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
```

---

## Dependencies

The API module uses:
- **axum 0.7** - Web framework with WebSocket support
- **tower-http** - CORS and tracing middleware
- **tokio** - Async runtime
- **serde/serde_json** - JSON serialization
- **chrono** - Timestamp handling

Add to your `Cargo.toml`:
```toml
axum = { version = "0.7", features = ["ws"] }
tower-http = { version = "0.5", features = ["cors", "trace"] }
tokio = { version = "1.35", features = ["full"] }
```
