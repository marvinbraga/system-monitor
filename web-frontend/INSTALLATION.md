# Installation Guide - System Monitor Web Dashboard

This guide provides detailed instructions for setting up and running the System Monitor Web Dashboard.

## Prerequisites

### Required Software

1. **Node.js** (v18.0.0 or higher)
   ```bash
   # Check version
   node --version

   # Install on Ubuntu/Debian
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   ```

2. **npm** (usually comes with Node.js)
   ```bash
   # Check version
   npm --version
   ```

3. **System Monitor Backend**
   - The backend must be running on `http://localhost:8080`
   - Refer to the main project README for backend setup

## Quick Start

### 1. Navigate to the web-frontend directory

```bash
cd /home/marvinbraga/dados/system-monitor/web-frontend
```

### 2. Install dependencies

```bash
npm install
```

This will install:
- React 18.2.0
- TypeScript 5.3.3
- Vite 5.0.12
- Recharts 2.10.4
- Axios 1.6.5
- Tailwind CSS 3.4.1
- date-fns 3.2.0
- And all required development dependencies

### 3. Start the development server

```bash
npm run dev
```

Or use the provided script:

```bash
./run-dev.sh
```

The dashboard will be available at: **http://localhost:3000**

## Available Scripts

### Development

```bash
npm run dev
```
Starts the Vite development server with hot module replacement on port 3000.

### Build for Production

```bash
npm run build
```
Creates an optimized production build in the `dist` directory.

### Preview Production Build

```bash
npm run preview
```
Serves the production build locally for testing.

### Linting

```bash
npm run lint
```
Runs ESLint to check code quality.

### Type Checking

```bash
npx tsc --noEmit
```
Checks TypeScript types without emitting files.

## Configuration

### Backend Connection

The frontend connects to the backend via proxy configuration in `vite.config.ts`:

```typescript
server: {
  port: 3000,
  proxy: {
    '/api': {
      target: 'http://localhost:8080',
      changeOrigin: true,
    },
    '/ws': {
      target: 'ws://localhost:8080',
      ws: true,
    },
  },
}
```

#### Changing Backend URL

If your backend runs on a different host/port:

1. **For Development**: Edit `vite.config.ts`
   ```typescript
   target: 'http://your-backend-host:port',
   ```

2. **For Production**: The backend URL is determined by the deployment environment. The production build uses relative URLs (`/api` and `/ws`), so ensure the frontend and backend are served from the same domain or configure CORS appropriately.

### WebSocket Configuration

Edit `src/api/websocket.ts` to change WebSocket behavior:

```typescript
private maxReconnectAttempts: number = 10;
private reconnectDelay: number = 1000; // Start with 1 second
private maxReconnectDelay: number = 30000; // Max 30 seconds
```

## Production Deployment

### Building for Production

1. Build the project:
   ```bash
   npm run build
   ```

2. The `dist` directory will contain:
   - Optimized and minified JavaScript bundles
   - CSS files
   - HTML entry point
   - Assets

### Deployment Options

#### Option 1: Serve with Nginx

1. Install Nginx:
   ```bash
   sudo apt-get install nginx
   ```

2. Copy built files:
   ```bash
   sudo cp -r dist/* /var/www/html/
   ```

3. Configure Nginx (`/etc/nginx/sites-available/system-monitor`):
   ```nginx
   server {
       listen 80;
       server_name your-domain.com;
       root /var/www/html;
       index index.html;

       location / {
           try_files $uri $uri/ /index.html;
       }

       # Proxy API requests
       location /api {
           proxy_pass http://localhost:8080;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }

       # Proxy WebSocket
       location /ws {
           proxy_pass http://localhost:8080;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection "Upgrade";
           proxy_set_header Host $host;
       }
   }
   ```

4. Enable and restart:
   ```bash
   sudo ln -s /etc/nginx/sites-available/system-monitor /etc/nginx/sites-enabled/
   sudo nginx -t
   sudo systemctl restart nginx
   ```

#### Option 2: Serve with the Backend

The backend can serve the static files. Build the frontend and configure the backend to serve from the `dist` directory.

#### Option 3: Deploy to Vercel/Netlify

For cloud deployment, push the repository and connect it to Vercel or Netlify. Configure environment variables for the backend API URL.

## Docker Deployment

A Dockerfile is provided for containerized deployment:

```bash
# Build image
docker build -t system-monitor-web .

# Run container
docker run -p 3000:3000 system-monitor-web
```

## Troubleshooting

### Port 3000 Already in Use

Change the port in `vite.config.ts`:
```typescript
server: {
  port: 3001, // Your preferred port
  // ...
}
```

### Cannot Connect to Backend

**Symptoms**: Dashboard loads but shows "Error loading metrics" or connection errors.

**Solutions**:
1. Verify backend is running:
   ```bash
   curl http://localhost:8080/api/health
   ```

2. Check browser console for CORS errors

3. Verify proxy configuration in `vite.config.ts`

4. For production, ensure backend allows CORS from frontend domain

### WebSocket Connection Fails

**Symptoms**: "Disconnected" status indicator, no real-time updates.

**Solutions**:
1. Check WebSocket URL in browser console

2. Verify backend WebSocket endpoint: `ws://localhost:8080/ws`

3. Check firewall rules allow WebSocket connections

4. For production behind a proxy, ensure WebSocket upgrade headers are preserved

### Build Errors

**Symptoms**: `npm run build` fails with type errors or module not found.

**Solutions**:
1. Clear cache and reinstall:
   ```bash
   rm -rf node_modules package-lock.json dist
   npm install
   ```

2. Update dependencies:
   ```bash
   npm update
   ```

3. Check Node.js version (must be 18+)

### Slow Performance

**Solutions**:
1. Reduce history limit in `useMetrics` hook (default: 60 records)
   ```typescript
   apiClient.getMetricsHistory(30) // Reduce to 30
   ```

2. Increase update interval in backend

3. Enable production build optimizations

### Charts Not Displaying

**Solutions**:
1. Check browser console for Recharts errors

2. Verify data format matches TypeScript types

3. Ensure sufficient metrics history is available

## Development Tips

### Hot Module Replacement

Vite provides instant HMR. Changes to React components are reflected immediately without page reload.

### Development Tools

1. **React Developer Tools**: Install browser extension for debugging
2. **Redux DevTools**: Not needed (we use hooks and local state)
3. **Vite DevTools**: Built into Vite

### Code Style

The project uses:
- ESLint for code quality
- Prettier for formatting (optional)
- TypeScript strict mode

### Adding New Components

1. Create component in `src/components/`
2. Import types from `src/types/metrics`
3. Use hooks from `src/hooks/`
4. Follow existing patterns for consistency

## Environment Variables

Create a `.env.local` file for local overrides:

```env
# Backend API URL (for production builds)
VITE_API_URL=http://your-backend.com

# WebSocket URL
VITE_WS_URL=ws://your-backend.com/ws
```

Access in code:
```typescript
const apiUrl = import.meta.env.VITE_API_URL || '/api';
```

## Performance Optimization

### Production Build Optimization

The build process automatically:
- Minifies JavaScript and CSS
- Tree-shakes unused code
- Splits code into chunks
- Optimizes images and assets
- Generates source maps

### Runtime Optimization

- WebSocket connection reuses single connection
- Charts use memoization to prevent unnecessary re-renders
- Virtualization for large lists (if needed)

## Security Considerations

1. **CORS**: Configure backend to allow requests from frontend domain
2. **Authentication**: Implement if needed (backend responsibility)
3. **HTTPS**: Use HTTPS in production
4. **Content Security Policy**: Configure CSP headers

## Support

For issues or questions:
1. Check this documentation
2. Review browser console errors
3. Check backend logs
4. Review WebSocket connection status

## Next Steps

After installation:
1. Start the backend service
2. Start the frontend development server
3. Open http://localhost:3000 in your browser
4. Monitor real-time system metrics!

## Additional Resources

- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
- [Tailwind CSS Documentation](https://tailwindcss.com/)
- [Recharts Documentation](https://recharts.org/)
