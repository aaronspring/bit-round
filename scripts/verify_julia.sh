#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"
DOCKER_DIR="$ROOT_DIR/docker"
TESTDATA_DIR="$ROOT_DIR/testdata"

echo "=== Julia Verification via Docker ==="
echo "Building Julia verification image..."

docker build -t bitround-julia-verification -f "$DOCKER_DIR/Dockerfile.julia" "$DOCKER_DIR"

echo ""
echo "Running Julia verification..."

docker run --rm \
    -v "$TESTDATA_DIR/inputs:/data/inputs" \
    -v "$TESTDATA_DIR/julia:/data/outputs" \
    bitround-julia-verification

echo ""
echo "Julia verification complete. Reference outputs saved to $TESTDATA_DIR/julia/"
