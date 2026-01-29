use shared::types::NetworkMetrics;
use std::fs;

/// Collects network traffic statistics
pub struct NetworkCollector {
    previous_stats: Option<NetworkStats>,
}

#[derive(Debug, Clone)]
struct NetworkStats {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_packets: u64,
    tx_packets: u64,
}

impl NetworkCollector {
    /// Creates a new NetworkCollector
    pub fn new() -> Self {
        let previous_stats = Self::read_net_dev();
        Self { previous_stats }
    }

    /// Collects network metrics
    pub fn collect(&mut self) -> NetworkMetrics {
        let current_stats = Self::read_net_dev();

        let metrics =
            if let (Some(current), Some(previous)) = (&current_stats, &self.previous_stats) {
                // Calculate deltas since last read
                NetworkMetrics {
                    rx_bytes: current.rx_bytes.saturating_sub(previous.rx_bytes),
                    tx_bytes: current.tx_bytes.saturating_sub(previous.tx_bytes),
                    rx_packets: current.rx_packets.saturating_sub(previous.rx_packets),
                    tx_packets: current.tx_packets.saturating_sub(previous.tx_packets),
                }
            } else {
                // First read or error reading stats
                NetworkMetrics {
                    rx_bytes: 0,
                    tx_bytes: 0,
                    rx_packets: 0,
                    tx_packets: 0,
                }
            };

        // Update previous stats for next collection
        self.previous_stats = current_stats;

        metrics
    }

    /// Reads network statistics from /proc/net/dev
    fn read_net_dev() -> Option<NetworkStats> {
        let content = fs::read_to_string("/proc/net/dev").ok()?;

        let mut total_rx_bytes = 0u64;
        let mut total_tx_bytes = 0u64;
        let mut total_rx_packets = 0u64;
        let mut total_tx_packets = 0u64;

        // Skip the first two header lines
        for line in content.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                let interface = parts[0].trim_end_matches(':');

                // Skip loopback interface
                if interface == "lo" {
                    continue;
                }

                // Parse network stats
                // Format: bytes packets errs drop fifo frame compressed multicast
                if let Ok(rx_bytes) = parts[1].parse::<u64>() {
                    total_rx_bytes += rx_bytes;
                }
                if let Ok(rx_packets) = parts[2].parse::<u64>() {
                    total_rx_packets += rx_packets;
                }
                if let Ok(tx_bytes) = parts[9].parse::<u64>() {
                    total_tx_bytes += tx_bytes;
                }
                if let Ok(tx_packets) = parts[10].parse::<u64>() {
                    total_tx_packets += tx_packets;
                }
            }
        }

        Some(NetworkStats {
            rx_bytes: total_rx_bytes,
            tx_bytes: total_tx_bytes,
            rx_packets: total_rx_packets,
            tx_packets: total_tx_packets,
        })
    }

    /// Gets per-interface network statistics
    pub fn get_interface_stats(&self) -> Vec<InterfaceStats> {
        let mut interfaces = Vec::new();

        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let interface = parts[0].trim_end_matches(':').to_string();

                    // Skip loopback
                    if interface == "lo" {
                        continue;
                    }

                    let rx_bytes = parts[1].parse::<u64>().unwrap_or(0);
                    let rx_packets = parts[2].parse::<u64>().unwrap_or(0);
                    let tx_bytes = parts[9].parse::<u64>().unwrap_or(0);
                    let tx_packets = parts[10].parse::<u64>().unwrap_or(0);

                    interfaces.push(InterfaceStats {
                        name: interface,
                        rx_bytes,
                        tx_bytes,
                        rx_packets,
                        tx_packets,
                    });
                }
            }
        }

        interfaces
    }

    /// Counts established TCP connections
    pub fn count_established_connections(&self) -> usize {
        let mut count = 0;

        for file in &["/proc/net/tcp", "/proc/net/tcp6"] {
            if let Ok(content) = fs::read_to_string(file) {
                for line in content.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    // State "01" is ESTABLISHED
                    if parts.len() > 3 && parts[3] == "01" {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Gets total bytes transferred (rx + tx) in MB
    pub fn get_total_mb(&self) -> f64 {
        if let Some(stats) = &self.previous_stats {
            (stats.rx_bytes + stats.tx_bytes) as f64 / 1024.0 / 1024.0
        } else {
            0.0
        }
    }

    /// Calculates current network throughput in MB/s
    pub fn get_throughput_mbps(&self, elapsed_seconds: f64) -> (f64, f64) {
        if let Some(stats) = &self.previous_stats {
            if elapsed_seconds > 0.0 {
                let rx_mbps = (stats.rx_bytes as f64 / 1024.0 / 1024.0) / elapsed_seconds;
                let tx_mbps = (stats.tx_bytes as f64 / 1024.0 / 1024.0) / elapsed_seconds;
                return (rx_mbps, tx_mbps);
            }
        }
        (0.0, 0.0)
    }
}

/// Per-interface network statistics
#[derive(Debug, Clone)]
pub struct InterfaceStats {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

impl Default for NetworkCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_collector() {
        let mut collector = NetworkCollector::new();

        // Give it a moment to establish baseline
        std::thread::sleep(std::time::Duration::from_millis(100));

        let metrics = collector.collect();

        // Metrics should be valid (may be zero if no network activity)
        // u64 values are always >= 0, no need to assert
        assert!(metrics.rx_bytes > 0 || metrics.tx_bytes > 0);
    }

    #[test]
    fn test_read_net_dev() {
        let stats = NetworkCollector::read_net_dev();
        assert!(stats.is_some());

        let stats = stats.unwrap();
        // u64 values are always >= 0
        assert!(stats.rx_bytes > 0 || stats.tx_bytes > 0);
    }

    #[test]
    fn test_interface_stats() {
        let collector = NetworkCollector::new();
        let interfaces = collector.get_interface_stats();

        // Most systems have at least one network interface
        if !interfaces.is_empty() {
            for iface in interfaces {
                assert!(!iface.name.is_empty());
                assert!(iface.name != "lo");
            }
        }
    }

    #[test]
    fn test_established_connections() {
        let collector = NetworkCollector::new();
        let count = collector.count_established_connections();

        // Should return a valid count (may be zero)
        // usize is always >= 0, check for non-zero instead
        assert!(count > 0);
    }

    #[test]
    fn test_total_mb() {
        let collector = NetworkCollector::new();
        let total = collector.get_total_mb();

        assert!(total >= 0.0);
    }
}
