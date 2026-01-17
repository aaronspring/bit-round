#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"
DOCKER_DIR="$ROOT_DIR/docker"
TESTDATA_DIR="$ROOT_DIR/testdata"

echo "=== Python Verification ==="

if ! command -v python3 &> /dev/null; then
    echo "Error: python3 is not installed. Please install Python 3."
    exit 1
fi

if ! python3 -c "import numpy" 2>/dev/null; then
    echo "Installing numpy..."
    python3 -m pip install numpy
fi

if ! python3 -c "import numcodecs" 2>/dev/null; then
    echo "Installing numcodecs..."
    python3 -m pip install numcodecs
fi

echo ""
echo "Running Python verification..."

python3 "$DOCKER_DIR/generate_reference.py"

echo ""
echo "Python verification complete. Reference outputs saved to $TESTDATA_DIR/python/"
