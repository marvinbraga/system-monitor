/**
 * Utility functions for formatting system metrics data
 */

/**
 * Format bytes to human-readable format
 * @param bytes - Number of bytes
 * @param decimals - Number of decimal places
 * @returns Formatted string (e.g., "1.5 GB")
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}

/**
 * Format percentage with specified decimal places
 * @param value - Percentage value (0-100)
 * @param decimals - Number of decimal places
 * @returns Formatted string (e.g., "75.5%")
 */
export function formatPercent(value: number, decimals: number = 1): string {
  return `${value.toFixed(decimals)}%`;
}

/**
 * Format temperature
 * @param celsius - Temperature in Celsius
 * @returns Formatted string (e.g., "45.5°C")
 */
export function formatTemperature(celsius: number): string {
  return `${celsius.toFixed(1)}°C`;
}

/**
 * Format network speed (bytes per second)
 * @param bytesPerSecond - Bytes per second
 * @returns Formatted string (e.g., "1.5 MB/s")
 */
export function formatNetworkSpeed(bytesPerSecond: number): string {
  return `${formatBytes(bytesPerSecond)}/s`;
}

/**
 * Get color class based on usage percentage
 * @param percent - Usage percentage (0-100)
 * @param thresholds - Custom thresholds { warning, danger }
 * @returns Tailwind color class
 */
export function getUsageColor(
  percent: number,
  thresholds = { warning: 70, danger: 90 }
): string {
  if (percent >= thresholds.danger) return 'text-red-500';
  if (percent >= thresholds.warning) return 'text-yellow-500';
  return 'text-green-500';
}

/**
 * Get background color class based on usage percentage
 * @param percent - Usage percentage (0-100)
 * @param thresholds - Custom thresholds { warning, danger }
 * @returns Tailwind background color class
 */
export function getUsageBackgroundColor(
  percent: number,
  thresholds = { warning: 70, danger: 90 }
): string {
  if (percent >= thresholds.danger) return 'bg-red-500';
  if (percent >= thresholds.warning) return 'bg-yellow-500';
  return 'bg-green-500';
}

/**
 * Get temperature color class
 * @param celsius - Temperature in Celsius
 * @returns Tailwind color class
 */
export function getTemperatureColor(celsius: number): string {
  if (celsius >= 80) return 'text-red-500';
  if (celsius >= 70) return 'text-orange-500';
  if (celsius >= 60) return 'text-yellow-500';
  return 'text-green-500';
}

/**
 * Format uptime duration
 * @param seconds - Uptime in seconds
 * @returns Formatted string (e.g., "2d 5h 30m")
 */
export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  const parts: string[] = [];
  if (days > 0) parts.push(`${days}d`);
  if (hours > 0) parts.push(`${hours}h`);
  if (minutes > 0 || parts.length === 0) parts.push(`${minutes}m`);

  return parts.join(' ');
}

/**
 * Truncate string to specified length
 * @param str - Input string
 * @param maxLength - Maximum length
 * @returns Truncated string with ellipsis
 */
export function truncate(str: string, maxLength: number): string {
  if (str.length <= maxLength) return str;
  return str.slice(0, maxLength - 3) + '...';
}

/**
 * Get severity badge color
 * @param severity - Anomaly severity
 * @returns Tailwind badge classes
 */
export function getSeverityBadgeColor(
  severity: 'Info' | 'Warning' | 'Critical'
): string {
  switch (severity) {
    case 'Info':
      return 'bg-blue-100 text-blue-800 border-blue-200';
    case 'Warning':
      return 'bg-yellow-100 text-yellow-800 border-yellow-200';
    case 'Critical':
      return 'bg-red-100 text-red-800 border-red-200';
    default:
      return 'bg-gray-100 text-gray-800 border-gray-200';
  }
}

/**
 * Format relative time
 * @param timestamp - ISO 8601 timestamp
 * @returns Relative time string (e.g., "2 minutes ago")
 */
export function formatRelativeTime(timestamp: string): string {
  const now = new Date();
  const past = new Date(timestamp);
  const diffMs = now.getTime() - past.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return 'just now';
  if (diffMin < 60) return `${diffMin} minute${diffMin !== 1 ? 's' : ''} ago`;
  if (diffHour < 24) return `${diffHour} hour${diffHour !== 1 ? 's' : ''} ago`;
  return `${diffDay} day${diffDay !== 1 ? 's' : ''} ago`;
}
