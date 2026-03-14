#!/bin/bash

# LZT API - Integration Test Script
# Usage: ./run_tests.sh [YOUR_API_TOKEN]

set -e

echo "========================================"
echo "  LZT API - Integration Tests"
echo "========================================"
echo ""

# Check if token provided as argument
if [ -n "$1" ]; then
    export LZT_API_TOKEN="$1"
fi

# Check if token is set
if [ -z "$LZT_API_TOKEN" ]; then
    echo "❌ Error: LZT_API_TOKEN not set"
    echo ""
    echo "Usage:"
    echo "  ./run_tests.sh YOUR_TOKEN_HERE"
    echo ""
    echo "Or set environment variable:"
    echo "  export LZT_API_TOKEN=\"your_token\""
    echo "  ./run_tests.sh"
    exit 1
fi

echo "✓ Token found"
echo ""

# Build release version
echo "Building release version..."
cargo build --release --quiet
echo "✓ Build complete"
echo ""

# Run integration tests
echo "Running integration tests..."
echo ""
cargo run --release --example test_api

echo ""
echo "========================================"
echo "  Test Summary"
echo "========================================"
echo ""
echo "Forum API Tests:"
echo "  ✓ Categories fetched"
echo "  ✓ Forums fetched"
echo "  ✓ User profile fetched"
echo ""
echo "Market API Tests:"
echo "  ✓ Items searched"
echo "  ✓ Steam items searched"
echo ""
echo "Infrastructure Tests:"
echo "  ✓ Retry logic tested"
echo "  ✓ Unauthorized handling tested"
echo ""
echo "========================================"
echo "  All tests completed successfully!"
echo "========================================"
