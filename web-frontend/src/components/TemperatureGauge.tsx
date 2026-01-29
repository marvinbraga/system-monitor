import React from 'react';
import { Temperature } from '../types/metrics';
import { formatTemperature, getTemperatureColor } from '../utils/formatters';

interface TemperatureGaugeProps {
  temperatures: Temperature[];
}

/**
 * Temperature gauge component displaying all sensors
 */
export const TemperatureGauge: React.FC<TemperatureGaugeProps> = ({ temperatures }) => {
  if (temperatures.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-white">Temperature</h2>
        <div className="text-gray-500 dark:text-gray-400">No temperature sensors available</div>
      </div>
    );
  }

  const maxTemp = Math.max(...temperatures.map((t) => t.value));
  const avgTemp = temperatures.reduce((sum, t) => sum + t.value, 0) / temperatures.length;

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white">Temperature</h2>
        <div className="text-right">
          <div className={`text-3xl font-bold ${getTemperatureColor(maxTemp)}`}>
            {formatTemperature(maxTemp)}
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Maximum</div>
        </div>
      </div>

      {/* Summary */}
      <div className="grid grid-cols-2 gap-4 mb-6">
        <div className="bg-gradient-to-br from-orange-50 to-red-50 dark:from-orange-900/20 dark:to-red-900/20 rounded-lg p-4">
          <div className="text-sm text-gray-600 dark:text-gray-300 mb-1">Average</div>
          <div className={`text-2xl font-bold ${getTemperatureColor(avgTemp)}`}>
            {formatTemperature(avgTemp)}
          </div>
        </div>
        <div className="bg-gradient-to-br from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-lg p-4">
          <div className="text-sm text-gray-600 dark:text-gray-300 mb-1">Sensors</div>
          <div className="text-2xl font-bold text-gray-800 dark:text-white">{temperatures.length}</div>
        </div>
      </div>

      {/* Temperature sensors list */}
      <div className="space-y-3">
        {temperatures.map((temp, index) => {
          const percentage = Math.min((temp.value / 100) * 100, 100);
          const color = getTemperatureColor(temp.value);

          return (
            <div key={index} className="border-l-4 border-gray-200 dark:border-gray-700 pl-3">
              <div className="flex items-center justify-between mb-1">
                <div>
                  <div className="text-sm font-medium text-gray-700 dark:text-gray-300">{temp.label}</div>
                  <div className="text-xs text-gray-500 dark:text-gray-400">{temp.sensor}</div>
                </div>
                <div className={`text-lg font-bold ${color}`}>
                  {formatTemperature(temp.value)}
                </div>
              </div>

              {/* Temperature bar */}
              <div className="h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                <div
                  className={`h-full transition-all duration-300 ${
                    temp.value >= 80
                      ? 'bg-red-500'
                      : temp.value >= 70
                      ? 'bg-orange-500'
                      : temp.value >= 60
                      ? 'bg-yellow-500'
                      : 'bg-green-500'
                  }`}
                  style={{ width: `${percentage}%` }}
                />
              </div>
            </div>
          );
        })}
      </div>

      {/* Legend */}
      <div className="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between text-xs">
          <div className="flex items-center space-x-4">
            <div className="flex items-center">
              <div className="w-3 h-3 bg-green-500 rounded-full mr-1" />
              <span className="text-gray-600 dark:text-gray-300">Normal (&lt;60°C)</span>
            </div>
            <div className="flex items-center">
              <div className="w-3 h-3 bg-yellow-500 rounded-full mr-1" />
              <span className="text-gray-600 dark:text-gray-300">Warm (60-70°C)</span>
            </div>
            <div className="flex items-center">
              <div className="w-3 h-3 bg-orange-500 rounded-full mr-1" />
              <span className="text-gray-600 dark:text-gray-300">Hot (70-80°C)</span>
            </div>
            <div className="flex items-center">
              <div className="w-3 h-3 bg-red-500 rounded-full mr-1" />
              <span className="text-gray-600 dark:text-gray-300">Critical (&gt;80°C)</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
