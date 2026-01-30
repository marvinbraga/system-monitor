-- System Metrics Table
-- Stores all collected system metrics with timestamp
CREATE TABLE IF NOT EXISTS metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL,

    -- CPU metrics
    cpu_global REAL NOT NULL,
    cpu_per_core TEXT NOT NULL,  -- JSON array of per-core percentages
    load_avg_1 REAL NOT NULL,
    load_avg_5 REAL NOT NULL,
    load_avg_15 REAL NOT NULL,

    -- Memory metrics
    memory_total INTEGER NOT NULL,
    memory_used INTEGER NOT NULL,
    memory_available INTEGER NOT NULL,
    memory_percent REAL NOT NULL,
    swap_total INTEGER NOT NULL,
    swap_used INTEGER NOT NULL,

    -- Complex metrics stored as JSON
    temperatures TEXT,  -- JSON array of temperature sensors
    disks TEXT,        -- JSON array of disk metrics
    usb_devices TEXT,  -- JSON array of USB devices
    gpu TEXT,          -- JSON object of GPU metrics (optional)

    -- Network metrics
    network_rx INTEGER NOT NULL,
    network_tx INTEGER NOT NULL,
    network_rx_packets INTEGER NOT NULL DEFAULT 0,
    network_tx_packets INTEGER NOT NULL DEFAULT 0,

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Anomalies Table
-- Stores detected anomalies with severity and category
CREATE TABLE IF NOT EXISTS anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL,
    severity TEXT NOT NULL CHECK(severity IN ('Info', 'Warning', 'Critical')),
    category TEXT NOT NULL CHECK(category IN ('Cpu', 'Memory', 'Temperature', 'Disk', 'Usb', 'Network', 'Gpu', 'System')),
    message TEXT NOT NULL,
    metrics TEXT NOT NULL,  -- JSON object with relevant metrics
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Configuration Table
-- Stores key-value configuration pairs
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_created_at ON metrics(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_anomalies_timestamp ON anomalies(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_anomalies_severity ON anomalies(severity);
CREATE INDEX IF NOT EXISTS idx_anomalies_category ON anomalies(category);
CREATE INDEX IF NOT EXISTS idx_anomalies_created_at ON anomalies(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_config_updated_at ON config(updated_at DESC);
