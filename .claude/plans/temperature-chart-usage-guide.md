# Temperature Chart - Usage Guide

## Quick Start

### For End Users

The temperature chart is now available on the main dashboard. No configuration needed - it automatically detects and displays all available temperature sensors on your system.

**Access the chart**:
1. Ensure the collector is running: `sudo systemctl status system-monitor-collector`
2. Open the web dashboard: http://localhost:5252
3. The temperature chart appears in the charts grid section

### For Developers

**Component Location**: `web-frontend/src/components/TemperatureChart.tsx`

**Usage in Dashboard**:
```tsx
import { TemperatureChart } from './TemperatureChart';

// In your component:
<TemperatureChart history={metricsHistory} />
```

**Props**:
- `history`: Array of SystemMetrics (from useMetrics hook)

## Features

### 1. Real-Time Temperature Monitoring
- Updates every 2 seconds via WebSocket
- Displays 2 minutes of historical data (60 data points)
- Smooth line charts for each sensor

### 2. Dynamic Sensor Detection
The chart automatically detects and displays:
- **CPU temperatures** (k10temp, coretemp)
  - Tctl (CPU temperature control)
  - Tccd1/Tccd2 (Core Complex Die temperatures)
- **NVMe/SSD temperatures** (nvme sensors)
  - Composite temperature
  - Individual sensor readings
- **GPU temperatures** (amdgpu, nvidia)
- **Other sensors** (WiFi, Ethernet, ACPI thermal zones)

### 3. Intelligent Display
- **If ≤5 sensors**: Shows all sensors in the chart
- **If >5 sensors**: Shows top 5 hottest sensors in chart, all sensors in grid below
- **Empty state**: Shows friendly message if no sensors available

### 4. Color Coding

**Chart Lines**:
- 🔴 **Red**: CPU (k10temp, coretemp) - hottest component
- 🔵 **Blue**: NVMe/SSD drives
- 🟣 **Purple**: GPU (amdgpu, nvidia)
- 🟦 **Cyan**: Disk drives
- 🟢 **Green/Orange/Pink**: Other sensors (WiFi, Ethernet, etc.)

**Current Temperature Grid**:
- 🟢 **Green background**: < 60°C (normal operating temperature)
- 🟡 **Yellow background**: 60-74°C (warm, but acceptable)
- 🔴 **Red background**: ≥ 75°C (hot, approaching thermal limits)

### 5. Theme Support
- Light mode: White background, dark text
- Dark mode: Dark gray background, light text
- Automatic color adaptation for all chart elements

### 6. Responsive Design
- Desktop: Side-by-side in 2-column grid
- Tablet: Stacks vertically
- Mobile: Full-width, optimized for touch

## What You See

### Top Section
```
┌─────────────────────────────────────────────────────┐
│ Temperature                          73.3°C         │
│                                      Tccd1          │
└─────────────────────────────────────────────────────┘
```
Shows the highest current temperature and its sensor name.

### Chart Section
A line chart with:
- **X-axis**: Time in HH:mm:ss format (last 2 minutes)
- **Y-axis**: Temperature in °C (auto-scaled)
- **Lines**: One colored line per sensor
- **Legend**: Sensor names with color indicators
- **Tooltip**: Hover to see exact temperatures at any time point

### Current Temperatures Grid
```
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Tccd1        │ │ Tctl         │ │ Composite    │
│ 73.3°C       │ │ 66.4°C       │ │ 35.8°C       │
└──────────────┘ └──────────────┘ └──────────────┘
```
Shows all sensors sorted by temperature (hottest first).

## Example System Configurations

### AMD Ryzen with NVMe (11 sensors)
```
Sensors detected:
- Coolant temp (x53) - 29.9°C
- Tctl (k10temp) - 66.4°C
- Tccd1 (k10temp) - 73.3°C
- Tccd2 (k10temp) - 44.8°C
- Composite (nvme) - 30.9°C
- Composite (nvme) - 35.8°C
- Sensor 1 (acpitz) - 16.8°C
- Sensor 1 (iwlwifi_1) - 24.0°C
- Sensor 1 (nvme) - 35.8°C
- Sensor 2 (nvme) - 24.9°C
- Sensor 1 (r8169_0_a00:00) - 32.5°C

Chart shows: Top 5 hottest (Tccd1, Tctl, Composite, Composite, Sensor 1)
Grid shows: All 11 sensors
```

### Intel with Integrated Graphics (3 sensors)
```
Sensors detected:
- Package id 0 (coretemp) - 65.0°C
- Core 0 (coretemp) - 60.0°C
- Core 1 (coretemp) - 58.0°C

Chart shows: All 3 sensors
Grid shows: All 3 sensors
```

### System Without Temperature Sensors (0 sensors)
```
Displays empty state message:
"Temperature monitoring not available
No temperature sensors detected on this system"
```

## Interpreting the Data

### Normal Operating Temperatures
- **CPU (idle)**: 30-50°C
- **CPU (load)**: 50-75°C
- **NVMe/SSD**: 30-50°C
- **GPU (idle)**: 30-50°C
- **GPU (load)**: 50-80°C

### Warning Signs
- **CPU > 80°C**: High load or cooling issues
- **CPU > 90°C**: Thermal throttling may occur
- **NVMe > 60°C**: Performance degradation possible
- **Any sensor > 85°C**: Check cooling system

### What to Watch For
1. **Sudden spikes**: May indicate thermal event or process startup
2. **Gradual increase**: May indicate dust buildup or failing cooling
3. **High baseline**: May indicate ambient temperature issues
4. **Asymmetric cores**: May indicate uneven cooling or workload

## Technical Details

### Data Source
- **Backend**: Rust collector reading from `/sys/class/hwmon/`
- **Database**: SQLite (metrics table, temperatures JSON column)
- **API**: REST endpoint `/api/v1/metrics/history` (last 60 records)
- **WebSocket**: Real-time updates at `ws://localhost:5253/ws`

### Update Frequency
- **Collector**: Reads sensors every 2 seconds
- **Database**: Stores every reading
- **WebSocket**: Pushes to clients every 2 seconds
- **Chart**: Updates in real-time (no page refresh needed)

### Browser Requirements
- Modern browser with ES6+ support
- JavaScript enabled
- SVG support (for Recharts)
- WebSocket support (for real-time updates)

### Performance
- **Memory**: ~1MB additional data (60 temperature snapshots)
- **CPU**: Minimal (chart updates managed by React)
- **Network**: ~1KB per WebSocket message
- **Rendering**: 60fps smooth animations via Recharts

## Troubleshooting

### Chart Shows "No data available"
**Causes**:
1. Collector not running
2. WebSocket disconnected
3. No historical data yet (wait 2 minutes)

**Solutions**:
```bash
# Check collector status
sudo systemctl status system-monitor-collector

# Restart collector
sudo systemctl restart system-monitor-collector

# Check API manually
curl http://localhost:5253/api/v1/metrics/current | jq '.data.temperatures'
```

### Chart Shows "Temperature monitoring not available"
**Causes**:
1. System has no temperature sensors
2. Sensors not accessible (permission issues)
3. Collector can't read `/sys/class/hwmon/`

**Solutions**:
```bash
# Check if sensors are available
ls -la /sys/class/hwmon/

# Check sensors manually
sensors

# Install lm-sensors if not present
sudo apt install lm-sensors
sudo sensors-detect
```

### Temperature values seem wrong
**Causes**:
1. Collector running in Docker (namespace isolation)
2. Sensor calibration issues
3. Sensor misidentification

**Solutions**:
```bash
# Ensure collector runs natively (NOT in Docker)
sudo systemctl status system-monitor-collector

# Compare with system sensors
watch -n 2 sensors

# Check collector logs
sudo journalctl -u system-monitor-collector -f
```

### Chart not updating in real-time
**Causes**:
1. WebSocket disconnected
2. Collector stopped
3. Browser WebSocket disabled

**Solutions**:
1. Check connection indicator in top-right (should be green)
2. Refresh page to reconnect WebSocket
3. Check browser console for WebSocket errors

## API Reference

### Get Current Temperatures
```bash
curl http://localhost:5253/api/v1/metrics/current | jq '.data.temperatures'
```

**Response**:
```json
[
  {
    "sensor": "k10temp",
    "value": 66.375,
    "label": "Tctl"
  },
  {
    "sensor": "nvme",
    "value": 30.85,
    "label": "Composite"
  }
]
```

### Get Temperature History (2 minutes)
```bash
curl http://localhost:5253/api/v1/metrics/history?limit=60 | jq '.data[].temperatures'
```

### WebSocket Subscription
```javascript
const ws = new WebSocket('ws://localhost:5253/ws');
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  if (data.type === 'metrics') {
    console.log('Temperatures:', data.data.temperatures);
  }
};
```

## Component Props API

```typescript
interface TemperatureChartProps {
  history: SystemMetrics[];  // Array of system metrics snapshots
}

interface SystemMetrics {
  timestamp: string;          // ISO 8601 datetime
  temperatures: Temperature[]; // Array of temperature readings
  // ... other metrics
}

interface Temperature {
  sensor: string;    // Sensor ID (k10temp, nvme, etc.)
  value: number;     // Temperature in Celsius
  label: string;     // Human-readable label (Tctl, Composite, etc.)
}
```

## Customization

### Changing Color Scheme
Edit `TemperatureChart.tsx` line ~90:
```typescript
const getSensorColor = (label: string): string => {
  // Modify colors here
  if (lowerLabel.includes('cpu')) return '#ef4444'; // Change CPU color
  // ...
}
```

### Changing Temperature Thresholds
Edit `TemperatureChart.tsx` line ~235:
```typescript
const isHot = temp >= 75;   // Change hot threshold
const isWarm = temp >= 60;  // Change warm threshold
```

### Changing Sensor Limit
Edit `TemperatureChart.tsx` line ~140:
```typescript
const displaySensors = sensorLabels.length > 5  // Change limit here
```

### Changing Chart Height
Edit `TemperatureChart.tsx` line ~176:
```typescript
<ResponsiveContainer width="100%" height={300}>  // Change height
```

## Related Components

- **TemperatureGauge**: Circular gauge showing current max temperature
- **CpuChart**: CPU usage chart (similar pattern)
- **MemoryChart**: Memory usage chart (similar pattern)
- **AnomalyList**: Shows temperature-related anomalies

## Future Roadmap

Potential enhancements (not yet implemented):
1. Threshold warning lines (85°C critical)
2. Temperature alerts/notifications
3. Sensor filtering (show/hide specific sensors)
4. Extended history (configurable time range)
5. Temperature export (CSV download)
6. Historical comparison (compare current vs previous day)

---

**Component Version**: 1.0.0
**Last Updated**: 2026-01-29
**Maintainer**: System Monitor Team
**License**: Same as System Monitor project
