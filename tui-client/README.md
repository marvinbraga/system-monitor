# System Monitor TUI Client

Terminal User Interface (TUI) client for the system-monitor application built with [ratatui](https://github.com/ratatui-org/ratatui).

## Features

- **Real-time Monitoring**: Display live system metrics with configurable refresh rates
- **Rich Visualizations**:
  - CPU usage (global and per-core with color-coded gauges)
  - Memory usage (RAM and SWAP with progress bars)
  - Disk usage and I/O statistics
  - Temperature sensors
  - Anomaly detection alerts
- **Interactive Controls**: Navigate through anomalies with keyboard shortcuts
- **Color-coded Status**: Visual indicators for warning levels (green/yellow/red)
- **Responsive Layout**: Automatically adapts to terminal size

## Installation

Build the TUI client from the workspace root:

```bash
cargo build --release --package tui-client
```

The binary will be located at `target/release/tui-client`.

## Usage

### Basic Usage

Run with default settings (connects to `http://localhost:8080`, refreshes every 2 seconds):

```bash
./target/release/tui-client
```

### Command Line Options

```
USAGE:
    tui-client [OPTIONS]

OPTIONS:
    -u, --api-url <URL>        API base URL (default: http://localhost:8080)
    -r, --refresh <SECONDS>    Refresh rate in seconds (default: 2)
    -w, --websocket            Use WebSocket streaming instead of HTTP polling
    -h, --help                 Print help information

EXAMPLES:
    tui-client
    tui-client --api-url http://192.168.1.100:8080 --refresh 5
    tui-client -w -r 1
```

### Keyboard Controls

- `q` or `Ctrl+C`: Quit application
- `↑` / `k`: Scroll up in anomalies list
- `↓` / `j`: Scroll down in anomalies list
- `Page Up`: Scroll up one page in anomalies
- `Page Down`: Scroll down one page in anomalies
- `Home`: Jump to first anomaly
- `End`: Jump to most recent anomaly

## Dashboard Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│ System Monitor TUI | Status: Connected | Updated: 2026-01-29 12:30 │
├─────────────────────────────────┬───────────────────────────────────┤
│ ┌─ CPU ───────────────────────┐ │ ┌─ Temperatures ──────────────┐ │
│ │ Global: 45.2%               │ │ │ CPU Package: 52.0°C         │ │
│ │ Core 0: 42.1%               │ │ │ Core 0: 51.0°C              │ │
│ │ Core 1: 48.3%               │ │ │ Core 1: 53.0°C              │ │
│ │ ...                         │ │ │ ...                         │ │
│ │ Load: 1m: 1.50 5m: 1.20     │ │ └─────────────────────────────┘ │
│ └─────────────────────────────┘ │ ┌─ Anomalies (5) ─────────────┐ │
│ ┌─ Memory ────────────────────┐ │ │ ⚠ 12:30:15 [CPU] High usage │ │
│ │ RAM: 8.2/16.0 GB (51.2%)    │ │ │ ℹ 12:29:45 [Memory] Normal  │ │
│ │ SWAP: 0.0/4.0 GB (0.0%)     │ │ │ ✖ 12:28:30 [Disk] Critical  │ │
│ │ Available: 7.8 GB           │ │ │ ...                         │ │
│ └─────────────────────────────┘ │ └─────────────────────────────┘ │
│ ┌─ Disks ─────────────────────┐ │                                 │
│ │ /dev/sda1 (/)               │ │                                 │
│ │ 120.5 GB / 250.0 GB (48.2%) │ │                                 │
│ │ I/O: R: 1.2 MB  W: 3.4 MB   │ │                                 │
│ └─────────────────────────────┘ │                                 │
└─────────────────────────────────┴───────────────────────────────────┘
```

## Architecture

The TUI client is organized into the following modules:

- **main.rs**: Entry point, terminal setup, event loop
- **api_client.rs**: HTTP client for communicating with the API server
- **config.rs**: Configuration parsing and defaults
- **ui/dashboard.rs**: Main dashboard layout and rendering
- **ui/cpu_view.rs**: CPU metrics visualization
- **ui/memory_view.rs**: Memory metrics visualization
- **ui/disk_view.rs**: Disk metrics visualization
- **ui/anomalies_view.rs**: Anomaly list with scrolling

## Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **tokio**: Async runtime
- **reqwest**: HTTP client
- **serde/serde_json**: Serialization
- **chrono**: Date/time handling
- **anyhow**: Error handling

## Development

### Running in Development

```bash
# From workspace root
cargo run --package tui-client

# With custom options
cargo run --package tui-client -- --api-url http://localhost:8080 --refresh 1
```

### Testing

```bash
cargo test --package tui-client
```

## Troubleshooting

### Connection Issues

If the TUI shows "Error: ..." in the status:

1. Ensure the API server is running on the specified URL
2. Check firewall settings
3. Verify the URL is correct (include `http://` or `https://`)

### Display Issues

If the TUI layout looks broken:

1. Resize your terminal window (minimum 80x24 recommended)
2. Ensure your terminal supports Unicode characters
3. Try a different terminal emulator

### High CPU Usage

If the TUI uses too much CPU:

1. Increase the refresh rate: `tui-client --refresh 5`
2. Close other terminal applications
3. Use a lighter terminal emulator

## License

Part of the system-monitor project.
