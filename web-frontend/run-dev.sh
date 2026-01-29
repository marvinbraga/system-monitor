#!/bin/bash

# Development startup script for System Monitor Web Dashboard

set -e

echo "==================================="
echo "System Monitor Web Dashboard"
echo "==================================="
echo ""

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
    echo ""
fi

# Start development server
echo "Starting development server..."
echo "Dashboard will be available at: http://localhost:3000"
echo ""
echo "Make sure the backend is running on: http://localhost:8080"
echo ""

npm run dev
