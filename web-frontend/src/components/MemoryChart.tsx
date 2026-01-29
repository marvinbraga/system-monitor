import React, { useMemo } from 'react';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';
import { SystemMetrics } from '../types/metrics';
import { formatBytes, formatPercent } from '../utils/formatters';
import { format } from 'date-fns';

interface MemoryChartProps {
  history: SystemMetrics[];
}

/**
 * Memory usage chart component
 */
export const MemoryChart: React.FC<MemoryChartProps> = ({ history }) => {
  const chartData = useMemo(() => {
    return history.map((metrics) => ({
      time: format(new Date(metrics.timestamp), 'HH:mm:ss'),
      usedGB: parseFloat((metrics.memory.used / (1024 * 1024 * 1024)).toFixed(2)),
      totalGB: parseFloat((metrics.memory.total / (1024 * 1024 * 1024)).toFixed(2)),
      usagePercent: parseFloat(metrics.memory.usage_percent.toFixed(2)),
    }));
  }, [history]);

  if (chartData.length === 0 || history.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4">Memory Usage</h2>
        <div className="text-gray-500 dark:text-gray-400">No data available</div>
      </div>
    );
  }

  const latestMetrics = history[history.length - 1].memory;

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white">Memory Usage</h2>
        <div className="text-right">
          <div className="text-3xl font-bold text-green-600">
            {formatPercent(latestMetrics.usage_percent)}
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">
            {formatBytes(latestMetrics.used)} / {formatBytes(latestMetrics.total)}
          </div>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={300}>
        <AreaChart data={chartData}>
          <defs>
            <linearGradient id="memoryGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stopColor="#10b981" stopOpacity={0.8} />
              <stop offset="95%" stopColor="#10b981" stopOpacity={0.1} />
            </linearGradient>
          </defs>
          <CartesianGrid strokeDasharray="3 3" stroke="currentColor" opacity={0.1} />
          <XAxis
            dataKey="time"
            stroke="currentColor"
            tick={{ fontSize: 12, fill: 'currentColor' }}
            interval="preserveStartEnd"
          />
          <YAxis
            stroke="currentColor"
            tick={{ fontSize: 12, fill: 'currentColor' }}
            domain={[0, 'dataMax']}
            label={{ value: 'Memory (GB)', angle: -90, position: 'insideLeft', fill: 'currentColor' }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '0.375rem',
            }}
            wrapperClassName="dark:[&_.recharts-tooltip-wrapper]:opacity-100"
            labelClassName="dark:text-gray-900"
            itemStyle={{ color: '#111827' }}
            formatter={(value: number, name: string) => {
              if (name === 'usagePercent') {
                return [`${value.toFixed(2)}%`, 'Usage %'];
              }
              return [`${value.toFixed(2)} GB`, name === 'usedGB' ? 'Used' : 'Total'];
            }}
          />
          <Area
            type="monotone"
            dataKey="usedGB"
            stroke="#10b981"
            strokeWidth={2}
            fill="url(#memoryGradient)"
          />
        </AreaChart>
      </ResponsiveContainer>

      {/* Memory breakdown */}
      <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
        <div className="grid grid-cols-2 gap-4">
          <div>
            <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">RAM</h3>
            <div className="space-y-1">
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">Used:</span>
                <span className="font-medium text-gray-800 dark:text-white">{formatBytes(latestMetrics.used)}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">Available:</span>
                <span className="font-medium text-gray-800 dark:text-white">{formatBytes(latestMetrics.available)}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">Total:</span>
                <span className="font-medium text-gray-800 dark:text-white">{formatBytes(latestMetrics.total)}</span>
              </div>
            </div>
          </div>

          <div>
            <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Swap</h3>
            <div className="space-y-1">
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">Used:</span>
                <span className="font-medium text-gray-800 dark:text-white">{formatBytes(latestMetrics.swap_used)}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">Free:</span>
                <span className="font-medium text-gray-800 dark:text-white">
                  {formatBytes(latestMetrics.swap_total - latestMetrics.swap_used)}
                </span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">Total:</span>
                <span className="font-medium text-gray-800 dark:text-white">{formatBytes(latestMetrics.swap_total)}</span>
              </div>
            </div>
          </div>
        </div>

        {/* Visual bar */}
        <div className="mt-4">
          <div className="h-4 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
            <div
              className="h-full bg-green-500 dark:bg-green-600 transition-all duration-300"
              style={{ width: `${latestMetrics.usage_percent}%` }}
            />
          </div>
        </div>
      </div>
    </div>
  );
};
