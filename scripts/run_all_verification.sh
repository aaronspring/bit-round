#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$(dirname "$SCRIPT_DIR")" && pwd)"

echo "=== Running All Verifications ==="
echo ""

"$ROOT_DIR/scripts/verify_python.sh"
echo ""
"$ROOT_DIR/scripts/verify_julia.sh"
echo ""
echo "=== All Verifications Complete ==="
echo ""
echo "Reference data generated:"
echo "  - Python: $ROOT_DIR/testdata/python/"
echo "  - Julia: $ROOT_DIR/testdata/julia/"
