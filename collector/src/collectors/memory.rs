use shared::types::MemoryMetrics;
use sysinfo::System;

/// Collects memory usage metrics including RAM and swap
pub struct MemoryCollector {
    system: System,
}

impl MemoryCollector {
    /// Creates a new MemoryCollector
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_memory();
        Self { system }
    }

    /// Refreshes the memory data from the system
    pub fn refresh(&mut self) {
        self.system.refresh_memory();
    }

    /// Collects memory metrics
    pub fn collect(&self) -> MemoryMetrics {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let available = self.system.available_memory();
        let usage_percent = if total > 0 {
            (used as f64 / total as f64 * 100.0) as f32
        } else {
            0.0
        };

        let swap_total = self.system.total_swap();
        let swap_used = self.system.used_swap();

        MemoryMetrics {
            total,
            used,
            available,
            usage_percent,
            swap_total,
            swap_used,
        }
    }
}

impl Default for MemoryCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_collector() {
        let mut collector = MemoryCollector::new();
        collector.refresh();
        let metrics = collector.collect();

        assert!(metrics.total > 0);
        assert!(metrics.used <= metrics.total);
        assert!(metrics.usage_percent >= 0.0);
        assert!(metrics.usage_percent <= 100.0);
    }

    #[test]
    fn test_memory_calculation() {
        let collector = MemoryCollector::new();
        let metrics = collector.collect();

        // Verify that used + available is approximately equal to total
        // (with some margin for caches and buffers)
        let sum = metrics.used + metrics.available;
        assert!(sum >= metrics.total * 90 / 100);
    }
}
