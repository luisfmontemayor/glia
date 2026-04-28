#!/bin/bash
# Script to build and run the R installation test Docker image

# Exit on error
set -e

# Always run from the root of the repository
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$(cd "$DIR/../.." && pwd)"
cd "$REPO_ROOT"

# Check if docker is installed
if ! command -v docker &> /dev/null; then
    echo "Error: docker is not installed or not in PATH."
    exit 1
fi

# Create a temporary file for logs to avoid context bloat during build
LOG_FILE=$(mktemp /tmp/glia-rtest-build-XXXXXX.log)
BUILD_CONTEXT=$(mktemp -d /tmp/glia-rtest-context-XXXXXX)

echo "--------------------------------------------------------"
echo " [Glia] R Installation Test"
echo " Build Logs: $LOG_FILE"
echo " Build Context: $BUILD_CONTEXT"
echo "--------------------------------------------------------"
echo " [1/2] Building Docker image 'glia-rtest'..."
echo "       Context: $BUILD_CONTEXT (Cleaned clone of $REPO_ROOT)"
echo "       File:    $DIR/Dockerfile.Rtest"
echo "--------------------------------------------------------"

# Sync the repo to the temporary build context, excluding local build artifacts
git archive HEAD | tar -x -C "$BUILD_CONTEXT/"

# Redirect build output to log file to avoid context bloat
# We use a background process to keep the CLI alive with progress updates
(
    while true; do
        sleep 30
        echo "Building... (check $LOG_FILE for details)"
    done
) &
KEEPALIVE_PID=$!

# Run docker build and capture exit code
if ! docker build -t glia-rtest -f "$DIR/Dockerfile.Rtest" "$BUILD_CONTEXT" > "$LOG_FILE" 2>&1; then
    kill $KEEPALIVE_PID
    echo "Error: Docker build failed. Check logs at: $LOG_FILE"
    tail -n 20 "$LOG_FILE"
    rm -rf "$BUILD_CONTEXT"
    exit 1
fi

# Clean up build context
rm -rf "$BUILD_CONTEXT"

# Clean up keep-alive process
kill $KEEPALIVE_PID
echo "Build finished successfully."

echo "--------------------------------------------------------"
echo " [2/2] Running R installation test container..."
echo "--------------------------------------------------------"
# Keep docker run output in stdout for verification
docker run --rm glia-rtest

echo "--------------------------------------------------------"
echo " [Glia] Test completed successfully."
echo " Build logs available at: $LOG_FILE"
echo "--------------------------------------------------------"
