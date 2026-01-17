#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"
TESTDATA_DIR="$ROOT_DIR/testdata"

echo "=== Generating Reference Data ==="
echo ""

echo "Checking Python installation..."
if ! command -v python3 &> /dev/null; then
    echo "Error: python3 is not installed. Please install Python 3.10+"
    exit 1
fi
echo "  Python found: $(python3 --version)"

echo ""
echo "Checking Julia installation..."
if ! command -v julia &> /dev/null; then
    echo "Error: julia is not installed. Please install Julia 1.10+"
    exit 1
fi
echo "  Julia found: $(julia --version)"

echo ""
echo "Installing Python dependencies..."
if ! python3 -c "import numpy" 2>/dev/null; then
    echo "  Installing numpy..."
    python3 -m pip install numpy
fi

if ! python3 -c "import numcodecs" 2>/dev/null; then
    echo "  Installing numcodecs..."
    python3 -m pip install numcodecs
fi
echo "  Dependencies installed"

echo ""
echo "Running Python verification..."
python3 "$SCRIPT_DIR/generate_reference.py"

echo ""
echo "Running Julia verification..."
julia "$SCRIPT_DIR/generate_reference.jl"

echo ""
echo "=== Reference Data Generation Complete ==="
echo ""
echo "Generated files:"
ls -la "$TESTDATA_DIR/python/" 2>/dev/null || echo "  Python: (none yet)"
ls -la "$TESTDATA_DIR/julia/" 2>/dev/null || echo "  Julia: (none yet)"
