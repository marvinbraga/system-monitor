# System Monitor Web Dashboard

A modern, real-time web dashboard for monitoring system metrics built with React, TypeScript, and Vite.

## Features

- **Real-time Updates**: WebSocket connection for live metric updates
- **Comprehensive Metrics**: CPU, Memory, Temperature, Disk, Network, and USB device monitoring
- **Anomaly Detection**: Real-time anomaly alerts with filtering
- **Interactive Charts**: Beautiful visualizations using Recharts
- **Responsive Design**: Works seamlessly on desktop and mobile devices
- **Modern UI**: Built with Tailwind CSS

## Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **Tailwind CSS** - Styling
- **Recharts** - Data visualization
- **Axios** - HTTP client
- **date-fns** - Date formatting

## Prerequisites

- Node.js 18 or higher
- npm or yarn
- System Monitor backend running on `http://localhost:8080`

## Installation

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm run dev
```

The dashboard will be available at `http://localhost:3000`

## Build for Production

```bash
npm run build
```

The built files will be in the `dist` directory.

## Preview Production Build

```bash
npm run preview
```

## Project Structure

```
web-frontend/
├── src/
│   ├── api/              # API client and WebSocket
│   │   ├── client.ts     # REST API client
│   │   └── websocket.ts  # WebSocket client with auto-reconnect
│   ├── components/       # React components
│   │   ├── Dashboard.tsx         # Main dashboard layout
│   │   ├── CpuChart.tsx         # CPU usage chart
│   │   ├── MemoryChart.tsx      # Memory usage chart
│   │   ├── TemperatureGauge.tsx # Temperature display
│   │   ├── DiskUsage.tsx        # Disk usage bars
│   │   ├── AnomalyList.tsx      # Anomaly list with filters
│   │   └── SystemInfo.tsx       # System information card
│   ├── hooks/            # React hooks
│   │   ├── useWebSocket.ts # WebSocket hook
│   │   └── useMetrics.ts   # Metrics state management
│   ├── types/            # TypeScript types
│   │   └── metrics.ts    # Type definitions matching Rust types
│   ├── utils/            # Utility functions
│   │   └── formatters.ts # Data formatting helpers
│   ├── App.tsx           # Root component
│   ├── main.tsx          # Entry point
│   └── index.css         # Global styles
├── index.html            # HTML template
├── package.json          # Dependencies
├── tsconfig.json         # TypeScript config
├── vite.config.ts        # Vite config
└── tailwind.config.js    # Tailwind config
```

## Configuration

### API Endpoint

The API endpoint is configured in `vite.config.ts`:

```typescript
proxy: {
  '/api': {
    target: 'http://localhost:8080',
    changeOrigin: true,
  },
  '/ws': {
    target: 'ws://localhost:8080',
    ws: true,
  },
}
```

### WebSocket URL

The WebSocket URL can be changed in `src/api/websocket.ts`:

```typescript
constructor(url: string = 'ws://localhost:8080/ws')
```

## Features Overview

### Real-time Monitoring

The dashboard uses WebSocket for real-time updates:
- CPU usage and per-core metrics
- Memory usage (RAM and Swap)
- Temperature sensors
- Disk usage and I/O
- Network traffic
- USB device detection

### Anomaly Detection

View and filter system anomalies by:
- Severity (Info, Warning, Critical)
- Category (CPU, Memory, Temperature, etc.)
- Search text

### Charts

- **CPU Chart**: Line chart showing global usage and load average
- **Memory Chart**: Area chart with RAM and Swap usage
- **Temperature Gauges**: Visual indicators for all temperature sensors
- **Disk Usage**: Progress bars with I/O statistics

## Customization

### Color Themes

Modify the color palette in `tailwind.config.js`:

```javascript
theme: {
  extend: {
    colors: {
      primary: {
        // Your color shades
      },
    },
  },
}
```

### Thresholds

Adjust warning and danger thresholds in `src/utils/formatters.ts`:

```typescript
export function getUsageColor(
  percent: number,
  thresholds = { warning: 70, danger: 90 }
): string {
  // ...
}
```

## Development

### Hot Module Replacement

Vite provides instant HMR during development. Changes to components will be reflected immediately without full page reload.

### Type Checking

Run TypeScript type checking:

```bash
npx tsc --noEmit
```

### Linting

Run ESLint:

```bash
npm run lint
```

## Troubleshooting

### WebSocket Connection Issues

If the WebSocket fails to connect:
1. Ensure the backend is running on port 8080
2. Check browser console for connection errors
3. Verify CORS settings in the backend

### Build Errors

If you encounter build errors:
1. Clear node_modules: `rm -rf node_modules`
2. Clear cache: `rm -rf dist`
3. Reinstall: `npm install`
4. Rebuild: `npm run build`

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## License

Part of the System Monitor project.
