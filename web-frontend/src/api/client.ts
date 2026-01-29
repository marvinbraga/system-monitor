import axios, { AxiosInstance } from 'axios';
import { SystemMetrics, Anomaly } from '../types/metrics';

/**
 * API client for system monitor backend
 */
class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string = '/api/v1') {
    this.client = axios.create({
      baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add response interceptor for error handling
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        console.error('API Error:', error);
        return Promise.reject(error);
      }
    );
  }

  /**
   * Get current system metrics
   */
  async getCurrentMetrics(): Promise<SystemMetrics> {
    const response = await this.client.get<{data: SystemMetrics, status: string}>('/metrics/current');
    return response.data.data;
  }

  /**
   * Get metrics history for a time range
   * @param limit - Number of records to fetch (default: 100)
   */
  async getMetricsHistory(limit: number = 100): Promise<SystemMetrics[]> {
    const response = await this.client.get<{data: {metrics: SystemMetrics[]}, status: string}>('/metrics/history', {
      params: { limit },
    });
    return response.data.data.metrics;
  }

  /**
   * Get metrics history within a time range
   * @param start - Start timestamp (ISO 8601)
   * @param end - End timestamp (ISO 8601)
   */
  async getMetricsRange(start: string, end: string): Promise<SystemMetrics[]> {
    const response = await this.client.get<SystemMetrics[]>('/metrics/range', {
      params: { start, end },
    });
    return response.data;
  }

  /**
   * Get recent anomalies
   * @param limit - Number of anomalies to fetch (default: 50)
   */
  async getAnomalies(limit: number = 50): Promise<Anomaly[]> {
    const response = await this.client.get<{data: {anomalies: Anomaly[]}, status: string}>('/anomalies', {
      params: { limit },
    });
    return response.data.data.anomalies;
  }

  /**
   * Get anomalies by severity
   * @param severity - Severity level (Info, Warning, Critical)
   * @param limit - Number of anomalies to fetch
   */
  async getAnomaliesBySeverity(
    severity: string,
    limit: number = 50
  ): Promise<Anomaly[]> {
    const response = await this.client.get<Anomaly[]>('/anomalies/severity', {
      params: { severity, limit },
    });
    return response.data;
  }

  /**
   * Get anomalies by category
   * @param category - Category (Cpu, Memory, Temperature, etc.)
   * @param limit - Number of anomalies to fetch
   */
  async getAnomaliesByCategory(
    category: string,
    limit: number = 50
  ): Promise<Anomaly[]> {
    const response = await this.client.get<Anomaly[]>('/anomalies/category', {
      params: { category, limit },
    });
    return response.data;
  }

  /**
   * Get system statistics
   */
  async getSystemStats(): Promise<{
    total_metrics: number;
    total_anomalies: number;
    critical_anomalies: number;
    warning_anomalies: number;
  }> {
    const response = await this.client.get('/stats');
    return response.data;
  }

  /**
   * Health check
   */
  async healthCheck(): Promise<{ status: string; timestamp: string }> {
    const response = await this.client.get('/health');
    return response.data;
  }
}

// Export singleton instance
export const apiClient = new ApiClient();

// Export class for testing
export default ApiClient;
