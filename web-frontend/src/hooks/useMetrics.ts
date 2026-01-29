import { useState, useEffect, useCallback } from 'react';
import { SystemMetrics, Anomaly } from '../types/metrics';
import { apiClient } from '../api/client';
import { useWebSocket } from './useWebSocket';

/**
 * React hook for managing system metrics state
 */
export function useMetrics() {
  const [currentMetrics, setCurrentMetrics] = useState<SystemMetrics | null>(null);
  const [metricsHistory, setMetricsHistory] = useState<SystemMetrics[]>([]);
  const [anomalies, setAnomalies] = useState<Anomaly[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const { lastMessage, isConnected } = useWebSocket();

  // Fetch initial data
  const fetchInitialData = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const [metrics, history, anomaliesData] = await Promise.all([
        apiClient.getCurrentMetrics(),
        apiClient.getMetricsHistory(60), // Last 60 records
        apiClient.getAnomalies(50), // Last 50 anomalies
      ]);

      setCurrentMetrics(metrics);
      setMetricsHistory(history);
      setAnomalies(anomaliesData);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch data'));
      console.error('Failed to fetch initial data:', err);
    } finally {
      setLoading(false);
    }
  }, []);

  // Update metrics from WebSocket
  useEffect(() => {
    if (lastMessage) {
      if (lastMessage.type === 'metrics') {
        const newMetrics = lastMessage.data as SystemMetrics;
        setCurrentMetrics(newMetrics);

        // Add to history (keep last 60 records)
        setMetricsHistory((prev) => {
          const updated = [...prev, newMetrics];
          return updated.slice(-60);
        });
      } else if (lastMessage.type === 'anomaly') {
        const newAnomaly = lastMessage.data as Anomaly;
        setAnomalies((prev) => [newAnomaly, ...prev].slice(0, 50));
      }
    }
  }, [lastMessage]);

  // Fetch initial data on mount
  useEffect(() => {
    fetchInitialData();
  }, [fetchInitialData]);

  // Refresh data manually
  const refresh = useCallback(async () => {
    await fetchInitialData();
  }, [fetchInitialData]);

  return {
    currentMetrics,
    metricsHistory,
    anomalies,
    loading,
    error,
    isConnected,
    refresh,
  };
}

/**
 * React hook for filtered anomalies
 */
export function useFilteredAnomalies(
  anomalies: Anomaly[],
  filters: {
    severity?: string[];
    category?: string[];
    search?: string;
  }
) {
  const [filtered, setFiltered] = useState<Anomaly[]>(anomalies);

  useEffect(() => {
    let result = [...anomalies];

    // Filter by severity
    if (filters.severity && filters.severity.length > 0) {
      result = result.filter((a) => filters.severity!.includes(a.severity));
    }

    // Filter by category
    if (filters.category && filters.category.length > 0) {
      result = result.filter((a) => filters.category!.includes(a.category));
    }

    // Filter by search text
    if (filters.search && filters.search.trim()) {
      const searchLower = filters.search.toLowerCase();
      result = result.filter(
        (a) =>
          a.message.toLowerCase().includes(searchLower) ||
          a.category.toLowerCase().includes(searchLower)
      );
    }

    setFiltered(result);
  }, [anomalies, filters]);

  return filtered;
}
