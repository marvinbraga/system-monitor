import React from 'react';
import { SystemMetrics } from '../types/metrics';
import { formatBytes } from '../utils/formatters';

interface SystemInfoProps {
  metrics: SystemMetrics | null;
}

/**
 * System information card component
 */
export const SystemInfo: React.FC<SystemInfoProps> = ({ metrics }) => {
  if (!metrics) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4">System Information</h2>
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-800">System Information</h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {/* CPU Info */}
        <div className="border-l-4 border-blue-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 mb-2">CPU</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600">Cores:</span>{' '}
              <span className="font-medium">{metrics.cpu.per_core.length}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">Load Average (1m):</span>{' '}
              <span className="font-medium">{metrics.cpu.load_avg_1.toFixed(2)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">Load Average (5m):</span>{' '}
              <span className="font-medium">{metrics.cpu.load_avg_5.toFixed(2)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">Load Average (15m):</span>{' '}
              <span className="font-medium">{metrics.cpu.load_avg_15.toFixed(2)}</span>
            </div>
          </div>
        </div>

        {/* Memory Info */}
        <div className="border-l-4 border-green-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 mb-2">Memory</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600">Total:</span>{' '}
              <span className="font-medium">{formatBytes(metrics.memory.total)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">Used:</span>{' '}
              <span className="font-medium">{formatBytes(metrics.memory.used)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">Available:</span>{' '}
              <span className="font-medium">{formatBytes(metrics.memory.available)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">Swap:</span>{' '}
              <span className="font-medium">
                {formatBytes(metrics.memory.swap_used)} / {formatBytes(metrics.memory.swap_total)}
              </span>
            </div>
          </div>
        </div>

        {/* Network Info */}
        <div className="border-l-4 border-purple-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 mb-2">Network</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600">RX:</span>{' '}
              <span className="font-medium">{formatBytes(metrics.network.rx_bytes)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">TX:</span>{' '}
              <span className="font-medium">{formatBytes(metrics.network.tx_bytes)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">RX Packets:</span>{' '}
              <span className="font-medium">{metrics.network.rx_packets.toLocaleString()}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">TX Packets:</span>{' '}
              <span className="font-medium">{metrics.network.tx_packets.toLocaleString()}</span>
            </div>
          </div>
        </div>

        {/* USB Devices */}
        <div className="border-l-4 border-yellow-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 mb-2">USB Devices</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600">Connected:</span>{' '}
              <span className="font-medium">{metrics.usb_devices.length}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600">With Timeout:</span>{' '}
              <span className="font-medium">
                {metrics.usb_devices.filter((d) => d.has_timeout).length}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Timestamp */}
      <div className="mt-4 pt-4 border-t border-gray-200">
        <div className="text-xs text-gray-500">
          Last updated: {new Date(metrics.timestamp).toLocaleString()}
        </div>
      </div>
    </div>
  );
};
