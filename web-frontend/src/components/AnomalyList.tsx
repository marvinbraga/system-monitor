import React, { useState } from 'react';
import { Anomaly, AnomalySeverity, AnomalyCategory } from '../types/metrics';
import { getSeverityBadgeColor, formatRelativeTime } from '../utils/formatters';

interface AnomalyListProps {
  anomalies: Anomaly[];
}

/**
 * Anomaly list component with filtering
 */
export const AnomalyList: React.FC<AnomalyListProps> = ({ anomalies }) => {
  const [selectedSeverity, setSelectedSeverity] = useState<string[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string[]>([]);
  const [searchTerm, setSearchTerm] = useState('');

  // Filter anomalies
  const filteredAnomalies = anomalies.filter((anomaly) => {
    // Filter by severity
    if (selectedSeverity.length > 0 && !selectedSeverity.includes(anomaly.severity)) {
      return false;
    }

    // Filter by category
    if (selectedCategory.length > 0 && !selectedCategory.includes(anomaly.category)) {
      return false;
    }

    // Filter by search term
    if (searchTerm.trim()) {
      const searchLower = searchTerm.toLowerCase();
      return (
        anomaly.message.toLowerCase().includes(searchLower) ||
        anomaly.category.toLowerCase().includes(searchLower) ||
        anomaly.severity.toLowerCase().includes(searchLower)
      );
    }

    return true;
  });

  const toggleFilter = (type: 'severity' | 'category', value: string) => {
    if (type === 'severity') {
      setSelectedSeverity((prev) =>
        prev.includes(value) ? prev.filter((v) => v !== value) : [...prev, value]
      );
    } else {
      setSelectedCategory((prev) =>
        prev.includes(value) ? prev.filter((v) => v !== value) : [...prev, value]
      );
    }
  };

  const countBySeverity = (severity: string) =>
    anomalies.filter((a) => a.severity === severity).length;

  const countByCategory = (category: string) =>
    anomalies.filter((a) => a.category === category).length;

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold text-gray-800">Anomalies</h2>
        <div className="text-sm text-gray-500">
          {filteredAnomalies.length} of {anomalies.length}
        </div>
      </div>

      {/* Search */}
      <div className="mb-4">
        <input
          type="text"
          placeholder="Search anomalies..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
      </div>

      {/* Filters */}
      <div className="mb-4 space-y-3">
        {/* Severity filters */}
        <div>
          <div className="text-xs font-medium text-gray-700 mb-2">Severity</div>
          <div className="flex flex-wrap gap-2">
            {Object.values(AnomalySeverity).map((severity) => {
              const count = countBySeverity(severity);
              const isSelected = selectedSeverity.includes(severity);
              return (
                <button
                  key={severity}
                  onClick={() => toggleFilter('severity', severity)}
                  className={`px-3 py-1 text-xs font-medium rounded-full border transition-colors ${
                    isSelected
                      ? getSeverityBadgeColor(severity as any)
                      : 'bg-white text-gray-600 border-gray-300 hover:bg-gray-50'
                  }`}
                >
                  {severity} ({count})
                </button>
              );
            })}
          </div>
        </div>

        {/* Category filters */}
        <div>
          <div className="text-xs font-medium text-gray-700 mb-2">Category</div>
          <div className="flex flex-wrap gap-2">
            {Object.values(AnomalyCategory).map((category) => {
              const count = countByCategory(category);
              const isSelected = selectedCategory.includes(category);
              return (
                <button
                  key={category}
                  onClick={() => toggleFilter('category', category)}
                  className={`px-3 py-1 text-xs font-medium rounded-full border transition-colors ${
                    isSelected
                      ? 'bg-purple-100 text-purple-800 border-purple-200'
                      : 'bg-white text-gray-600 border-gray-300 hover:bg-gray-50'
                  }`}
                >
                  {category} ({count})
                </button>
              );
            })}
          </div>
        </div>

        {/* Clear filters */}
        {(selectedSeverity.length > 0 || selectedCategory.length > 0 || searchTerm) && (
          <button
            onClick={() => {
              setSelectedSeverity([]);
              setSelectedCategory([]);
              setSearchTerm('');
            }}
            className="text-xs text-blue-600 hover:text-blue-800 font-medium"
          >
            Clear all filters
          </button>
        )}
      </div>

      {/* Anomaly list */}
      <div className="space-y-3 max-h-96 overflow-y-auto">
        {filteredAnomalies.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            {anomalies.length === 0 ? 'No anomalies detected' : 'No anomalies match your filters'}
          </div>
        ) : (
          filteredAnomalies.map((anomaly) => (
            <div
              key={anomaly.id}
              className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
            >
              <div className="flex items-start justify-between mb-2">
                <div className="flex items-center space-x-2">
                  <span
                    className={`px-2 py-1 text-xs font-medium rounded border ${getSeverityBadgeColor(
                      anomaly.severity as any
                    )}`}
                  >
                    {anomaly.severity}
                  </span>
                  <span className="px-2 py-1 text-xs font-medium rounded border bg-purple-100 text-purple-800 border-purple-200">
                    {anomaly.category}
                  </span>
                </div>
                <div className="text-xs text-gray-500">{formatRelativeTime(anomaly.timestamp)}</div>
              </div>

              <p className="text-sm text-gray-800 mb-2">{anomaly.message}</p>

              {/* Metrics details */}
              {Object.keys(anomaly.metrics).length > 0 && (
                <details className="mt-2">
                  <summary className="text-xs text-gray-600 cursor-pointer hover:text-gray-800">
                    View metrics
                  </summary>
                  <pre className="mt-2 text-xs bg-gray-50 p-2 rounded overflow-x-auto">
                    {JSON.stringify(anomaly.metrics, null, 2)}
                  </pre>
                </details>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};
