# System Monitor Web Dashboard - Files Manifest

Complete list of all files created for the React web dashboard.

## Configuration Files (9 files)

### Build & Development
1. **package.json** - Dependencies and npm scripts
2. **vite.config.ts** - Vite build tool configuration (dev server, proxy, build settings)
3. **tsconfig.json** - TypeScript compiler configuration
4. **tsconfig.node.json** - TypeScript config for Node.js files (Vite config)

### Styling
5. **tailwind.config.js** - Tailwind CSS configuration (colors, theme)
6. **postcss.config.js** - PostCSS configuration for Tailwind

### Linting
7. **.eslintrc.cjs** - ESLint configuration for code quality
8. **.gitignore** - Git ignore rules for Node.js projects

### Docker & Deployment
9. **Dockerfile** - Multi-stage Docker build (Node builder + Nginx runtime)
10. **docker-compose.yml** - Docker Compose orchestration
11. **nginx.conf** - Nginx configuration for production deployment

## Application Source Files (17 files)

### Entry Points & Root (3 files)
1. **index.html** - HTML entry point and template
2. **src/main.tsx** - React application entry point
3. **src/App.tsx** - Root React component
4. **src/index.css** - Global CSS styles and Tailwind imports

### API Layer (2 files)
5. **src/api/client.ts** - REST API client using Axios
   - getCurrentMetrics()
   - getMetricsHistory()
   - getAnomalies()
   - getAnomaliesBySeverity()
   - getAnomaliesByCategory()
   - healthCheck()

6. **src/api/websocket.ts** - WebSocket client with auto-reconnect
   - Connection management
   - Exponential backoff reconnection
   - Event handlers (onMessage, onConnect, onDisconnect, onError)
   - Message parsing

### React Hooks (2 files)
7. **src/hooks/useWebSocket.ts** - WebSocket connection hook
   - Connection state management
   - Message handling
   - Error handling

8. **src/hooks/useMetrics.ts** - Metrics state management hook
   - Fetch initial data from API
   - Update from WebSocket
   - Anomaly filtering
   - Data refresh

### TypeScript Types (1 file)
9. **src/types/metrics.ts** - Type definitions matching Rust backend
   - SystemMetrics
   - CpuMetrics
   - MemoryMetrics
   - Temperature
   - DiskMetrics
   - UsbDevice
   - NetworkMetrics
   - Anomaly
   - AnomalySeverity enum
   - AnomalyCategory enum

### Utility Functions (1 file)
10. **src/utils/formatters.ts** - Data formatting helpers
    - formatBytes() - Human-readable byte sizes
    - formatPercent() - Percentage formatting
    - formatTemperature() - Temperature display
    - formatNetworkSpeed() - Network speed formatting
    - getUsageColor() - Color based on usage percentage
    - getUsageBackgroundColor() - Background color classes
    - getTemperatureColor() - Color based on temperature
    - formatUptime() - Uptime duration formatting
    - truncate() - String truncation
    - getSeverityBadgeColor() - Anomaly severity colors
    - formatRelativeTime() - Relative time display

### React Components (7 files)

11. **src/components/Dashboard.tsx** - Main dashboard container
    - Layout orchestration
    - Header with connection status
    - Refresh functionality
    - Component composition

12. **src/components/CpuChart.tsx** - CPU usage visualization
    - Line chart for CPU usage over time
    - Load average display
    - Per-core usage bars
    - Recharts integration

13. **src/components/MemoryChart.tsx** - Memory usage visualization
    - Area chart for memory usage
    - RAM and Swap breakdown
    - Usage percentage display
    - Visual progress bar

14. **src/components/TemperatureGauge.tsx** - Temperature monitoring
    - Multiple sensor display
    - Color-coded temperature indicators
    - Average and maximum temperature
    - Visual gauges

15. **src/components/DiskUsage.tsx** - Disk space monitoring
    - Multiple disk support
    - Usage bars
    - I/O statistics (read/write MB)
    - Mount point information

16. **src/components/AnomalyList.tsx** - Anomaly display and filtering
    - Anomaly list with search
    - Severity filters (Info, Warning, Critical)
    - Category filters (CPU, Memory, Temperature, etc.)
    - Relative timestamps
    - Metrics details view

17. **src/components/SystemInfo.tsx** - System information card
    - CPU information (cores, load averages)
    - Memory information (total, used, available, swap)
    - Network information (RX/TX bytes and packets)
    - USB device count

## Documentation Files (4 files)

1. **README.md** - Main project documentation
   - Features overview
   - Tech stack
   - Installation instructions
   - Project structure
   - Configuration guide
   - Development guide
   - Troubleshooting

2. **INSTALLATION.md** - Detailed installation guide
   - Prerequisites
   - Step-by-step installation
   - Configuration options
   - Production deployment
   - Docker deployment
   - Troubleshooting
   - Performance optimization

3. **PROJECT_SUMMARY.md** - Comprehensive project overview
   - Architecture overview
   - Technology decisions
   - Component architecture
   - Data flow diagrams
   - API integration
   - Security considerations
   - Future enhancements

4. **QUICK_START.md** - Quick reference guide
   - 5-minute setup
   - Common commands
   - Quick troubleshooting
   - Key configuration points

5. **FILES_MANIFEST.md** - This file
   - Complete file listing
   - File descriptions
   - Organization overview

## Scripts (1 file)

1. **run-dev.sh** - Development startup script
   - Checks for node_modules
   - Installs dependencies if needed
   - Starts development server
   - Displays helpful URLs

## File Count Summary

- **Configuration Files**: 11
- **Source Code Files**: 17
  - Entry points: 4
  - API layer: 2
  - Hooks: 2
  - Types: 1
  - Utils: 1
  - Components: 7
- **Documentation Files**: 5
- **Scripts**: 1

**Total: 34 files**

## File Dependencies

### External Dependencies (package.json)
**Production:**
- react (18.2.0)
- react-dom (18.2.0)
- axios (1.6.5)
- recharts (2.10.4)
- date-fns (3.2.0)
- react-router-dom (6.21.3)

**Development:**
- typescript (5.3.3)
- vite (5.0.12)
- @vitejs/plugin-react (4.2.1)
- tailwindcss (3.4.1)
- autoprefixer (10.4.17)
- postcss (8.4.33)
- eslint (8.56.0)
- Various @types/* packages

### Internal Dependencies

#### Component Dependencies
```
App.tsx
└── Dashboard.tsx
    ├── useMetrics hook
    │   ├── useWebSocket hook
    │   └── api/client.ts
    ├── CpuChart.tsx
    ├── MemoryChart.tsx
    ├── TemperatureGauge.tsx
    ├── DiskUsage.tsx
    ├── AnomalyList.tsx
    └── SystemInfo.tsx
```

#### Type Dependencies
```
All components import from:
└── types/metrics.ts

All components use:
└── utils/formatters.ts
```

## Build Output

After running `npm run build`, the following is generated:

```
dist/
├── index.html              # Production HTML
├── assets/
│   ├── index-[hash].js    # Main bundle (minified)
│   ├── vendor-[hash].js   # Vendor chunks (React, etc.)
│   ├── index-[hash].css   # Compiled CSS (Tailwind)
│   └── [other assets]     # Images, fonts, etc.
└── [other files]
```

## Key Design Decisions

### Why Vite?
- Faster than Webpack
- Better dev experience (HMR)
- Native ES modules
- Optimized builds

### Why Tailwind CSS?
- Rapid development
- Consistent design system
- Small bundle size (unused classes removed)
- Responsive utilities

### Why Recharts?
- React-friendly API
- Composable components
- Good performance
- Active maintenance

### Why Axios?
- Better API than fetch
- Interceptors
- Request/response transformation
- Browser and Node.js support

### Why No State Management Library?
- Application is simple enough for hooks
- Avoids unnecessary complexity
- Easier to understand
- Smaller bundle size
- Hooks provide sufficient state management

### Why TypeScript?
- Type safety
- Better IDE support
- Catches errors early
- Self-documenting code
- Matches Rust's type safety

## File Organization Principles

1. **Separation of Concerns**
   - API logic in `/api`
   - UI components in `/components`
   - Business logic in `/hooks`
   - Utilities in `/utils`
   - Types in `/types`

2. **Naming Conventions**
   - Components: PascalCase (Dashboard.tsx)
   - Hooks: camelCase with 'use' prefix (useMetrics.ts)
   - Utils: camelCase (formatters.ts)
   - Types: camelCase (metrics.ts)
   - Config: kebab-case or standard (vite.config.ts)

3. **Import Organization**
   - React imports first
   - Third-party imports second
   - Local imports third
   - Type imports last

4. **File Size Guidelines**
   - Components: < 300 lines
   - Hooks: < 150 lines
   - Utils: < 200 lines
   - Extract to separate files when limits reached

## Maintenance Notes

### Regular Updates
- Security patches: As released
- Dependencies: Monthly review
- TypeScript: Quarterly major version check
- React: When stable versions released

### Adding New Files
When adding new files, update:
1. This manifest
2. README.md (if user-facing)
3. tsconfig.json (if new path aliases)
4. .gitignore (if build artifacts)

### Removing Files
When removing files, check:
1. Import statements in other files
2. Documentation references
3. Build configuration
4. Deployment scripts

## Version History

### v1.0.0 - Initial Release (2026-01-29)
- Complete React dashboard
- Real-time WebSocket updates
- Comprehensive metrics display
- Anomaly detection and filtering
- Responsive design
- Docker support
- Full documentation

---

**Last Updated**: 2026-01-29
**Maintained By**: System Monitor Project Team
**File Count**: 34 files
