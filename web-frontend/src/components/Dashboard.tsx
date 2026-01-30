import React, { useState, useMemo } from 'react';
import { useMetrics } from '../hooks/useMetrics';
import { useTheme } from '../hooks/useTheme';
import { CpuChart } from './CpuChart';
import { MemoryChart } from './MemoryChart';
import { TemperatureChart } from './TemperatureChart';
import { TemperatureGauge } from './TemperatureGauge';
import { DiskUsage } from './DiskUsage';
import { AnomalyList } from './AnomalyList';
import { SystemInfo } from './SystemInfo';
import { GpuChart } from './GpuChart';

type UsbSortOption = 'product-asc' | 'product-desc' | 'manufacturer-asc' | 'manufacturer-desc' | 'id-asc' | 'id-desc' | 'none';

/**
 * Main dashboard component
 */
export const Dashboard: React.FC = () => {
  const { currentMetrics, metricsHistory, anomalies, loading, error, isConnected, refresh } =
    useMetrics();
  const { theme, toggleTheme } = useTheme();
  const [usbSortBy, setUsbSortBy] = useState<UsbSortOption>('product-asc');

  // Sort USB devices based on selected criterion
  const sortedUsbDevices = useMemo(() => {
    if (!currentMetrics?.usb_devices) return [];

    const devices = [...currentMetrics.usb_devices];

    switch (usbSortBy) {
      case 'product-asc':
        return devices.sort((a, b) =>
          (a.product || 'Unknown Device').localeCompare(b.product || 'Unknown Device')
        );
      case 'product-desc':
        return devices.sort((a, b) =>
          (b.product || 'Unknown Device').localeCompare(a.product || 'Unknown Device')
        );
      case 'manufacturer-asc':
        return devices.sort((a, b) =>
          (a.manufacturer || 'Unknown Manufacturer').localeCompare(b.manufacturer || 'Unknown Manufacturer')
        );
      case 'manufacturer-desc':
        return devices.sort((a, b) =>
          (b.manufacturer || 'Unknown Manufacturer').localeCompare(a.manufacturer || 'Unknown Manufacturer')
        );
      case 'id-asc':
        return devices.sort((a, b) => a.id.localeCompare(b.id));
      case 'id-desc':
        return devices.sort((a, b) => b.id.localeCompare(a.id));
      case 'none':
        return devices;
      default:
        return devices;
    }
  }, [currentMetrics?.usb_devices, usbSortBy]);

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 dark:border-blue-400 mx-auto mb-4"></div>
          <p className="text-gray-600 dark:text-gray-400">Loading system metrics...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900">
        <div className="text-center">
          <div className="text-red-500 dark:text-red-400 text-xl mb-4">Error loading metrics</div>
          <p className="text-gray-600 dark:text-gray-400 mb-4">{error.message}</p>
          <button
            onClick={refresh}
            className="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900 dark:text-white">System Monitor</h1>
              <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                Real-time system metrics and monitoring
              </p>
            </div>

            <div className="flex items-center space-x-4">
              {/* Connection status */}
              <div className="flex items-center space-x-2">
                <div
                  className={`w-2 h-2 rounded-full ${
                    isConnected ? 'bg-green-500 animate-pulse' : 'bg-red-500'
                  }`}
                />
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {isConnected ? 'Connected' : 'Disconnected'}
                </span>
              </div>

              {/* Theme toggle */}
              <button
                onClick={toggleTheme}
                className="p-2 rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                aria-label="Toggle theme"
              >
                {theme === 'dark' ? (
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                  </svg>
                ) : (
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                    />
                  </svg>
                )}
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="space-y-6">
          {/* System Info */}
          <SystemInfo metrics={currentMetrics} />

          {/* Charts Grid */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <CpuChart history={metricsHistory} />
            <MemoryChart history={metricsHistory} />
            <TemperatureChart history={metricsHistory} />
            <GpuChart history={metricsHistory} />
          </div>

          {/* Temperature and Disk */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {currentMetrics && (
              <>
                <TemperatureGauge temperatures={currentMetrics.temperatures} />
                <DiskUsage disks={currentMetrics.disks} />
              </>
            )}
          </div>

          {/* Anomalies */}
          <AnomalyList anomalies={anomalies} onRefresh={refresh} />

          {/* USB Devices */}
          {currentMetrics && currentMetrics.usb_devices.length > 0 && (
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold text-gray-800 dark:text-white">USB Devices</h2>
                <div className="flex items-center space-x-2">
                  <label className="text-sm text-gray-600 dark:text-gray-400">Sort by:</label>
                  <select
                    value={usbSortBy}
                    onChange={(e) => setUsbSortBy(e.target.value as UsbSortOption)}
                    className="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  >
                    <option value="product-asc">Product (A-Z)</option>
                    <option value="product-desc">Product (Z-A)</option>
                    <option value="manufacturer-asc">Manufacturer (A-Z)</option>
                    <option value="manufacturer-desc">Manufacturer (Z-A)</option>
                    <option value="id-asc">ID (A-Z)</option>
                    <option value="id-desc">ID (Z-A)</option>
                    <option value="none">Filesystem Order</option>
                  </select>
                </div>
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {sortedUsbDevices.map((device, index) => (
                  <div
                    key={index}
                    className="border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:shadow-md transition-shadow bg-white dark:bg-gray-800"
                  >
                    <div className="flex items-start justify-between mb-2">
                      <div className="flex-1">
                        <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200 truncate">
                          {device.product || 'Unknown Device'}
                        </h3>
                        <p className="text-xs text-gray-500 dark:text-gray-400 truncate">
                          {device.manufacturer || 'Unknown Manufacturer'}
                        </p>
                      </div>
                      {device.has_timeout && (
                        <span className="ml-2 px-2 py-0.5 text-xs font-medium rounded bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 border border-yellow-200 dark:border-yellow-700">
                          Timeout
                        </span>
                      )}
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400 font-mono mt-2">{device.id}</div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="text-center text-sm text-gray-500 dark:text-gray-400">
            System Monitor Dashboard - Real-time monitoring powered by Rust and React
          </div>
        </div>
      </footer>
    </div>
  );
};
