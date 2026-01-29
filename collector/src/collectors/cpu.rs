use shared::types::CpuMetrics;
use std::fs;
use sysinfo::System;

/// Collects CPU usage metrics including global usage, per-core usage, and load averages
pub struct CpuCollector {
    system: System,
}

impl CpuCollector {
    /// Creates a new CpuCollector
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu_all();
        Self { system }
    }

    /// Refreshes the CPU data from the system
    pub fn refresh(&mut self) {
        // First refresh to get accurate CPU usage
        self.system.refresh_cpu_all();
        // Small sleep to let sysinfo calculate deltas
        std::thread::sleep(std::time::Duration::from_millis(200));
        // Second refresh for accurate readings
        self.system.refresh_cpu_all();
    }

    /// Collects CPU metrics
    pub fn collect(&self) -> CpuMetrics {
        let global_usage = self.system.global_cpu_usage();

        let per_core: Vec<f32> = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        let (load_avg_1, load_avg_5, load_avg_15) = Self::get_load_average();

        CpuMetrics {
            global_usage,
            per_core,
            load_avg_1,
            load_avg_5,
            load_avg_15,
        }
    }

    /// Reads load average from /proc/loadavg
    fn get_load_average() -> (f64, f64, f64) {
        if let Ok(content) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                let load1 = parts[0].parse::<f64>().unwrap_or(0.0);
                let load5 = parts[1].parse::<f64>().unwrap_or(0.0);
                let load15 = parts[2].parse::<f64>().unwrap_or(0.0);
                return (load1, load5, load15);
            }
        }
        (0.0, 0.0, 0.0)
    }
}

impl Default for CpuCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_collector() {
        let mut collector = CpuCollector::new();
        collector.refresh();
        let metrics = collector.collect();

        assert!(metrics.global_usage >= 0.0);
        assert!(metrics.global_usage <= 100.0);
        assert!(!metrics.per_core.is_empty());
    }

    #[test]
    fn test_load_average() {
        let (load1, load5, load15) = CpuCollector::get_load_average();
        assert!(load1 >= 0.0);
        assert!(load5 >= 0.0);
        assert!(load15 >= 0.0);
    }
}
