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

interface TemperatureChartProps {
  history: SystemMetrics[];
}

/**
 * Temperature monitoring chart component
 * Displays temperature trends for CPU, NVMe, GPU, and other sensors
 */
export const TemperatureChart: React.FC<TemperatureChartProps> = ({ history }) => {
  const { theme } = useTheme();

  // Extract unique sensor labels and transform data for chart
  const { chartData, sensorLabels, currentTemps } = useMemo(() => {
    // Find all unique sensor labels across history
    const labelsSet = new Set<string>();
    history.forEach((metrics) => {
      metrics.temperatures?.forEach((temp) => {
        if (temp.label) {
          labelsSet.add(temp.label);
        }
      });
    });

    const labels = Array.from(labelsSet);

    // Transform history to chart format
    const data = history.map((metrics) => {
      const point: Record<string, string | number> = {
        time: format(new Date(metrics.timestamp), 'HH:mm:ss'),
      };

      // Create a map of label -> temperature for this timestamp
      const tempMap = new Map<string, number>();
      metrics.temperatures?.forEach((temp) => {
        if (temp.label && temp.value != null) {
          // If multiple sensors have same label, take max temperature
          const existing = tempMap.get(temp.label);
          if (existing === undefined || temp.value > existing) {
            tempMap.set(temp.label, temp.value);
          }
        }
      });

      // Add each sensor's temperature to the data point
      labels.forEach((label) => {
        const temp = tempMap.get(label);
        if (temp !== undefined) {
          point[label] = parseFloat(temp.toFixed(1));
        }
      });

      return point;
    });

    // Get current temperatures (latest snapshot)
    const current = new Map<string, number>();
    if (history.length > 0) {
      const latest = history[history.length - 1];
      latest.temperatures?.forEach((temp) => {
        if (temp.label && temp.value != null) {
          const existing = current.get(temp.label);
          if (existing === undefined || temp.value > existing) {
            current.set(temp.label, temp.value);
          }
        }
      });
    }

    return {
      chartData: data,
      sensorLabels: labels,
      currentTemps: current,
    };
  }, [history]);

  // Theme-aware colors
  const axisColor = theme === 'dark' ? '#9ca3af' : '#6b7280';
  const gridColor = theme === 'dark' ? '#374151' : '#e5e7eb';
  const tooltipBg = theme === 'dark' ? 'rgba(31, 41, 55, 0.95)' : 'rgba(255, 255, 255, 0.95)';
  const tooltipBorder = theme === 'dark' ? '#4b5563' : '#e5e7eb';
  const tooltipText = theme === 'dark' ? '#f3f4f6' : '#111827';

  // Color mapping for different sensor types
  const getSensorColor = (label: string): string => {
    const lowerLabel = label.toLowerCase();
    if (lowerLabel.includes('cpu') || lowerLabel.includes('k10temp') || lowerLabel.includes('coretemp')) {
      return '#ef4444'; // Red for CPU (hot)
    }
    if (lowerLabel.includes('nvme') || lowerLabel.includes('ssd')) {
      return '#3b82f6'; // Blue for NVMe (cooler)
    }
    if (lowerLabel.includes('gpu') || lowerLabel.includes('amdgpu') || lowerLabel.includes('nvidia')) {
      return '#a855f7'; // Purple for GPU
    }
    if (lowerLabel.includes('disk') || lowerLabel.includes('hdd')) {
      return '#06b6d4'; // Cyan for disks
    }
    // Fallback colors for other sensors
    const colors = ['#10b981', '#f59e0b', '#ec4899', '#8b5cf6', '#14b8a6'];
    const index = Array.from(sensorLabels).indexOf(label) % colors.length;
    return colors[index];
  };

  // Handle empty data
  if (chartData.length === 0 || sensorLabels.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">Temperature</h2>
        <div className="flex items-center justify-center h-64 text-gray-500 dark:text-gray-400">
          <div className="text-center">
            <svg
              className="w-12 h-12 mx-auto mb-3 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
              />
            </svg>
            <p>Temperature monitoring not available</p>
            <p className="text-sm mt-1">No temperature sensors detected on this system</p>
          </div>
        </div>
      </div>
    );
  }

  // Limit to top 5 sensors by current temperature if too many
  const displaySensors = sensorLabels.length > 5
    ? Array.from(currentTemps.entries())
        .sort((a, b) => b[1] - a[1])
        .slice(0, 5)
        .map(([label]) => label)
    : sensorLabels;

  // Find highest current temperature for display
  const maxTemp = Math.max(...Array.from(currentTemps.values()));
  const maxTempLabel = Array.from(currentTemps.entries()).find(([_, temp]) => temp === maxTemp)?.[0] || '';

  // Custom tooltip formatter
  interface TooltipPayload {
    name: string;
    value: number;
    color: string;
  }

  interface TooltipProps {
    active?: boolean;
    payload?: TooltipPayload[];
    label?: string;
  }

  const CustomTooltip: React.FC<TooltipProps> = ({ active, payload, label }) => {
    if (active && payload && payload.length) {
      return (
        <div
          style={{
            backgroundColor: tooltipBg,
            border: `1px solid ${tooltipBorder}`,
            borderRadius: '0.375rem',
            padding: '0.5rem',
          }}
        >
          <p style={{ color: tooltipText, marginBottom: '0.25rem', fontSize: '0.75rem' }}>{label}</p>
          {payload.map((entry, index) => (
            <p key={index} style={{ color: entry.color, fontSize: '0.875rem', margin: '0.125rem 0' }}>
              {entry.name}: {entry.value}°C
            </p>
          ))}
        </div>
      );
    }
    return null;
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white">Temperature</h2>
        <div className="text-right">
          <div className="text-3xl font-bold text-red-600">{maxTemp.toFixed(1)}°C</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">{maxTempLabel || 'Max Temperature'}</div>
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
            stroke={axisColor}
            tick={{ fontSize: 12, fill: axisColor }}
            domain={[0, 'auto']}
            label={{ value: 'Temperature (°C)', angle: -90, position: 'insideLeft', fill: axisColor }}
          />
          <Tooltip content={<CustomTooltip />} />
          <Legend
            wrapperStyle={{ color: axisColor }}
            iconType="line"
          />
          {displaySensors.map((label) => (
            <Line
              key={label}
              type="monotone"
              dataKey={label}
              stroke={getSensorColor(label)}
              strokeWidth={2}
              dot={false}
              name={label}
              connectNulls
            />
          ))}
        </LineChart>
      </ResponsiveContainer>

      {/* Current temperatures for all sensors */}
      {currentTemps.size > 0 && (
        <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Current Temperatures</h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
            {Array.from(currentTemps.entries())
              .sort((a, b) => b[1] - a[1]) // Sort by temperature descending
              .map(([label, temp]) => {
                const isHot = temp >= 75;
                const isWarm = temp >= 60 && temp < 75;
                const bgColor = isHot
                  ? 'bg-red-100 dark:bg-red-900/30'
                  : isWarm
                  ? 'bg-yellow-100 dark:bg-yellow-900/30'
                  : 'bg-green-100 dark:bg-green-900/30';
                const textColor = isHot
                  ? 'text-red-700 dark:text-red-400'
                  : isWarm
                  ? 'text-yellow-700 dark:text-yellow-400'
                  : 'text-green-700 dark:text-green-400';

                return (
                  <div key={label} className={`${bgColor} rounded p-3`}>
                    <div className="text-xs text-gray-600 dark:text-gray-400 mb-1">{label}</div>
                    <div className={`text-lg font-bold ${textColor}`}>{temp.toFixed(1)}°C</div>
                  </div>
                );
              })}
          </div>
          {sensorLabels.length > 5 && (
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-2">
              Showing top 5 sensors in chart. All {sensorLabels.length} sensors displayed above.
            </p>
          )}
        </div>
      )}
    </div>
  );
};
