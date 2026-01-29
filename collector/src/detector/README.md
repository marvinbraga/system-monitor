# Anomaly Detection System

This module provides comprehensive anomaly detection for the system monitor collector service, migrated from the original `main.rs` implementation.

## Overview

The detector module consists of three main components:

1. **`AnomalyRules`** - Main detection engine with stateful rule checking
2. **`analyzer`** - Helper functions for metric analysis and comparison
3. **`mod.rs`** - Module exports

## Usage Example

```rust
use collector::AnomalyRules;
use shared::types::SystemMetrics;

// Initialize the detector with the number of CPUs
let mut detector = AnomalyRules::new(8);

// Collect metrics (using your metrics collector)
let metrics = collect_system_metrics();

// Check for anomalies
let anomalies = detector.check(&metrics);

// Process detected anomalies
for anomaly in anomalies {
    println!("[{}] {} - {}",
        anomaly.severity,
        anomaly.category,
        anomaly.message
    );
}
```

## Detection Rules

### CPU Anomalies

1. **CPU Spike** (Warning)
   - Trigger: CPU usage increases by >40% between intervals
   - Example: 30% → 75%

2. **Sustained Critical CPU** (Critical)
   - Trigger: CPU usage >90% for two consecutive measurements
   - Example: 91% → 92%

3. **High Load Average** (Critical)
   - Trigger: 15-minute load average > (2 × number of CPUs)
   - Example: 16.5 on an 8-core system

### Memory Anomalies

1. **Memory Spike** (Warning)
   - Trigger: Memory usage increases by >20% between intervals
   - Example: 60% → 85%

2. **Critical Memory** (Critical)
   - Trigger: Memory usage >95%
   - Example: 96.5%

3. **SWAP Activation** (Warning)
   - Trigger: SWAP usage goes from 0 to >0
   - Indicates system is running low on physical RAM

### Temperature Anomalies

1. **Critical Temperature** (Critical)
   - Trigger: Temperature crosses 85°C threshold
   - Monitors all sensors (CPU, NVMe, etc.)

2. **Temperature Drop** (Warning)
   - Trigger: Temperature drops >30°C between intervals
   - May indicate sensor issues or sudden cooling

### Disk Anomalies

1. **Critical Disk Usage** (Warning)
   - Trigger: Any disk >90% full
   - Example: 92.5% on /dev/sda

2. **High Disk I/O** (Warning)
   - Trigger: Combined read+write >500 MB/s
   - Example: 520 MB/s

### USB Anomalies

1. **USB Timeout** (Critical)
   - Trigger: Any USB device reports a timeout
   - Indicates potential hardware issues

## Module Structure

### `rules.rs`

Contains the `AnomalyRules` struct with:

```rust
pub struct AnomalyRules {
    previous_metrics: Option<SystemMetrics>,
    num_cpus: usize,
}

impl AnomalyRules {
    pub fn new(num_cpus: usize) -> Self;
    pub fn check(&mut self, current: &SystemMetrics) -> Vec<Anomaly>;
    pub fn reset(&mut self);
    pub fn has_previous_metrics(&self) -> bool;
}
```

**Key Features:**
- Stateful comparison between current and previous metrics
- Automatic state management
- Returns structured `Anomaly` objects with metadata

### `analyzer.rs`

Helper functions for metric analysis:

```rust
// Calculate delta between measurements
pub fn calculate_delta(current: &SystemMetrics, previous: &SystemMetrics) -> MetricsDelta;

// Calculate rate of change per second
pub fn calculate_rate(delta: f64, time_interval_secs: f64) -> f64;

// Classify severity based on thresholds
pub fn classify_severity(value: f32, warning: f32, critical: f32) -> Option<AnomalySeverity>;

// Metric extractors
pub fn get_max_temperature(metrics: &SystemMetrics) -> f32;
pub fn get_max_disk_usage(metrics: &SystemMetrics) -> f32;
pub fn get_max_disk_io(metrics: &SystemMetrics) -> f64;
pub fn has_usb_timeout(metrics: &SystemMetrics) -> bool;
```

## Threshold Constants

All thresholds are defined in `rules.rs`:

```rust
const CPU_SPIKE_THRESHOLD: f32 = 40.0;          // % increase
const CPU_CRITICAL_THRESHOLD: f32 = 90.0;       // % usage
const MEMORY_SPIKE_THRESHOLD: f32 = 20.0;       // % increase
const MEMORY_CRITICAL_THRESHOLD: f32 = 95.0;    // % usage
const TEMPERATURE_CRITICAL: f32 = 85.0;         // °C
const TEMPERATURE_DROP_THRESHOLD: f32 = 30.0;   // °C decrease
const DISK_CRITICAL_THRESHOLD: f32 = 90.0;      // % usage
const DISK_IO_HIGH_THRESHOLD: f64 = 500.0;      // MB/s
const LOAD_AVG_MULTIPLIER: f64 = 2.0;           // times number of CPUs
```

## Anomaly Output Format

Each detected anomaly includes:

```rust
pub struct Anomaly {
    pub id: String,                    // UUID
    pub timestamp: DateTime<Utc>,       // When detected
    pub severity: AnomalySeverity,      // Info/Warning/Critical
    pub category: AnomalyCategory,      // Cpu/Memory/Temperature/Disk/Usb/Network/System
    pub message: String,                // Human-readable description
    pub metrics: serde_json::Value,     // Structured metadata
}
```

### Example Anomaly

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2026-01-29T12:34:56Z",
  "severity": "Warning",
  "category": "Cpu",
  "message": "CPU spike detected: 30% → 75%",
  "metrics": {
    "previous": 30.0,
    "current": 75.0,
    "delta": 45.0
  }
}
```

## Testing

Run the detector tests with:

```bash
cargo test --package collector --lib detector
```

The test suite includes:
- CPU spike detection
- Memory critical thresholds
- SWAP activation
- Temperature thresholds
- State management (reset, previous metrics)
- Delta calculation
- Rate calculation
- Severity classification

## Migration Notes

This implementation migrates all anomaly detection logic from the original `src/main.rs`:

| Original Code | New Location | Notes |
|---------------|--------------|-------|
| `struct AnomalyDetector` | `AnomalyRules` | Renamed, logging separated |
| `check_anomalies()` | `check()` | Split into comparative/absolute checks |
| Inline threshold checks | `rules.rs` constants | Centralized configuration |
| Ad-hoc delta calculations | `analyzer::calculate_delta()` | Reusable utility |

### Key Improvements

1. **Separation of Concerns**: Detection logic separated from logging/display
2. **Testability**: All rules have unit tests
3. **Reusability**: Helper functions in analyzer module
4. **Type Safety**: Uses shared types (`Anomaly`, `AnomalySeverity`, etc.)
5. **Metadata**: Each anomaly includes structured metrics for analysis
6. **Modularity**: Easy to add new detection rules

## Future Enhancements

Potential improvements:

1. **Configurable Thresholds**: Load from config file or database
2. **Machine Learning**: Anomaly detection based on historical patterns
3. **Anomaly Correlation**: Detect related anomalies across categories
4. **Adaptive Thresholds**: Adjust based on system characteristics
5. **Process-Level Detection**: OOM and segfault tracking (requires kernel log parsing)
6. **Network Anomalies**: Rate-based detection for network traffic spikes

## Dependencies

- `shared` - Common types (`SystemMetrics`, `Anomaly`, etc.)
- `chrono` - Timestamp handling
- `uuid` - Unique anomaly IDs
- `serde_json` - Metadata serialization
