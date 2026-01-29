# Quick Start Guide - System Monitor Web Dashboard

Get up and running in 5 minutes!

## Prerequisites

- Node.js 18+ installed
- System Monitor backend running on port 8080

## Installation

### 1. Navigate to the directory
```bash
cd /home/marvinbraga/dados/system-monitor/web-frontend
```

### 2. Install dependencies
```bash
npm install
```

### 3. Start development server
```bash
npm run dev
```

Or use the convenience script:
```bash
./run-dev.sh
```

### 4. Open in browser
```
http://localhost:3000
```

That's it! The dashboard should now be displaying real-time system metrics.

## Common Commands

### Development
```bash
npm run dev          # Start dev server with HMR
npm run build        # Build for production
npm run preview      # Preview production build
npm run lint         # Run ESLint
```

### Quick Scripts
```bash
./run-dev.sh         # Install deps + start dev server
```

## Troubleshooting

### "Cannot connect to backend"
**Fix**: Ensure backend is running on port 8080
```bash
curl http://localhost:8080/api/health
```

### "Port 3000 already in use"
**Fix**: Change port in `vite.config.ts`:
```typescript
server: { port: 3001 }
```

### "WebSocket disconnected"
**Fix**: Check backend WebSocket endpoint:
```bash
# Backend should have /ws endpoint available
```

### "npm install fails"
**Fix**: Clear cache and retry:
```bash
rm -rf node_modules package-lock.json
npm install
```

## Project Structure

```
src/
├── api/           # API client & WebSocket
├── components/    # React components
├── hooks/         # Custom hooks
├── types/         # TypeScript types
└── utils/         # Helper functions
```

## Key Files

- **src/components/Dashboard.tsx** - Main dashboard
- **src/hooks/useMetrics.ts** - Metrics state management
- **src/api/websocket.ts** - WebSocket client
- **vite.config.ts** - Dev server & proxy config

## Features at a Glance

- ✅ Real-time CPU, Memory, Temperature monitoring
- ✅ Disk usage and I/O statistics
- ✅ Network traffic monitoring
- ✅ USB device detection
- ✅ Anomaly alerts with filtering
- ✅ WebSocket auto-reconnect
- ✅ Responsive design

## Configuration

### Backend URL
Edit `vite.config.ts`:
```typescript
proxy: {
  '/api': {
    target: 'http://localhost:8080',  // Change here
  }
}
```

### WebSocket URL
Edit `src/api/websocket.ts`:
```typescript
constructor(url: string = 'ws://localhost:8080/ws')  // Change here
```

## Production Deployment

### Build
```bash
npm run build
```

### Docker
```bash
docker build -t system-monitor-web .
docker run -p 3000:80 system-monitor-web
```

### Docker Compose
```bash
docker-compose up -d
```

## Getting Help

1. Check [README.md](README.md) for detailed docs
2. Check [INSTALLATION.md](INSTALLATION.md) for setup guide
3. Check [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for architecture
4. Review browser console for errors
5. Check backend logs

## Next Steps

- Customize colors in `tailwind.config.js`
- Add new metrics components
- Configure alert thresholds
- Set up production deployment

## Useful Links

- Frontend: http://localhost:3000
- Backend API: http://localhost:8080/api
- WebSocket: ws://localhost:8080/ws

Happy monitoring!
