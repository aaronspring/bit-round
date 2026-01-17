#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"
DOCKER_DIR="$ROOT_DIR/docker"
TESTDATA_DIR="$ROOT_DIR/testdata"

echo "=== Python Verification via Docker ==="
echo "Building Python verification image..."

docker build -t bitround-python-verification -f "$DOCKER_DIR/Dockerfile.python" "$DOCKER_DIR"

echo ""
echo "Running Python verification..."

docker run --rm \
    -v "$TESTDATA_DIR/inputs:/data/inputs" \
    -v "$TESTDATA_DIR/python:/data/outputs" \
    bitround-python-verification

echo ""
echo "Python verification complete. Reference outputs saved to $TESTDATA_DIR/python/"
