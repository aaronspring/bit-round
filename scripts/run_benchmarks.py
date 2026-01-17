#!/usr/bin/env python3
"""
Benchmark runner for bitround implementations.
Runs Python, Julia, and Rust benchmarks and generates a combined report.
"""

import subprocess
import json
import argparse
import os
import sys

SCRIPT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(SCRIPT_DIR)


def run_command(cmd, description):
    """Run a command and capture its output."""
    print(f"Running {description}...")
    try:
        result = subprocess.run(
            cmd, shell=True, capture_output=True, text=True, timeout=300, cwd=SCRIPT_DIR
        )
        if result.returncode != 0:
            print(f"Error running {description}: {result.stderr}")
            return None
        return result.stdout
    except subprocess.TimeoutExpired:
        print(f"Timeout running {description}")
        return None
    except Exception as e:
        print(f"Error running {description}: {e}")
        return None


def run_python_benchmark(nbits, warmup, iterations):
    """Run Python benchmark and return results."""
    cmd = f"python3 bench_python.py --nbits {nbits} --warmup {warmup} --iterations {iterations} --json"
    output = run_command(cmd, "Python benchmark")
    if output:
        try:
            return json.loads(output)
        except json.JSONDecodeError as e:
            print(f"Error parsing Python JSON: {e}")
            print(f"Output: {output[:500]}")
    return None


def run_julia_benchmark(nbits, warmup, iterations):
    """Run Julia benchmark and return results."""
    cmd = f"julia --project=. bench_julia.jl --nbits {nbits} --warmup {warmup} --iterations {iterations} --json"
    output = run_command(cmd, "Julia benchmark")
    if output:
        try:
            return json.loads(output)
        except json.JSONDecodeError as e:
            print(f"Error parsing Julia JSON: {e}")
            print(f"Output: {output[:500]}")
    return None


def run_rust_benchmark(nbits, warmup, iterations):
    """Run Rust benchmark and return results."""
    cmd = f"cargo run --release --bin bench -- --nbits {nbits} --warmup {warmup} --iterations {iterations} --json"
    output = run_command(cmd, "Rust benchmark")
    if output:
        try:
            return json.loads(output)
        except json.JSONDecodeError as e:
            print(f"Error parsing Rust JSON: {e}")
            print(f"Output: {output[:500]}")
    return None


def get_machine_specs(data):
    """Extract machine specs from benchmark data."""
    if "machine_specs" in data:
        return data["machine_specs"]
    return {
        "computer_family": "Unknown",
        "cpu_model": "Unknown",
        "cpu_cores": "Unknown",
        "ram_gb": "Unknown",
        "os": "Unknown",
    }


def get_results(data, key):
    """Extract results from benchmark data."""
    if key in data:
        return data[key]
    return {}


def format_markdown_report(python_data, julia_data, rust_data):
    """Format combined results as markdown."""
    python_specs = get_machine_specs(python_data) if python_data else {}
    julia_specs = get_machine_specs(julia_data) if julia_data else {}
    rust_specs = get_machine_specs(rust_data) if rust_data else {}

    python_results = get_results(python_data, "python") if python_data else {}
    julia_results = get_results(julia_data, "julia") if julia_data else {}
    rust_results = get_results(rust_data, "rust") if rust_data else {}

    md = []
    md.append("# Bitround Benchmark Results")
    md.append("")
    md.append("## Machine Specifications")
    md.append("")
    md.append("| Implementation | Computer | CPU | Cores | RAM | OS |")
    md.append("|----------------|----------|-----|-------|-----|-----|")
    md.append(
        f"| Python | {python_specs.get('computer_family', 'Unknown')} | {python_specs.get('cpu_model', 'Unknown')} | {python_specs.get('cpu_cores', 'Unknown')} | {python_specs.get('ram_gb', 'Unknown')} | {python_specs.get('os', 'Unknown')} |"
    )
    md.append(
        f"| Julia | {julia_specs.get('computer_family', 'Unknown')} | {julia_specs.get('cpu_model', 'Unknown')} | {julia_specs.get('cpu_cores', 'Unknown')} | {julia_specs.get('ram_gb', 'Unknown')} | {julia_specs.get('os', 'Unknown')} |"
    )
    md.append(
        f"| Rust | {rust_specs.get('computer_family', 'Unknown')} | {rust_specs.get('cpu_model', 'Unknown')} | {rust_specs.get('cpu_cores', 'Unknown')} | {rust_specs.get('ram_gb', 'Unknown')} | {rust_specs.get('os', 'Unknown')} |"
    )
    md.append("")
    md.append("## Timing Results (microseconds)")
    md.append("")
    md.append("### Encoding")
    md.append("")
    md.append("| Array Size | Elements | Python | Julia | Rust |")
    md.append("|------------|----------|--------|-------|------|")

    sizes = ["1x1x1", "10x10x10", "100x100x100"]

    for size_str in sizes:
        python_row = python_results.get(size_str, {})
        julia_row = julia_results.get(size_str, {})
        rust_row = rust_results.get(size_str, {})

        python_encode = python_row.get("encode_us", {}).get("mean_us", 0)
        julia_encode = julia_row.get("encode_us", {}).get("mean_us", 0)
        rust_encode = rust_row.get("encode_us", {}).get("mean_us", 0)

        n_elements = python_row.get(
            "n_elements", julia_row.get("n_elements", rust_row.get("n_elements", 0))
        )

        md.append(
            f"| {size_str} | {n_elements} | {python_encode:.2f} | {julia_encode:.2f} | {rust_encode:.2f} |"
        )

    md.append("")
    md.append("### Decoding")
    md.append("")
    md.append("| Array Size | Elements | Python | Julia | Rust |")
    md.append("|------------|----------|--------|-------|------|")

    for size_str in sizes:
        python_row = python_results.get(size_str, {})
        julia_row = julia_results.get(size_str, {})
        rust_row = rust_results.get(size_str, {})

        python_decode = python_row.get("decode_us", {}).get("mean_us", 0)
        julia_decode = julia_row.get("decode_us", {}).get("mean_us", 0)
        rust_decode = rust_row.get("decode_us", {}).get("mean_us", 0)

        n_elements = python_row.get(
            "n_elements", julia_row.get("n_elements", rust_row.get("n_elements", 0))
        )

        md.append(
            f"| {size_str} | {n_elements} | {python_decode:.2f} | {julia_decode:.2f} | {rust_decode:.2f} |"
        )

    md.append("")
    md.append("*Times shown are mean values in microseconds.*")
    md.append("")

    return "\n".join(md)


def main():
    parser = argparse.ArgumentParser(description="Run bitround benchmarks")
    parser.add_argument("--nbits", type=int, default=16, help="Number of bits to keep")
    parser.add_argument(
        "--warmup", type=int, default=3, help="Number of warmup iterations"
    )
    parser.add_argument(
        "--iterations", type=int, default=10, help="Number of measured iterations"
    )
    parser.add_argument(
        "--output", type=str, default="", help="Output file for results"
    )
    parser.add_argument(
        "--python-only", action="store_true", help="Run only Python benchmark"
    )
    parser.add_argument(
        "--julia-only", action="store_true", help="Run only Julia benchmark"
    )
    parser.add_argument(
        "--rust-only", action="store_true", help="Run only Rust benchmark"
    )
    args = parser.parse_args()

    print("=" * 60)
    print("Bitround Benchmark Runner")
    print("=" * 60)
    print()
    print("Configuration:")
    print(f"  nbits: {args.nbits}")
    print(f"  warmup iterations: {args.warmup}")
    print(f"  measured iterations: {args.iterations}")
    print()

    python_data = None
    julia_data = None
    rust_data = None

    if not args.julia_only and not args.rust_only:
        python_data = run_python_benchmark(args.nbits, args.warmup, args.iterations)
        print("Python benchmark complete.")
        print()

    if not args.python_only and not args.rust_only:
        julia_data = run_julia_benchmark(args.nbits, args.warmup, args.iterations)
        print("Julia benchmark complete.")
        print()

    if not args.python_only and not args.julia_only:
        rust_data = run_rust_benchmark(args.nbits, args.warmup, args.iterations)
        print("Rust benchmark complete.")
        print()

    report = format_markdown_report(python_data, julia_data, rust_data)

    if args.output:
        with open(args.output, "w") as f:
            f.write(report)
        print(f"Results written to: {args.output}")
    else:
        print(report)


if __name__ == "__main__":
    main()
