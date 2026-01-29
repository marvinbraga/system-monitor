use chrono::{DateTime, Duration, Utc};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::{Error as SqlxError, Row};
use std::str::FromStr;

use shared::types::{Anomaly, AnomalyCategory, AnomalySeverity, SystemMetrics};

use super::models::{AnomalyRow, MetricsRow, NewAnomaly, NewMetrics};

/// Repository for managing system metrics and anomalies in SQLite database
pub struct MetricsRepository {
    pool: SqlitePool,
}

impl MetricsRepository {
    /// Creates a new MetricsRepository and establishes connection to SQLite database
    ///
    /// # Arguments
    /// * `database_url` - SQLite database URL (e.g., "sqlite://metrics.db")
    ///
    /// # Returns
    /// Result containing MetricsRepository or SqlxError
    pub async fn new(database_url: &str) -> Result<Self, SqlxError> {
        // Parse the database URL and enable create_if_missing
        let options = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true)
            .foreign_keys(true);

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Runs database migrations to set up the schema
    ///
    /// # Returns
    /// Result indicating success or SqlxError
    pub async fn run_migrations(&self) -> Result<(), SqlxError> {
        // Read and execute the migration SQL
        let migration_sql = include_str!("migrations/001_initial.sql");

        sqlx::query(migration_sql).execute(&self.pool).await?;

        Ok(())
    }

    /// Stores system metrics in the database
    ///
    /// # Arguments
    /// * `metrics` - SystemMetrics structure to store
    ///
    /// # Returns
    /// Result containing the inserted row ID or SqlxError
    pub async fn store_metrics(&self, metrics: &SystemMetrics) -> Result<i64, SqlxError> {
        // Convert SystemMetrics to NewMetrics format
        let new_metrics = self.convert_to_new_metrics(metrics)?;

        let result = sqlx::query(
            r#"
            INSERT INTO metrics (
                timestamp,
                cpu_global,
                cpu_per_core,
                load_avg_1,
                load_avg_5,
                load_avg_15,
                memory_total,
                memory_used,
                memory_available,
                memory_percent,
                swap_total,
                swap_used,
                temperatures,
                disks,
                usb_devices,
                network_rx,
                network_tx,
                network_rx_packets,
                network_tx_packets
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_metrics.timestamp.to_rfc3339())
        .bind(new_metrics.cpu_global)
        .bind(&new_metrics.cpu_per_core)
        .bind(new_metrics.load_avg_1)
        .bind(new_metrics.load_avg_5)
        .bind(new_metrics.load_avg_15)
        .bind(new_metrics.memory_total)
        .bind(new_metrics.memory_used)
        .bind(new_metrics.memory_available)
        .bind(new_metrics.memory_percent)
        .bind(new_metrics.swap_total)
        .bind(new_metrics.swap_used)
        .bind(&new_metrics.temperatures)
        .bind(&new_metrics.disks)
        .bind(&new_metrics.usb_devices)
        .bind(new_metrics.network_rx)
        .bind(new_metrics.network_tx)
        .bind(new_metrics.network_rx_packets)
        .bind(new_metrics.network_tx_packets)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Stores an anomaly in the database
    ///
    /// # Arguments
    /// * `anomaly` - Anomaly structure to store
    ///
    /// # Returns
    /// Result containing the inserted row ID or SqlxError
    pub async fn store_anomaly(&self, anomaly: &Anomaly) -> Result<i64, SqlxError> {
        let new_anomaly = self.convert_to_new_anomaly(anomaly)?;

        let result = sqlx::query(
            r#"
            INSERT INTO anomalies (
                timestamp,
                severity,
                category,
                message,
                metrics
            ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_anomaly.timestamp.to_rfc3339())
        .bind(&new_anomaly.severity)
        .bind(&new_anomaly.category)
        .bind(&new_anomaly.message)
        .bind(&new_anomaly.metrics)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Retrieves metrics within a time range
    ///
    /// # Arguments
    /// * `start` - Start of time range
    /// * `end` - End of time range
    ///
    /// # Returns
    /// Result containing Vec of SystemMetrics or SqlxError
    pub async fn get_metrics_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<SystemMetrics>, SqlxError> {
        let rows = sqlx::query_as::<_, MetricsRow>(
            r#"
            SELECT * FROM metrics
            WHERE timestamp BETWEEN ? AND ?
            ORDER BY timestamp ASC
            "#,
        )
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;

        let metrics: Result<Vec<SystemMetrics>, SqlxError> = rows
            .into_iter()
            .map(|row| self.convert_from_metrics_row(row))
            .collect();

        metrics
    }

    /// Retrieves anomalies within a time range
    ///
    /// # Arguments
    /// * `start` - Start of time range
    /// * `end` - End of time range
    ///
    /// # Returns
    /// Result containing Vec of Anomaly or SqlxError
    pub async fn get_anomalies_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Anomaly>, SqlxError> {
        let rows = sqlx::query_as::<_, AnomalyRow>(
            r#"
            SELECT * FROM anomalies
            WHERE timestamp BETWEEN ? AND ?
            ORDER BY timestamp ASC
            "#,
        )
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;

        let anomalies: Result<Vec<Anomaly>, SqlxError> = rows
            .into_iter()
            .map(|row| self.convert_from_anomaly_row(row))
            .collect();

        anomalies
    }

    /// Retrieves recent metrics with a limit
    ///
    /// # Arguments
    /// * `limit` - Maximum number of records to retrieve
    ///
    /// # Returns
    /// Result containing Vec of SystemMetrics or SqlxError
    pub async fn get_recent_metrics(&self, limit: i64) -> Result<Vec<SystemMetrics>, SqlxError> {
        let rows = sqlx::query_as::<_, MetricsRow>(
            r#"
            SELECT * FROM metrics
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let metrics: Result<Vec<SystemMetrics>, SqlxError> = rows
            .into_iter()
            .map(|row| self.convert_from_metrics_row(row))
            .collect();

        metrics
    }

    /// Retrieves recent anomalies with a limit
    ///
    /// # Arguments
    /// * `limit` - Maximum number of records to retrieve
    ///
    /// # Returns
    /// Result containing Vec of Anomaly or SqlxError
    pub async fn get_recent_anomalies(&self, limit: i64) -> Result<Vec<Anomaly>, SqlxError> {
        let rows = sqlx::query_as::<_, AnomalyRow>(
            r#"
            SELECT * FROM anomalies
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let anomalies: Result<Vec<Anomaly>, SqlxError> = rows
            .into_iter()
            .map(|row| self.convert_from_anomaly_row(row))
            .collect();

        anomalies
    }

    /// Removes old data based on retention period
    ///
    /// # Arguments
    /// * `retention_days` - Number of days to retain data
    ///
    /// # Returns
    /// Result containing tuple of (deleted_metrics_count, deleted_anomalies_count) or SqlxError
    pub async fn cleanup_old_data(&self, retention_days: i64) -> Result<(u64, u64), SqlxError> {
        let cutoff_date = Utc::now() - Duration::days(retention_days);

        // Delete old metrics
        let metrics_result = sqlx::query(
            r#"
            DELETE FROM metrics
            WHERE timestamp < ?
            "#,
        )
        .bind(cutoff_date.to_rfc3339())
        .execute(&self.pool)
        .await?;

        // Delete old anomalies
        let anomalies_result = sqlx::query(
            r#"
            DELETE FROM anomalies
            WHERE timestamp < ?
            "#,
        )
        .bind(cutoff_date.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok((
            metrics_result.rows_affected(),
            anomalies_result.rows_affected(),
        ))
    }

    /// Stores or updates a configuration value
    ///
    /// # Arguments
    /// * `key` - Configuration key
    /// * `value` - Configuration value
    ///
    /// # Returns
    /// Result indicating success or SqlxError
    pub async fn set_config(&self, key: &str, value: &str) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            INSERT INTO config (key, value, updated_at)
            VALUES (?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Retrieves a configuration value
    ///
    /// # Arguments
    /// * `key` - Configuration key
    ///
    /// # Returns
    /// Result containing Option<String> with the value or SqlxError
    pub async fn get_config(&self, key: &str) -> Result<Option<String>, SqlxError> {
        let result = sqlx::query(
            r#"
            SELECT value FROM config WHERE key = ?
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| row.get("value")))
    }

    // Helper methods for conversion

    fn convert_to_new_metrics(&self, metrics: &SystemMetrics) -> Result<NewMetrics, SqlxError> {
        Ok(NewMetrics {
            timestamp: metrics.timestamp,
            cpu_global: metrics.cpu.global_usage,
            cpu_per_core: serde_json::to_string(&metrics.cpu.per_core).map_err(|e| {
                SqlxError::Protocol(format!("Failed to serialize cpu_per_core: {}", e))
            })?,
            load_avg_1: metrics.cpu.load_avg_1,
            load_avg_5: metrics.cpu.load_avg_5,
            load_avg_15: metrics.cpu.load_avg_15,
            memory_total: metrics.memory.total as i64,
            memory_used: metrics.memory.used as i64,
            memory_available: metrics.memory.available as i64,
            memory_percent: metrics.memory.usage_percent,
            swap_total: metrics.memory.swap_total as i64,
            swap_used: metrics.memory.swap_used as i64,
            temperatures: if metrics.temperatures.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&metrics.temperatures).map_err(|e| {
                    SqlxError::Protocol(format!("Failed to serialize temperatures: {}", e))
                })?)
            },
            disks: if metrics.disks.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&metrics.disks).map_err(|e| {
                    SqlxError::Protocol(format!("Failed to serialize disks: {}", e))
                })?)
            },
            usb_devices: if metrics.usb_devices.is_empty() {
                None
            } else {
                Some(serde_json::to_string(&metrics.usb_devices).map_err(|e| {
                    SqlxError::Protocol(format!("Failed to serialize usb_devices: {}", e))
                })?)
            },
            network_rx: metrics.network.rx_bytes as i64,
            network_tx: metrics.network.tx_bytes as i64,
            network_rx_packets: metrics.network.rx_packets as i64,
            network_tx_packets: metrics.network.tx_packets as i64,
        })
    }

    fn convert_to_new_anomaly(&self, anomaly: &Anomaly) -> Result<NewAnomaly, SqlxError> {
        let severity = match anomaly.severity {
            AnomalySeverity::Info => "Info",
            AnomalySeverity::Warning => "Warning",
            AnomalySeverity::Critical => "Critical",
        };

        let category = match anomaly.category {
            AnomalyCategory::Cpu => "Cpu",
            AnomalyCategory::Memory => "Memory",
            AnomalyCategory::Temperature => "Temperature",
            AnomalyCategory::Disk => "Disk",
            AnomalyCategory::Usb => "Usb",
            AnomalyCategory::Network => "Network",
            AnomalyCategory::System => "System",
        };

        Ok(NewAnomaly {
            timestamp: anomaly.timestamp,
            severity: severity.to_string(),
            category: category.to_string(),
            message: anomaly.message.clone(),
            metrics: serde_json::to_string(&anomaly.metrics)
                .map_err(|e| SqlxError::Protocol(format!("Failed to serialize metrics: {}", e)))?,
        })
    }

    fn convert_from_metrics_row(&self, row: MetricsRow) -> Result<SystemMetrics, SqlxError> {
        use shared::types::{
            CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics, Temperature, UsbDevice,
        };

        let timestamp = DateTime::parse_from_rfc3339(&row.timestamp)
            .map_err(|e| SqlxError::Protocol(format!("Failed to parse timestamp: {}", e)))?
            .with_timezone(&Utc);

        let cpu_per_core: Vec<f32> = serde_json::from_str(&row.cpu_per_core).map_err(|e| {
            SqlxError::Protocol(format!("Failed to deserialize cpu_per_core: {}", e))
        })?;

        let temperatures: Vec<Temperature> = if let Some(temps_json) = row.temperatures {
            serde_json::from_str(&temps_json).map_err(|e| {
                SqlxError::Protocol(format!("Failed to deserialize temperatures: {}", e))
            })?
        } else {
            Vec::new()
        };

        let disks: Vec<DiskMetrics> = if let Some(disks_json) = row.disks {
            serde_json::from_str(&disks_json)
                .map_err(|e| SqlxError::Protocol(format!("Failed to deserialize disks: {}", e)))?
        } else {
            Vec::new()
        };

        let usb_devices: Vec<UsbDevice> = if let Some(usb_json) = row.usb_devices {
            serde_json::from_str(&usb_json).map_err(|e| {
                SqlxError::Protocol(format!("Failed to deserialize usb_devices: {}", e))
            })?
        } else {
            Vec::new()
        };

        Ok(SystemMetrics {
            timestamp,
            cpu: CpuMetrics {
                global_usage: row.cpu_global,
                per_core: cpu_per_core,
                load_avg_1: row.load_avg_1,
                load_avg_5: row.load_avg_5,
                load_avg_15: row.load_avg_15,
            },
            memory: MemoryMetrics {
                total: row.memory_total as u64,
                used: row.memory_used as u64,
                available: row.memory_available as u64,
                usage_percent: row.memory_percent,
                swap_total: row.swap_total as u64,
                swap_used: row.swap_used as u64,
            },
            temperatures,
            disks,
            usb_devices,
            network: NetworkMetrics {
                rx_bytes: row.network_rx as u64,
                tx_bytes: row.network_tx as u64,
                rx_packets: row.network_rx_packets as u64,
                tx_packets: row.network_tx_packets as u64,
            },
        })
    }

    fn convert_from_anomaly_row(&self, row: AnomalyRow) -> Result<Anomaly, SqlxError> {
        let timestamp = DateTime::parse_from_rfc3339(&row.timestamp)
            .map_err(|e| SqlxError::Protocol(format!("Failed to parse timestamp: {}", e)))?
            .with_timezone(&Utc);

        let severity = match row.severity.as_str() {
            "Info" => AnomalySeverity::Info,
            "Warning" => AnomalySeverity::Warning,
            "Critical" => AnomalySeverity::Critical,
            _ => {
                return Err(SqlxError::Protocol(format!(
                    "Invalid severity: {}",
                    row.severity
                )))
            }
        };

        let category = match row.category.as_str() {
            "Cpu" => AnomalyCategory::Cpu,
            "Memory" => AnomalyCategory::Memory,
            "Temperature" => AnomalyCategory::Temperature,
            "Disk" => AnomalyCategory::Disk,
            "Usb" => AnomalyCategory::Usb,
            "Network" => AnomalyCategory::Network,
            "System" => AnomalyCategory::System,
            _ => {
                return Err(SqlxError::Protocol(format!(
                    "Invalid category: {}",
                    row.category
                )))
            }
        };

        let metrics: serde_json::Value = serde_json::from_str(&row.metrics)
            .map_err(|e| SqlxError::Protocol(format!("Failed to deserialize metrics: {}", e)))?;

        Ok(Anomaly {
            id: row.id.to_string(),
            timestamp,
            severity,
            category,
            message: row.message,
            metrics,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_repository_creation() {
        let repo = MetricsRepository::new("sqlite::memory:").await;
        assert!(repo.is_ok());
    }
}
