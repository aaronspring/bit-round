#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"
TESTDATA_DIR="$ROOT_DIR/testdata"

echo "=== Julia Verification ==="

if ! command -v julia &> /dev/null; then
    echo "Error: julia is not installed. Please install Julia."
    exit 1
fi

echo ""
echo "Running Julia verification..."

julia "$SCRIPT_DIR/generate_reference.jl"

echo ""
echo "Julia verification complete. Reference outputs saved to $TESTDATA_DIR/julia/"
