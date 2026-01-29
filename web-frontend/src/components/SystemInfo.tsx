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
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">System Information</h2>
        <div className="text-gray-500 dark:text-gray-400">Loading...</div>
      </div>
    );
  }

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">System Information</h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {/* CPU Info */}
        <div className="border-l-4 border-blue-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-300 mb-2">CPU</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Cores:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.cpu.per_core.length}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Load Average (1m):</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.cpu.load_avg_1.toFixed(2)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Load Average (5m):</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.cpu.load_avg_5.toFixed(2)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Load Average (15m):</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.cpu.load_avg_15.toFixed(2)}</span>
            </div>
          </div>
        </div>

        {/* Memory Info */}
        <div className="border-l-4 border-green-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-300 mb-2">Memory</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Total:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{formatBytes(metrics.memory.total)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Used:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{formatBytes(metrics.memory.used)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Available:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{formatBytes(metrics.memory.available)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Swap:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">
                {formatBytes(metrics.memory.swap_used)} / {formatBytes(metrics.memory.swap_total)}
              </span>
            </div>
          </div>
        </div>

        {/* Network Info */}
        <div className="border-l-4 border-purple-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-300 mb-2">Network</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">RX:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{formatBytes(metrics.network.rx_bytes)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">TX:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{formatBytes(metrics.network.tx_bytes)}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">RX Packets:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.network.rx_packets.toLocaleString()}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">TX Packets:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.network.tx_packets.toLocaleString()}</span>
            </div>
          </div>
        </div>

        {/* USB Devices */}
        <div className="border-l-4 border-yellow-500 pl-4">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-300 mb-2">USB Devices</h3>
          <div className="space-y-1">
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">Connected:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">{metrics.usb_devices.length}</span>
            </div>
            <div className="text-sm">
              <span className="text-gray-600 dark:text-gray-300">With Timeout:</span>{' '}
              <span className="font-medium text-gray-800 dark:text-white">
                {metrics.usb_devices.filter((d) => d.has_timeout).length}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Timestamp */}
      <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
        <div className="text-xs text-gray-500 dark:text-gray-400">
          Last updated: {new Date(metrics.timestamp).toLocaleString()}
        </div>
      </div>
    </div>
  );
};
