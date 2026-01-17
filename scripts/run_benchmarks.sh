#!/bin/bash
#
# Benchmark runner for bitround implementations
# Runs Python, Julia, and Rust benchmarks and generates a combined report
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

NBITS=16
N_WARMUP=3
N_ITERATIONS=10
OUTPUT_FILE=""
EDGE_SIZES="1,10,100"

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --nbits N         Number of bits to keep (default: 16)"
    echo "  --warmup N        Number of warmup iterations (default: 3)"
    echo "  --iterations N    Number of measured iterations (default: 10)"
    echo "  --output FILE     Output file for results (default: stdout)"
    echo "  --sizes SIZES     Comma-separated edge sizes (default: 1,10,100)"
    echo "  --python-only     Run only Python benchmark"
    echo "  --julia-only      Run only Julia benchmark"
    echo "  --rust-only       Run only Rust benchmark"
    echo "  --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                        # Run all benchmarks"
    echo "  $0 --nbits 11 --iterations 20"
    echo "  $0 --sizes 0,1,2          # Edge sizes 1x1x1, 10x10x10, 100x100x100"
    echo "  $0 --output results.md"
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --nbits)
                NBITS="$2"
                shift 2
                ;;
            --warmup)
                N_WARMUP="$2"
                shift 2
                ;;
            --iterations)
                N_ITERATIONS="$2"
                shift 2
                ;;
            --output)
                OUTPUT_FILE="$2"
                shift 2
                ;;
            --sizes)
                EDGE_SIZES="$2"
                shift 2
                ;;
            --python-only)
                RUN_PYTHON=true
                RUN_JULIA=false
                RUN_RUST=false
                shift
                ;;
            --julia-only)
                RUN_PYTHON=false
                RUN_JULIA=true
                RUN_RUST=false
                shift
                ;;
            --rust-only)
                RUN_PYTHON=false
                RUN_JULIA=false
                RUN_RUST=true
                shift
                ;;
            --help)
                usage
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
}

RUN_PYTHON=true
RUN_JULIA=true
RUN_RUST=true
PYTHON_JSON=""
JULIA_JSON=""
RUST_JSON=""

parse_args "$@"

echo "========================================"
echo "Bitround Benchmark Runner"
echo "========================================"
echo ""
echo "Configuration:"
echo "  nbits: $NBITS"
echo "  warmup iterations: $N_WARMUP"
echo "  measured iterations: $N_ITERATIONS"
echo "  edge sizes: $EDGE_SIZES"
echo ""

run_python() {
    echo "Running Python benchmark..."
    PYTHON_JSON=$(python3 "$SCRIPT_DIR/../bench_python.py" \
        --nbits "$NBITS" \
        --warmup "$N_WARMUP" \
        --iterations "$N_ITERATIONS" \
        --json 2>/dev/null)
    if [ -z "$PYTHON_JSON" ] || ! echo "$PYTHON_JSON" | python3 -c "import json, sys; json.load(sys.stdin)" 2>/dev/null; then
        PYTHON_JSON='{"error": "Python benchmark failed", "python": {}}'
    fi
    echo "Python benchmark complete."
    echo ""
}

run_julia() {
    echo "Running Julia benchmark..."
    JULIA_JSON=$(julia --project="$SCRIPT_DIR/.." \
        "$SCRIPT_DIR/../bench_julia.jl" \
        --nbits "$NBITS" \
        --warmup "$N_WARMUP" \
        --iterations "$N_ITERATIONS" \
        --json 2>/dev/null)
    if [ -z "$JULIA_JSON" ] || ! echo "$JULIA_JSON" | python3 -c "import json, sys; json.load(sys.stdin)" 2>/dev/null; then
        JULIA_JSON='{"error": "Julia benchmark failed", "julia": {}}'
    fi
    echo "Julia benchmark complete."
    echo ""
}

run_rust() {
    echo "Running Rust benchmark..."
    RUST_JSON=$(cd "$SCRIPT_DIR/.." && cargo run --release --bin bench \
        -- \
        --nbits "$NBITS" \
        --warmup "$N_WARMUP" \
        --iterations "$N_ITERATIONS" \
        --json 2>/dev/null)
    if [ -z "$RUST_JSON" ] || ! echo "$RUST_JSON" | python3 -c "import json, sys; json.load(sys.stdin)" 2>/dev/null; then
        RUST_JSON='{"error": "Rust benchmark failed", "rust": {}}'
    fi
    echo "Rust benchmark complete."
    echo ""
}

if $RUN_PYTHON; then
    run_python
fi

if $RUN_JULIA; then
    run_julia
fi

if $RUN_RUST; then
    run_rust
fi

generate_combined_report() {
    local python_json_file=$(mktemp)
    local julia_json_file=$(mktemp)
    local rust_json_file=$(mktemp)
    local tmp_file=$(mktemp)

    echo "$PYTHON_JSON" > "$python_json_file"
    echo "$JULIA_JSON" > "$julia_json_file"
    echo "$RUST_JSON" > "$rust_json_file"

    cat > "$tmp_file" << 'PYTHON_SCRIPT'
import json
import sys

def read_json_file(path):
    try:
        with open(path, 'r') as f:
            return json.load(f)
    except Exception as e:
        return {"error": str(e), "python": {}}

python_data = read_json_file(sys.argv[1])
julia_data = read_json_file(sys.argv[2])
rust_data = read_json_file(sys.argv[3])

def get_machine_specs(data):
    if "machine_specs" in data:
        return data["machine_specs"]
    return {
        "computer_family": "Unknown",
        "cpu_model": "Unknown",
        "cpu_cores": "Unknown",
        "ram_gb": "Unknown",
        "os": "Unknown"
    }

def get_results(data, key):
    if isinstance(data, dict) and key in data:
        return data[key]
    return {}

python_specs = get_machine_specs(python_data)
julia_specs = get_machine_specs(julia_data)
rust_specs = get_machine_specs(rust_data)

python_results = get_results(python_data, "python")
julia_results = get_results(julia_data, "julia")
rust_results = get_results(rust_data, "rust")

all_sizes = set()
for results in [python_results, julia_results, rust_results]:
    if isinstance(results, dict):
        all_sizes.update(results.keys())

sizes = sorted(all_sizes, key=lambda s: [int(x) for x in s.split('x')])

print("# Bitround Benchmark Results")
print("")
print("## Machine Specifications")
print("")
print("| Implementation | Computer | CPU | Cores | RAM | OS |")
print("|----------------|----------|-----|-------|-----|-----|")
print(f"| Python | {python_specs.get('computer_family', 'Unknown')} | {python_specs.get('cpu_model', 'Unknown')} | {python_specs.get('cpu_cores', 'Unknown')} | {python_specs.get('ram_gb', 'Unknown')} | {python_specs.get('os', 'Unknown')} |")
print(f"| Julia | {julia_specs.get('computer_family', 'Unknown')} | {julia_specs.get('cpu_model', 'Unknown')} | {julia_specs.get('cpu_cores', 'Unknown')} | {julia_specs.get('ram_gb', 'Unknown')} | {julia_specs.get('os', 'Unknown')} |")
print(f"| Rust | {rust_specs.get('computer_family', 'Unknown')} | {rust_specs.get('cpu_model', 'Unknown')} | {rust_specs.get('cpu_cores', 'Unknown')} | {rust_specs.get('ram_gb', 'Unknown')} | {rust_specs.get('os', 'Unknown')} |")
print("")
print("## Timing Results (microseconds)")
print("")
print("### Encoding")
print("")
print("| Array Size | Elements | Python (μs) | Julia (μs) | Rust (μs) | Rust Speedup vs Python | Rust Speedup vs Julia |")
print("|------------|----------|-------------|------------|-----------|------------------------|----------------------|")

for size_str in sizes:
    python_row = python_results.get(size_str, {}) if isinstance(python_results, dict) else {}
    julia_row = julia_results.get(size_str, {}) if isinstance(julia_results, dict) else {}
    rust_row = rust_results.get(size_str, {}) if isinstance(rust_results, dict) else {}

    python_encode = python_row.get("encode_us", {}).get("mean_us") if isinstance(python_row, dict) else None
    julia_encode = julia_row.get("encode_us", {}).get("mean_us") if isinstance(julia_row, dict) else None
    rust_encode = rust_row.get("encode_us", {}).get("mean_us") if isinstance(rust_row, dict) else None

    n_elements = 0
    for row in [python_row, julia_row, rust_row]:
        if isinstance(row, dict) and "n_elements" in row:
            n_elements = row["n_elements"]
            break

    python_str = f"{python_encode:.2f}" if python_encode is not None else "N/A"
    julia_str = f"{julia_encode:.2f}" if julia_encode is not None else "N/A"
    rust_str = f"{rust_encode:.2f}" if rust_encode is not None else "N/A"

    rust_vs_python = "N/A"
    if rust_encode is not None and rust_encode > 0 and python_encode is not None and python_encode > 0:
        rust_vs_python = f"{round(python_encode / rust_encode, 2)}x"
    rust_vs_julia = "N/A"
    if rust_encode is not None and rust_encode > 0 and julia_encode is not None and julia_encode > 0:
        rust_vs_julia = f"{round(julia_encode / rust_encode, 2)}x"

    print(f"| {size_str} | {n_elements} | {python_str} | {julia_str} | {rust_str} | {rust_vs_python} | {rust_vs_julia} |")

print("")
print("### Decoding")
print("")
print("| Array Size | Elements | Python (μs) | Julia (μs) | Rust (μs) | Rust Speedup vs Python | Rust Speedup vs Julia |")
print("|------------|----------|-------------|------------|-----------|------------------------|----------------------|")

for size_str in sizes:
    python_row = python_results.get(size_str, {}) if isinstance(python_results, dict) else {}
    julia_row = julia_results.get(size_str, {}) if isinstance(julia_results, dict) else {}
    rust_row = rust_results.get(size_str, {}) if isinstance(rust_results, dict) else {}

    python_decode = python_row.get("decode_us", {}).get("mean_us") if isinstance(python_row, dict) else None
    julia_decode = julia_row.get("decode_us", {}).get("mean_us") if isinstance(julia_row, dict) else None
    rust_decode = rust_row.get("decode_us", {}).get("mean_us") if isinstance(rust_row, dict) else None

    n_elements = 0
    for row in [python_row, julia_row, rust_row]:
        if isinstance(row, dict) and "n_elements" in row:
            n_elements = row["n_elements"]
            break

    python_str = f"{python_decode:.2f}" if python_decode is not None else "N/A"
    julia_str = f"{julia_decode:.2f}" if julia_decode is not None else "N/A"
    rust_str = f"{rust_decode:.2f}" if rust_decode is not None else "N/A"

    rust_vs_python = "N/A"
    if rust_decode is not None and rust_decode > 0 and python_decode is not None and python_decode > 0:
        rust_vs_python = f"{round(python_decode / rust_decode, 2)}x"
    rust_vs_julia = "N/A"
    if rust_decode is not None and rust_decode > 0 and julia_decode is not None and julia_decode > 0:
        rust_vs_julia = f"{round(julia_decode / rust_decode, 2)}x"

    print(f"| {size_str} | {n_elements} | {python_str} | {julia_str} | {rust_str} | {rust_vs_python} | {rust_vs_julia} |")

print("")
print("*Speedup values show how many times faster Rust is compared to other implementations.*")
PYTHON_SCRIPT

    python3 "$tmp_file" "$python_json_file" "$julia_json_file" "$rust_json_file"
    rm -f "$python_json_file" "$julia_json_file" "$rust_json_file" "$tmp_file"
}

if [ -n "$OUTPUT_FILE" ]; then
    generate_combined_report > "$OUTPUT_FILE"
    echo "Results written to: $OUTPUT_FILE"
else
    generate_combined_report
fi
