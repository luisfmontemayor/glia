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

echo "--------------------------------------------------------"
echo " [Glia] R Installation Test"
echo "--------------------------------------------------------"
echo " [1/2] Building Docker image 'glia-rtest'..."
echo "       Context: $REPO_ROOT"
echo "       File:    test/installations/Dockerfile.Rtest"
echo "--------------------------------------------------------"

# Note: Use -f to point to the Dockerfile but the build context is project root (.)
docker build -t glia-rtest -f test/installations/Dockerfile.Rtest .

echo "--------------------------------------------------------"
echo " [2/2] Running R installation test container..."
echo "--------------------------------------------------------"
docker run --rm glia-rtest

echo "--------------------------------------------------------"
echo " [Glia] Test completed successfully."
echo "--------------------------------------------------------"
