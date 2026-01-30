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
import { useTheme } from '../hooks/useTheme';
import { format } from 'date-fns';

interface GpuChartProps {
  history: SystemMetrics[];
}

/**
 * GPU usage and temperature chart component
 */
export const GpuChart: React.FC<GpuChartProps> = ({ history }) => {
  const { theme } = useTheme();

  const chartData = useMemo(() => {
    return history
      .filter((metrics) => metrics.gpu !== null)
      .map((metrics) => ({
        time: format(new Date(metrics.timestamp), 'HH:mm:ss'),
        usage: parseFloat((metrics.gpu?.usage_percent ?? 0).toFixed(1)),
        temperature: parseFloat((metrics.gpu?.temperature ?? 0).toFixed(1)),
        memoryUsage: parseFloat((metrics.gpu?.memory_usage_percent ?? 0).toFixed(1)),
        power: parseFloat((metrics.gpu?.power_draw_watts ?? 0).toFixed(1)),
      }));
  }, [history]);

  // Get current GPU info
  const currentGpu = history.length > 0 ? history[history.length - 1].gpu : null;

  if (!currentGpu || chartData.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">GPU</h2>
        <div className="text-gray-500 dark:text-gray-400">No GPU data available</div>
      </div>
    );
  }

  const latestUsage = chartData[chartData.length - 1]?.usage ?? 0;
  const latestTemp = chartData[chartData.length - 1]?.temperature ?? 0;

  // Theme-aware colors
  const axisColor = theme === 'dark' ? '#9ca3af' : '#6b7280';
  const gridColor = theme === 'dark' ? '#374151' : '#e5e7eb';
  const tooltipBg = theme === 'dark' ? 'rgba(31, 41, 55, 0.95)' : 'rgba(255, 255, 255, 0.95)';
  const tooltipBorder = theme === 'dark' ? '#4b5563' : '#e5e7eb';
  const tooltipText = theme === 'dark' ? '#f3f4f6' : '#111827';

  // Temperature color based on value
  const getTempColor = (temp: number) => {
    if (temp >= 85) return 'text-red-600';
    if (temp >= 70) return 'text-yellow-600';
    return 'text-green-600';
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h2 className="text-xl font-semibold text-gray-800 dark:text-white">GPU</h2>
          <p className="text-sm text-gray-500 dark:text-gray-400 truncate max-w-xs">
            {currentGpu.name}
          </p>
        </div>
        <div className="text-right">
          <div className="text-3xl font-bold text-purple-600">{latestUsage.toFixed(1)}%</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Current Usage</div>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke={gridColor} />
          <XAxis
            dataKey="time"
            stroke={axisColor}
            tick={{ fontSize: 12, fill: axisColor }}
            interval="preserveStartEnd"
          />
          <YAxis
            yAxisId="left"
            stroke={axisColor}
            tick={{ fontSize: 12, fill: axisColor }}
            domain={[0, 100]}
            label={{ value: 'Usage %', angle: -90, position: 'insideLeft', fill: axisColor }}
          />
          <YAxis
            yAxisId="right"
            orientation="right"
            stroke={axisColor}
            tick={{ fontSize: 12, fill: axisColor }}
            domain={[0, 100]}
            label={{ value: 'Temp °C', angle: 90, position: 'insideRight', fill: axisColor }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: tooltipBg,
              border: `1px solid ${tooltipBorder}`,
              borderRadius: '0.375rem',
              color: tooltipText,
            }}
            labelStyle={{ color: tooltipText }}
            itemStyle={{ color: tooltipText }}
          />
          <Legend wrapperStyle={{ color: axisColor }} iconType="line" />
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="usage"
            stroke="#9333ea"
            strokeWidth={2}
            dot={false}
            name="GPU Usage %"
          />
          <Line
            yAxisId="right"
            type="monotone"
            dataKey="temperature"
            stroke="#ef4444"
            strokeWidth={2}
            dot={false}
            name="Temperature °C"
          />
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="memoryUsage"
            stroke="#06b6d4"
            strokeWidth={2}
            dot={false}
            name="VRAM Usage %"
            strokeDasharray="5 5"
          />
        </LineChart>
      </ResponsiveContainer>

      {/* GPU Stats Grid */}
      <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {/* Temperature */}
          <div className="text-center p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <div className={`text-2xl font-bold ${getTempColor(latestTemp)}`}>
              {latestTemp.toFixed(0)}°C
            </div>
            <div className="text-xs text-gray-500 dark:text-gray-400">Temperature</div>
          </div>

          {/* VRAM Usage */}
          <div className="text-center p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <div className="text-2xl font-bold text-cyan-600">
              {currentGpu.memory_used_mb}
              <span className="text-sm font-normal text-gray-500 dark:text-gray-400">
                /{currentGpu.memory_total_mb} MB
              </span>
            </div>
            <div className="text-xs text-gray-500 dark:text-gray-400">VRAM</div>
          </div>

          {/* Power Draw */}
          <div className="text-center p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <div className="text-2xl font-bold text-orange-600">
              {currentGpu.power_draw_watts.toFixed(1)}W
            </div>
            <div className="text-xs text-gray-500 dark:text-gray-400">Power</div>
          </div>

          {/* Fan Speed */}
          <div className="text-center p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <div className="text-2xl font-bold text-blue-600">
              {currentGpu.fan_speed_percent.toFixed(0)}%
            </div>
            <div className="text-xs text-gray-500 dark:text-gray-400">Fan Speed</div>
          </div>
        </div>
      </div>
    </div>
  );
};
