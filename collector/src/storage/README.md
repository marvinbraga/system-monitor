# Storage Module

Database layer for the collector service, providing SQLite-based persistence for system metrics and anomalies.

## Structure

```
storage/
├── migrations/
│   └── 001_initial.sql    # Database schema definition
├── models.rs              # Database model structs
├── repository.rs          # Repository implementation
└── mod.rs                 # Module exports
```

## Features

### Tables

1. **metrics** - Stores system metrics with:
   - CPU usage (global and per-core)
   - Memory usage (RAM and swap)
   - Temperature readings (JSON)
   - Disk metrics (JSON)
   - USB devices (JSON)
   - Network statistics
   - Timestamps and indexes

2. **anomalies** - Stores detected anomalies with:
   - Severity levels (Info, Warning, Critical)
   - Categories (Cpu, Memory, Temperature, Disk, Usb, Network, System)
   - Detailed messages and metrics
   - Timestamps and indexes

3. **config** - Key-value configuration storage

### Repository Methods

#### Connection
- `new(database_url)` - Create repository and connect to database
- `run_migrations()` - Execute database migrations

#### Storage
- `store_metrics(&SystemMetrics)` - Store system metrics
- `store_anomaly(&Anomaly)` - Store anomaly

#### Retrieval
- `get_metrics_range(start, end)` - Get metrics in time range
- `get_anomalies_range(start, end)` - Get anomalies in time range
- `get_recent_metrics(limit)` - Get N most recent metrics
- `get_recent_anomalies(limit)` - Get N most recent anomalies

#### Configuration
- `set_config(key, value)` - Store/update configuration
- `get_config(key)` - Retrieve configuration value

#### Maintenance
- `cleanup_old_data(retention_days)` - Remove old data based on retention policy

## Usage

```rust
use collector::MetricsRepository;
use shared::types::SystemMetrics;

// Create repository
let repo = MetricsRepository::new("sqlite://metrics.db").await?;
repo.run_migrations().await?;

// Store metrics
let metrics = SystemMetrics { /* ... */ };
let id = repo.store_metrics(&metrics).await?;

// Query recent data
let recent = repo.get_recent_metrics(10).await?;

// Cleanup old data (keep 30 days)
let (deleted_metrics, deleted_anomalies) =
    repo.cleanup_old_data(30).await?;
```

## Examples

See `examples/` directory:
- `basic_usage.rs` - Basic CRUD operations
- `test_range_queries.rs` - Time range queries and cleanup

Run examples:
```bash
cargo run --package collector --example basic_usage
cargo run --package collector --example test_range_queries
```

## Database Schema

The schema is defined in `migrations/001_initial.sql` and includes:
- Appropriate indexes for performance
- CHECK constraints for data integrity
- JSON fields for complex nested data
- Timestamp tracking for all records

## Performance Considerations

1. **Indexes** - Created on frequently queried columns (timestamp, severity, category)
2. **Connection Pool** - Configurable pool size (default: 5 connections)
3. **Batch Operations** - Single transaction per operation
4. **JSON Storage** - Complex arrays stored as JSON for flexibility

## Error Handling

All repository methods return `Result<T, SqlxError>` for proper error handling.
Common errors:
- Database connection failures
- Constraint violations
- Serialization errors
- Query execution errors
