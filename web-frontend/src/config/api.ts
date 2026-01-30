/**
 * API Configuration
 *
 * In Docker: uses VITE_API_URL env var (http://host.docker.internal:5253)
 * In development: uses localhost:5253
 */
export const API_CONFIG = {
  BASE_URL: import.meta.env.VITE_API_URL || 'http://localhost:5253',
  WS_URL: (import.meta.env.VITE_API_URL || 'http://localhost:5253').replace('http', 'ws'),
  TIMEOUT: 10000,
};
