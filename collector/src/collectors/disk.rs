use shared::types::DiskMetrics;
use std::collections::HashMap;
use std::fs;
use std::io;
use sysinfo::Disks;

/// Collects disk usage and I/O statistics
pub struct DiskCollector {
    disks: Disks,
    previous_stats: HashMap<String, DiskStats>,
}

#[derive(Debug, Clone)]
struct DiskStats {
    read_sectors: u64,
    write_sectors: u64,
}

impl DiskCollector {
    /// Creates a new DiskCollector
    pub fn new() -> io::Result<Self> {
        let disks = Disks::new_with_refreshed_list();
        let previous_stats = Self::read_diskstats()?;

        Ok(Self {
            disks,
            previous_stats,
        })
    }

    /// Refreshes disk information
    pub fn refresh(&mut self) {
        self.disks.refresh();
    }

    /// Collects disk metrics including space usage and I/O stats
    pub fn collect(&mut self) -> Vec<DiskMetrics> {
        let mut metrics = Vec::new();

        // Get current I/O stats
        let current_stats = Self::read_diskstats().unwrap_or_default();

        for disk in self.disks.iter() {
            let name = disk.name().to_string_lossy().to_string();

            let mount_point = disk.mount_point().to_string_lossy().to_string();

            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);

            let usage_percent = if total > 0 {
                (used as f64 / total as f64 * 100.0) as f32
            } else {
                0.0
            };

            // Calculate I/O rates
            let (read_mb, write_mb) = self.calculate_io_rate(&name, &current_stats);

            metrics.push(DiskMetrics {
                name,
                mount_point,
                total,
                used,
                available,
                usage_percent,
                read_mb,
                write_mb,
            });
        }

        // Update previous stats for next calculation
        self.previous_stats = current_stats;

        metrics
    }

    /// Reads /proc/diskstats to get I/O statistics
    fn read_diskstats() -> io::Result<HashMap<String, DiskStats>> {
        let mut stats = HashMap::new();

        let content = fs::read_to_string("/proc/diskstats")?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 {
                let device = parts[2].to_string();

                // Skip partitions, only track whole disks and major devices
                // This filters out things like sda1, sda2, etc.
                if device.chars().last().is_some_and(|c| c.is_numeric()) && device.len() > 3 {
                    continue;
                }

                let read_sectors = parts[5].parse::<u64>().unwrap_or(0);
                let write_sectors = parts[9].parse::<u64>().unwrap_or(0);

                stats.insert(
                    device,
                    DiskStats {
                        read_sectors,
                        write_sectors,
                    },
                );
            }
        }

        Ok(stats)
    }

    /// Calculates I/O rate in MB/s based on sector differences
    fn calculate_io_rate(
        &self,
        disk_name: &str,
        current_stats: &HashMap<String, DiskStats>,
    ) -> (f64, f64) {
        // Extract device name from path (e.g., "/dev/sda" -> "sda")
        let device_name = disk_name.trim_start_matches("/dev/").to_string();

        // Try different name variations
        let possible_names = vec![
            device_name.clone(),
            device_name.trim_end_matches(char::is_numeric).to_string(),
        ];

        for name in possible_names {
            if let (Some(current), Some(previous)) =
                (current_stats.get(&name), self.previous_stats.get(&name))
            {
                // Calculate sectors read/written since last check
                let read_sectors = current.read_sectors.saturating_sub(previous.read_sectors);
                let write_sectors = current.write_sectors.saturating_sub(previous.write_sectors);

                // Convert sectors to MB (assuming 512 bytes per sector)
                // Divide by 2 to get approximate MB/s (since we sample every ~2 seconds)
                let read_mb = (read_sectors * 512) as f64 / 1024.0 / 1024.0 / 2.0;
                let write_mb = (write_sectors * 512) as f64 / 1024.0 / 1024.0 / 2.0;

                return (read_mb, write_mb);
            }
        }

        (0.0, 0.0)
    }

    /// Gets the maximum disk usage percentage across all disks
    pub fn get_max_usage(&self) -> f32 {
        self.disks
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);
                if total > 0 {
                    (used as f64 / total as f64 * 100.0) as f32
                } else {
                    0.0
                }
            })
            .fold(0.0f32, |max, val| max.max(val))
    }

    /// Gets the maximum I/O rate across all disks
    pub fn get_max_io_rate(&mut self) -> f32 {
        let metrics = self.collect();
        metrics
            .iter()
            .map(|m| (m.read_mb + m.write_mb) as f32)
            .fold(0.0f32, |max, val| max.max(val))
    }
}

impl Default for DiskCollector {
    fn default() -> Self {
        Self::new().expect("Failed to initialize DiskCollector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_collector() {
        let mut collector = DiskCollector::new().unwrap();
        let metrics = collector.collect();

        assert!(!metrics.is_empty());

        for metric in &metrics {
            assert!(!metric.name.is_empty());
            assert!(!metric.mount_point.is_empty());
            assert!(metric.usage_percent >= 0.0);
            assert!(metric.usage_percent <= 100.0);
            assert!(metric.read_mb >= 0.0);
            assert!(metric.write_mb >= 0.0);
        }
    }

    #[test]
    fn test_read_diskstats() {
        let stats = DiskCollector::read_diskstats();
        assert!(stats.is_ok());

        let stats = stats.unwrap();
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_max_usage() {
        let collector = DiskCollector::new().unwrap();
        let max = collector.get_max_usage();

        assert!(max >= 0.0);
        assert!(max <= 100.0);
    }
}
