import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5252,
    proxy: {
      '/api/v1': {
        target: 'http://localhost:5253',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://localhost:5253',
        ws: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
  },
});
