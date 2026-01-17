#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"
TESTDATA_DIR="$ROOT_DIR/testdata"

echo "=== Generating Reference Data via Docker ==="
echo ""

echo "Building Python verification image..."
docker build -t bitround-python-verification -f "$ROOT_DIR/docker/Dockerfile.python" "$ROOT_DIR/docker"

echo ""
echo "Building Julia verification image..."
docker build -t bitround-julia-verification -f "$ROOT_DIR/docker/Dockerfile.julia" "$ROOT_DIR/docker"

echo ""
echo "Running Python verification..."
docker run --rm \
    -v "$TESTDATA_DIR/inputs:/data/inputs" \
    -v "$TESTDATA_DIR/python:/data/outputs" \
    bitround-python-verification

echo ""
echo "Running Julia verification..."
docker run --rm \
    -v "$TESTDATA_DIR/inputs:/data/inputs" \
    -v "$TESTDATA_DIR/julia:/data/outputs" \
    bitround-julia-verification

echo ""
echo "=== Reference Data Generation Complete ==="
echo ""
echo "Generated files:"
ls -la "$TESTDATA_DIR/python/" 2>/dev/null || echo "  Python: (none yet)"
ls -la "$TESTDATA_DIR/julia/" 2>/dev/null || echo "  Julia: (none yet)"
