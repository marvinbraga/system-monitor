import React from 'react';
import { useMetrics } from '../hooks/useMetrics';
import { CpuChart } from './CpuChart';
import { MemoryChart } from './MemoryChart';
import { TemperatureGauge } from './TemperatureGauge';
import { DiskUsage } from './DiskUsage';
import { AnomalyList } from './AnomalyList';
import { SystemInfo } from './SystemInfo';

/**
 * Main dashboard component
 */
export const Dashboard: React.FC = () => {
  const { currentMetrics, metricsHistory, anomalies, loading, error, isConnected, refresh } =
    useMetrics();

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading system metrics...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="text-red-500 text-xl mb-4">Error loading metrics</div>
          <p className="text-gray-600 mb-4">{error.message}</p>
          <button
            onClick={refresh}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">System Monitor</h1>
              <p className="text-sm text-gray-500 mt-1">Real-time system metrics and monitoring</p>
            </div>

            <div className="flex items-center space-x-4">
              {/* Connection status */}
              <div className="flex items-center space-x-2">
                <div
                  className={`w-2 h-2 rounded-full ${
                    isConnected ? 'bg-green-500 animate-pulse' : 'bg-red-500'
                  }`}
                />
                <span className="text-sm text-gray-600">
                  {isConnected ? 'Connected' : 'Disconnected'}
                </span>
              </div>

              {/* Refresh button */}
              <button
                onClick={refresh}
                className="px-4 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-700 transition-colors flex items-center space-x-2"
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                  />
                </svg>
                <span>Refresh</span>
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
          <AnomalyList anomalies={anomalies} />

          {/* USB Devices */}
          {currentMetrics && currentMetrics.usb_devices.length > 0 && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-xl font-semibold mb-4 text-gray-800">USB Devices</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {currentMetrics.usb_devices.map((device, index) => (
                  <div
                    key={index}
                    className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
                  >
                    <div className="flex items-start justify-between mb-2">
                      <div className="flex-1">
                        <h3 className="text-sm font-semibold text-gray-800 truncate">
                          {device.product || 'Unknown Device'}
                        </h3>
                        <p className="text-xs text-gray-500 truncate">
                          {device.manufacturer || 'Unknown Manufacturer'}
                        </p>
                      </div>
                      {device.has_timeout && (
                        <span className="ml-2 px-2 py-0.5 text-xs font-medium rounded bg-yellow-100 text-yellow-800 border border-yellow-200">
                          Timeout
                        </span>
                      )}
                    </div>
                    <div className="text-xs text-gray-500 font-mono mt-2">{device.id}</div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white border-t border-gray-200 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="text-center text-sm text-gray-500">
            System Monitor Dashboard - Real-time monitoring powered by Rust and React
          </div>
        </div>
      </footer>
    </div>
  );
};
