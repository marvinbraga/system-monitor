use shared::types::GpuMetrics;
use std::process::Command;
use tracing::{debug, warn};

/// Collects GPU metrics using nvidia-smi for NVIDIA GPUs
pub struct GpuCollector {
    nvidia_smi_available: bool,
}

impl GpuCollector {
    /// Creates a new GpuCollector and checks for nvidia-smi availability
    pub fn new() -> Self {
        let nvidia_smi_available = Command::new("nvidia-smi")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if nvidia_smi_available {
            debug!("nvidia-smi detected, GPU metrics collection enabled");
        } else {
            debug!("nvidia-smi not available, GPU metrics collection disabled");
        }

        Self { nvidia_smi_available }
    }

    /// Collects GPU metrics from nvidia-smi
    pub fn collect(&self) -> Option<GpuMetrics> {
        if !self.nvidia_smi_available {
            return None;
        }

        self.collect_nvidia_metrics()
    }

    /// Collects metrics from NVIDIA GPU using nvidia-smi
    fn collect_nvidia_metrics(&self) -> Option<GpuMetrics> {
        let output = Command::new("nvidia-smi")
            .args([
                "--query-gpu=name,temperature.gpu,utilization.gpu,utilization.memory,memory.total,memory.used,memory.free,power.draw,fan.speed",
                "--format=csv,noheader,nounits"
            ])
            .output()
            .ok()?;

        if !output.status.success() {
            warn!("nvidia-smi command failed");
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout.trim();

        if line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() < 9 {
            warn!("Unexpected nvidia-smi output format: {}", line);
            return None;
        }

        // Parse each field with fallback to 0 on parse errors
        let name = parts[0].trim().to_string();
        let temperature = parts[1].trim().parse::<f32>().unwrap_or(0.0);
        let usage_percent = parts[2].trim().parse::<f32>().unwrap_or(0.0);
        let memory_usage_percent = parts[3].trim().parse::<f32>().unwrap_or(0.0);
        let memory_total_mb = parts[4].trim().parse::<u64>().unwrap_or(0);
        let memory_used_mb = parts[5].trim().parse::<u64>().unwrap_or(0);
        let memory_free_mb = parts[6].trim().parse::<u64>().unwrap_or(0);

        // Power draw might show "[N/A]" if not supported
        let power_draw_watts = parts[7]
            .trim()
            .replace("[N/A]", "0")
            .parse::<f32>()
            .unwrap_or(0.0);

        // Fan speed might show "[N/A]" on some cards
        let fan_speed_percent = parts[8]
            .trim()
            .replace("[N/A]", "0")
            .parse::<f32>()
            .unwrap_or(0.0);

        Some(GpuMetrics {
            name,
            temperature,
            usage_percent,
            memory_usage_percent,
            memory_total_mb,
            memory_used_mb,
            memory_free_mb,
            power_draw_watts,
            fan_speed_percent,
        })
    }

    /// Checks if GPU metrics collection is available
    pub fn is_available(&self) -> bool {
        self.nvidia_smi_available
    }
}

impl Default for GpuCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_collector_creation() {
        let collector = GpuCollector::new();
        // Just ensure it doesn't panic during creation
        let _ = collector.is_available();
    }

    #[test]
    fn test_gpu_metrics_collection() {
        let collector = GpuCollector::new();

        if collector.is_available() {
            let metrics = collector.collect();
            assert!(metrics.is_some(), "Should collect metrics when nvidia-smi is available");

            if let Some(gpu) = metrics {
                assert!(!gpu.name.is_empty(), "GPU name should not be empty");
                assert!(gpu.temperature >= 0.0 && gpu.temperature <= 120.0, "Temperature should be reasonable");
                assert!(gpu.usage_percent >= 0.0 && gpu.usage_percent <= 100.0, "Usage should be percentage");
                assert!(gpu.memory_total_mb > 0, "Memory total should be positive");
            }
        } else {
            let metrics = collector.collect();
            assert!(metrics.is_none(), "Should return None when nvidia-smi is not available");
        }
    }
}
