import React, { useMemo } from 'react';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { SystemMetrics } from '../types/metrics';
import { format } from 'date-fns';

interface CpuChartProps {
  history: SystemMetrics[];
}

/**
 * CPU usage chart component
 */
export const CpuChart: React.FC<CpuChartProps> = ({ history }) => {
  const chartData = useMemo(() => {
    return history.map((metrics) => ({
      time: format(new Date(metrics.timestamp), 'HH:mm:ss'),
      usage: parseFloat(metrics.cpu.global_usage.toFixed(2)),
      load1: parseFloat(metrics.cpu.load_avg_1.toFixed(2)),
    }));
  }, [history]);

  if (chartData.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4">CPU Usage</h2>
        <div className="text-gray-500 dark:text-gray-400">No data available</div>
      </div>
    );
  }

  const latestUsage = chartData[chartData.length - 1]?.usage ?? 0;

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white">CPU Usage</h2>
        <div className="text-right">
          <div className="text-3xl font-bold text-blue-600">{latestUsage.toFixed(1)}%</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Current Usage</div>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
          <XAxis
            dataKey="time"
            stroke="#9ca3af"
            tick={{ fontSize: 12 }}
            interval="preserveStartEnd"
          />
          <YAxis
            stroke="#9ca3af"
            tick={{ fontSize: 12 }}
            domain={[0, 100]}
            label={{ value: 'Usage %', angle: -90, position: 'insideLeft' }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '0.375rem',
            }}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey="usage"
            stroke="#3b82f6"
            strokeWidth={2}
            dot={false}
            name="CPU Usage %"
          />
          <Line
            type="monotone"
            dataKey="load1"
            stroke="#10b981"
            strokeWidth={2}
            dot={false}
            name="Load Average (1m)"
            strokeDasharray="5 5"
          />
        </LineChart>
      </ResponsiveContainer>

      {/* Per-core usage */}
      {history.length > 0 && history[history.length - 1].cpu.per_core.length > 0 && (
        <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Per-Core Usage</h3>
          <div className="grid grid-cols-4 md:grid-cols-8 gap-2">
            {history[history.length - 1].cpu.per_core.map((usage, index) => (
              <div key={index} className="text-center">
                <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">Core {index}</div>
                <div className="relative h-16 bg-gray-100 dark:bg-gray-700 rounded">
                  <div
                    className="absolute bottom-0 w-full bg-blue-500 dark:bg-blue-600 rounded transition-all duration-300"
                    style={{ height: `${usage}%` }}
                  />
                  <div className="absolute inset-0 flex items-center justify-center text-xs font-medium text-gray-900 dark:text-white">
                    {usage.toFixed(0)}%
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};
