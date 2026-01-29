use shared::types::Temperature;
use std::fs;
use std::path::PathBuf;

/// Collects temperature readings from various hardware sensors
pub struct TemperatureCollector;

impl TemperatureCollector {
    /// Creates a new TemperatureCollector
    pub fn new() -> Self {
        Self
    }

    /// Collects all temperature sensor readings
    pub fn collect(&self) -> Vec<Temperature> {
        let mut temperatures = Vec::new();

        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let hwmon_dir = entry.path();
                self.collect_from_hwmon(&hwmon_dir, &mut temperatures);
            }
        }

        temperatures
    }

    /// Collects temperature readings from a specific hwmon directory
    fn collect_from_hwmon(&self, hwmon_dir: &PathBuf, temperatures: &mut Vec<Temperature>) {
        // Read the device/sensor name
        let sensor_name = fs::read_to_string(hwmon_dir.join("name"))
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string();

        // Try to read up to 10 temperature inputs (temp1_input through temp10_input)
        for i in 1..=10 {
            let temp_file = hwmon_dir.join(format!("temp{}_input", i));
            if let Ok(temp_str) = fs::read_to_string(&temp_file) {
                if let Ok(temp_raw) = temp_str.trim().parse::<i32>() {
                    // Temperature is in millidegrees Celsius
                    let temp_value = temp_raw as f32 / 1000.0;

                    // Try to read the label for this sensor
                    let label = fs::read_to_string(hwmon_dir.join(format!("temp{}_label", i)))
                        .unwrap_or_else(|_| format!("Sensor {}", i))
                        .trim()
                        .to_string();

                    temperatures.push(Temperature {
                        sensor: sensor_name.clone(),
                        value: temp_value,
                        label,
                    });
                }
            }
        }
    }

    /// Gets CPU temperatures (from k10temp sensor)
    pub fn get_cpu_temps(&self) -> Vec<Temperature> {
        self.collect()
            .into_iter()
            .filter(|t| t.sensor.contains("k10temp") || t.sensor.contains("coretemp"))
            .collect()
    }

    /// Gets NVMe drive temperatures
    pub fn get_nvme_temps(&self) -> Vec<Temperature> {
        self.collect()
            .into_iter()
            .filter(|t| t.sensor.contains("nvme"))
            .collect()
    }

    /// Gets GPU temperatures
    pub fn get_gpu_temps(&self) -> Vec<Temperature> {
        self.collect()
            .into_iter()
            .filter(|t| {
                t.sensor.contains("amdgpu")
                    || t.sensor.contains("nvidia")
                    || t.sensor.contains("radeon")
            })
            .collect()
    }

    /// Gets the maximum temperature across all sensors
    pub fn get_max_temp(&self) -> f32 {
        self.collect()
            .iter()
            .map(|t| t.value)
            .fold(0.0f32, |max, val| max.max(val))
    }
}

impl Default for TemperatureCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_collector() {
        let collector = TemperatureCollector::new();
        let temps = collector.collect();

        // Should be able to collect some temperatures on most systems
        // But this may fail in containers or VMs
        if !temps.is_empty() {
            for temp in &temps {
                assert!(temp.value >= 0.0);
                assert!(temp.value <= 150.0); // Sanity check
                assert!(!temp.sensor.is_empty());
                assert!(!temp.label.is_empty());
            }
        }
    }

    #[test]
    fn test_max_temp() {
        let collector = TemperatureCollector::new();
        let max = collector.get_max_temp();

        // Max temp should be non-negative
        assert!(max >= 0.0);
    }

    #[test]
    fn test_filtered_temps() {
        let collector = TemperatureCollector::new();

        let cpu_temps = collector.get_cpu_temps();
        let nvme_temps = collector.get_nvme_temps();
        let gpu_temps = collector.get_gpu_temps();

        // All filtered temps should match their respective patterns
        for temp in cpu_temps {
            assert!(temp.sensor.contains("k10temp") || temp.sensor.contains("coretemp"));
        }

        for temp in nvme_temps {
            assert!(temp.sensor.contains("nvme"));
        }

        for temp in gpu_temps {
            assert!(
                temp.sensor.contains("amdgpu")
                    || temp.sensor.contains("nvidia")
                    || temp.sensor.contains("radeon")
            );
        }
    }
}
