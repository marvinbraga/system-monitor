use shared::types::UsbDevice;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Collects USB device information and detects timeouts
pub struct UsbCollector;

impl UsbCollector {
    /// Creates a new UsbCollector
    pub fn new() -> Self {
        Self
    }

    /// Collects information about all USB devices
    pub fn collect(&self) -> Vec<UsbDevice> {
        let mut devices = Vec::new();

        // Read USB devices from /sys/bus/usb/devices/
        if let Ok(entries) = fs::read_dir("/sys/bus/usb/devices") {
            for entry in entries.flatten() {
                let device_path = entry.path();

                // Only process actual USB devices (not interfaces)
                if let Some(device) = self.read_device(&device_path) {
                    devices.push(device);
                }
            }
        }

        // Check for timeouts in dmesg
        let timeout_devices = self.check_usb_timeouts();

        // Mark devices that have timeouts
        for device in &mut devices {
            if timeout_devices.contains(&device.id) {
                device.has_timeout = true;
            }
        }

        devices
    }

    /// Reads USB device information from a sysfs path
    fn read_device(&self, device_path: &Path) -> Option<UsbDevice> {
        // Check if this is a USB device (has idVendor and idProduct files)
        let vendor_file = device_path.join("idVendor");
        let product_file = device_path.join("idProduct");

        if !vendor_file.exists() || !product_file.exists() {
            return None;
        }

        let vendor = fs::read_to_string(&vendor_file)
            .unwrap_or_default()
            .trim()
            .to_string();

        let product = fs::read_to_string(&product_file)
            .unwrap_or_default()
            .trim()
            .to_string();

        let id = format!("{}:{}", vendor, product);

        let manufacturer = fs::read_to_string(device_path.join("manufacturer"))
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .to_string();

        let product_name = fs::read_to_string(device_path.join("product"))
            .unwrap_or_else(|_| format!("USB Device {}", id))
            .trim()
            .to_string();

        Some(UsbDevice {
            id,
            manufacturer,
            product: product_name,
            has_timeout: false, // Will be updated later
        })
    }

    /// Checks dmesg for USB timeout errors in the last 5 minutes
    fn check_usb_timeouts(&self) -> Vec<String> {
        let mut timeout_devices = Vec::new();

        let output = Command::new("dmesg")
            .args(["-T", "--since", "5 minutes ago"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let content = String::from_utf8_lossy(&output.stdout).to_lowercase();

                for line in content.lines() {
                    if line.contains("usb") && line.contains("timeout") {
                        // Try to extract device ID from the message
                        // This is a best-effort approach
                        if let Some(device_id) = self.extract_device_id(line) {
                            timeout_devices.push(device_id);
                        }
                    }
                }
            }
        }

        timeout_devices
    }

    /// Attempts to extract a USB device ID from a dmesg line
    fn extract_device_id(&self, line: &str) -> Option<String> {
        // Look for patterns like "usb 1-2" or "1-2.3"
        // This is a simplified extraction - real USB device tracking is complex
        for word in line.split_whitespace() {
            if (word.starts_with("usb") || word.contains('-'))
                && word.chars().any(|c| c.is_numeric())
                && word.contains('-')
            {
                return Some(word.to_string());
            }
        }
        None
    }

    /// Checks if there are any recent USB timeouts
    pub fn has_recent_timeouts(&self) -> bool {
        !self.check_usb_timeouts().is_empty()
    }

    /// Gets count of devices with timeouts
    pub fn timeout_count(&self) -> usize {
        self.collect().iter().filter(|d| d.has_timeout).count()
    }
}

impl Default for UsbCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_collector() {
        let collector = UsbCollector::new();
        let devices = collector.collect();

        // May or may not have USB devices depending on the system
        for device in &devices {
            assert!(!device.id.is_empty());
            assert!(!device.manufacturer.is_empty());
            assert!(!device.product.is_empty());
        }
    }

    #[test]
    fn test_timeout_check() {
        let collector = UsbCollector::new();
        let has_timeouts = collector.has_recent_timeouts();

        // This test just ensures the method doesn't panic
        let _ = has_timeouts; // Function executed successfully
    }

    #[test]
    fn test_timeout_count() {
        let collector = UsbCollector::new();
        let count = collector.timeout_count();

        // usize is always >= 0, just verify it's a valid number
        assert!(count < 10000); // Sanity check
    }

    #[test]
    fn test_extract_device_id() {
        let collector = UsbCollector::new();

        let test_cases = vec![
            ("usb 1-2: timeout error", Some("1-2")),
            ("device timeout on usb1-2.3", Some("1-2.3")),
            ("no usb info here", None),
        ];

        for (input, expected) in test_cases {
            let result = collector.extract_device_id(input);
            if let Some(expected_id) = expected {
                assert!(result.is_some());
                assert!(result.unwrap().contains(expected_id));
            }
        }
    }
}
