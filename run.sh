#!/bin/bash

# Face Detection Rust - Run Script
# This script helps run the application with different configurations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if port is available
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 1
    else
        return 0
    fi
}

# Function to find available port
find_available_port() {
    local port=$1
    while ! check_port $port; do
        port=$((port + 1))
        if [ $port -gt 9000 ]; then
            print_error "No available ports found between 8080-9000"
            exit 1
        fi
    done
    echo $port
}

# Default values
PORT=${PORT:-8080}
LOG_LEVEL=${RUST_LOG:-info}
MODE=${MODE:-development}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--port)
            PORT="$2"
            shift 2
            ;;
        -l|--log-level)
            LOG_LEVEL="$2"
            shift 2
            ;;
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -d|--debug)
            LOG_LEVEL="debug"
            MODE="debug"
            shift
            ;;
        -r|--release)
            MODE="release"
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo
            echo "Options:"
            echo "  -p, --port PORT       Set server port (default: 8080)"
            echo "  -l, --log-level LEVEL Set log level (default: info)"
            echo "  -m, --mode MODE       Set run mode (development/debug/release)"
            echo "  -d, --debug          Enable debug mode (log level: debug)"
            echo "  -r, --release        Run in release mode"
            echo "  -h, --help           Show this help message"
            echo
            echo "Examples:"
            echo "  $0                    # Run with default settings"
            echo "  $0 -p 3000            # Run on port 3000"
            echo "  $0 -d                 # Run in debug mode"
            echo "  $0 -r                 # Run in release mode"
            echo "  $0 -p 8081 -l debug   # Custom port and log level"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

echo "=========================================="
echo "  Face Detection Rust - Run Script"
echo "=========================================="
echo

# Check if project is built
if [ "$MODE" = "release" ]; then
    if [ ! -f "target/release/face-detect-rust" ]; then
        print_status "Building project in release mode..."
        cargo build --release
    fi
    BINARY="target/release/face-detect-rust"
else
    BINARY="cargo run --"
fi

# Find available port if needed
if ! check_port $PORT; then
    print_warning "Port $PORT is already in use"
    NEW_PORT=$(find_available_port $PORT)
    print_status "Using port $NEW_PORT instead"
    PORT=$NEW_PORT
fi

# Set environment variables
export PORT=$PORT
export RUST_LOG=$LOG_LEVEL

print_status "Starting Face Detection Rust server..."
print_status "Port: $PORT"
print_status "Log Level: $LOG_LEVEL"
print_status "Mode: $MODE"
echo

# Create uploads directory if it doesn't exist
mkdir -p uploads

# Run the application
if [ "$MODE" = "release" ]; then
    exec $BINARY
else
    exec cargo run
fi