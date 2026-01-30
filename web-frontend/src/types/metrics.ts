// TypeScript types matching Rust shared types

export interface SystemMetrics {
  timestamp: string; // ISO 8601 datetime
  cpu: CpuMetrics;
  memory: MemoryMetrics;
  temperatures: Temperature[];
  disks: DiskMetrics[];
  usb_devices: UsbDevice[];
  network: NetworkMetrics;
  gpu: GpuMetrics | null;
}

export interface CpuMetrics {
  global_usage: number;
  per_core: number[];
  load_avg_1: number;
  load_avg_5: number;
  load_avg_15: number;
}

export interface MemoryMetrics {
  total: number;
  used: number;
  available: number;
  usage_percent: number;
  swap_total: number;
  swap_used: number;
}

export interface Temperature {
  sensor: string;
  value: number;
  label: string;
}

export interface DiskMetrics {
  name: string;
  mount_point: string;
  total: number;
  used: number;
  available: number;
  usage_percent: number;
  read_mb: number;
  write_mb: number;
}

export interface UsbDevice {
  id: string;
  manufacturer: string;
  product: string;
  has_timeout: boolean;
}

export interface NetworkMetrics {
  rx_bytes: number;
  tx_bytes: number;
  rx_packets: number;
  tx_packets: number;
}

export interface GpuMetrics {
  name: string;
  temperature: number;
  usage_percent: number;
  memory_usage_percent: number;
  memory_total_mb: number;
  memory_used_mb: number;
  memory_free_mb: number;
  power_draw_watts: number;
  fan_speed_percent: number;
}

export interface Anomaly {
  id: string;
  timestamp: string; // ISO 8601 datetime
  severity: AnomalySeverity;
  category: AnomalyCategory;
  message: string;
  metrics: Record<string, unknown>;
}

export enum AnomalySeverity {
  Info = 'Info',
  Warning = 'Warning',
  Critical = 'Critical',
}

export enum AnomalyCategory {
  Cpu = 'Cpu',
  Memory = 'Memory',
  Temperature = 'Temperature',
  Disk = 'Disk',
  Usb = 'Usb',
  Network = 'Network',
  Gpu = 'Gpu',
  System = 'System',
}

// WebSocket message types
export interface WebSocketMessage {
  type: 'metrics' | 'anomaly' | 'error';
  data: SystemMetrics | Anomaly | ErrorData;
}

export interface ErrorData {
  message: string;
  code?: string;
}

// Historical data for charts
export interface MetricsHistory {
  timestamps: string[];
  cpu_usage: number[];
  memory_usage: number[];
  network_rx: number[];
  network_tx: number[];
}
