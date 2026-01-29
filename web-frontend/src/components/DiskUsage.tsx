import React from 'react';
import { DiskMetrics } from '../types/metrics';
import { formatBytes, formatPercent, getUsageBackgroundColor } from '../utils/formatters';

interface DiskUsageProps {
  disks: DiskMetrics[];
}

/**
 * Disk usage component displaying all mounted disks
 */
export const DiskUsage: React.FC<DiskUsageProps> = ({ disks }) => {
  if (disks.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">Disk Usage</h2>
        <div className="text-gray-500 dark:text-gray-400">No disk information available</div>
      </div>
    );
  }

  const totalSpace = disks.reduce((sum, disk) => sum + disk.total, 0);
  const totalUsed = disks.reduce((sum, disk) => sum + disk.used, 0);
  const totalAvailable = disks.reduce((sum, disk) => sum + disk.available, 0);

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white">Disk Usage</h2>
        <div className="text-right">
          <div className="text-lg font-bold text-gray-800 dark:text-white">
            {formatBytes(totalUsed)} / {formatBytes(totalSpace)}
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Total Space</div>
        </div>
      </div>

      {/* Overall usage bar */}
      <div className="mb-6">
        <div className="flex justify-between text-sm text-gray-600 dark:text-gray-300 mb-2">
          <span>Overall Usage</span>
          <span>{formatPercent((totalUsed / totalSpace) * 100)}</span>
        </div>
        <div className="h-4 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
          <div
            className={`h-full transition-all duration-300 ${getUsageBackgroundColor(
              (totalUsed / totalSpace) * 100
            )}`}
            style={{ width: `${(totalUsed / totalSpace) * 100}%` }}
          />
        </div>
      </div>

      {/* Individual disks */}
      <div className="space-y-4">
        {disks.map((disk, index) => (
          <div key={index} className="border-l-4 border-purple-500 pl-4">
            <div className="flex items-start justify-between mb-2">
              <div className="flex-1">
                <div className="flex items-center space-x-2">
                  <h3 className="text-sm font-semibold text-gray-800 dark:text-white">{disk.name}</h3>
                  <span className="text-xs text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-0.5 rounded">
                    {disk.mount_point}
                  </span>
                </div>
                <div className="mt-1 text-xs text-gray-600 dark:text-gray-300">
                  {formatBytes(disk.used)} / {formatBytes(disk.total)} used
                  <span className="mx-2">•</span>
                  {formatBytes(disk.available)} available
                </div>
              </div>
              <div className="text-right ml-4">
                <div
                  className={`text-lg font-bold ${
                    disk.usage_percent >= 90
                      ? 'text-red-500'
                      : disk.usage_percent >= 70
                      ? 'text-yellow-500'
                      : 'text-green-500'
                  }`}
                >
                  {formatPercent(disk.usage_percent)}
                </div>
              </div>
            </div>

            {/* Usage bar */}
            <div className="h-3 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden mb-2">
              <div
                className={`h-full transition-all duration-300 ${getUsageBackgroundColor(
                  disk.usage_percent
                )}`}
                style={{ width: `${disk.usage_percent}%` }}
              />
            </div>

            {/* I/O stats */}
            <div className="flex justify-between text-xs text-gray-600 dark:text-gray-300 mt-2">
              <div className="flex items-center space-x-4">
                <div className="flex items-center">
                  <svg
                    className="w-4 h-4 mr-1 text-blue-500"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M7 16l-4-4m0 0l4-4m-4 4h18"
                    />
                  </svg>
                  <span>Read: {disk.read_mb.toFixed(2)} MB</span>
                </div>
                <div className="flex items-center">
                  <svg
                    className="w-4 h-4 mr-1 text-green-500"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M17 8l4 4m0 0l-4 4m4-4H3"
                    />
                  </svg>
                  <span>Write: {disk.write_mb.toFixed(2)} MB</span>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Summary stats */}
      <div className="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700 grid grid-cols-3 gap-4">
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-800 dark:text-white">{disks.length}</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Disks</div>
        </div>
        <div className="text-center">
          <div className="text-2xl font-bold text-blue-600">{formatBytes(totalUsed)}</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Used</div>
        </div>
        <div className="text-center">
          <div className="text-2xl font-bold text-green-600">{formatBytes(totalAvailable)}</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Available</div>
        </div>
      </div>
    </div>
  );
};
