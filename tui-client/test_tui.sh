#!/bin/bash
# Test script for TUI client

echo "System Monitor TUI Client Test"
echo "================================"
echo ""

# Check if binary exists
if [ ! -f "../target/debug/tui-client" ]; then
    echo "Building TUI client..."
    cargo build --package tui-client
    if [ $? -ne 0 ]; then
        echo "ERROR: Build failed!"
        exit 1
    fi
fi

echo "TUI client binary found at: ../target/debug/tui-client"
echo ""

# Display help
echo "=== Help Output ==="
../target/debug/tui-client --help
echo ""

# Test argument parsing
echo "=== Testing Configuration Parsing ==="
echo "Testing with custom refresh rate and URL..."
echo "(This will fail to connect since no server is running, but tests arg parsing)"
echo ""

# Run with a short timeout to test startup
timeout 2s ../target/debug/tui-client --api-url http://localhost:8080 --refresh 1 2>&1 || true

echo ""
echo "=== Test Complete ==="
echo ""
echo "To run the TUI client:"
echo "  1. Start the system-monitor server:"
echo "     cargo run --package collector"
echo ""
echo "  2. In another terminal, run the TUI client:"
echo "     cargo run --package tui-client"
echo ""
echo "     Or with custom options:"
echo "     cargo run --package tui-client -- --refresh 5"
echo ""
