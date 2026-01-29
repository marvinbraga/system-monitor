pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod temperature;
pub mod usb;

use chrono::Utc;
use shared::types::SystemMetrics;
use std::io;

use cpu::CpuCollector;
use disk::DiskCollector;
use memory::MemoryCollector;
use network::NetworkCollector;
use temperature::TemperatureCollector;
use usb::UsbCollector;

/// Main metrics collector that aggregates all individual collectors
pub struct MetricsCollector {
    cpu: CpuCollector,
    memory: MemoryCollector,
    temperature: TemperatureCollector,
    disk: DiskCollector,
    usb: UsbCollector,
    network: NetworkCollector,
}

impl MetricsCollector {
    /// Creates a new MetricsCollector with all sub-collectors initialized
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            cpu: CpuCollector::new(),
            memory: MemoryCollector::new(),
            temperature: TemperatureCollector::new(),
            disk: DiskCollector::new()?,
            usb: UsbCollector::new(),
            network: NetworkCollector::new(),
        })
    }

    /// Collects all system metrics from all collectors
    pub fn collect_all(&mut self) -> io::Result<SystemMetrics> {
        // Refresh system state first
        self.cpu.refresh();
        self.memory.refresh();
        self.disk.refresh();

        Ok(SystemMetrics {
            timestamp: Utc::now(),
            cpu: self.cpu.collect(),
            memory: self.memory.collect(),
            temperatures: self.temperature.collect(),
            disks: self.disk.collect(),
            usb_devices: self.usb.collect(),
            network: self.network.collect(),
        })
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new().expect("Failed to initialize MetricsCollector")
    }
}
