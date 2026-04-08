#!/usr/bin/env python3
"""
Benchmark bitround for 3D arrays with edge lengths 10^n where n=0 to 3.
Tests encoding and decoding with random input data.
Outputs copy-pasteable results in Python, Julia, Rust order.
"""

import numpy as np
from numcodecs import BitRound
import time
import json
import sys
import platform
import subprocess


def get_machine_specs():
    """Gather machine specifications for benchmark report."""
    specs = {
        "computer_family": "Unknown",
        "cpu_model": "Unknown",
        "cpu_cores": "Unknown",
        "ram_gb": "Unknown",
        "os": platform.system() + " " + platform.release(),
    }

    specs["computer_family"] = " ".join(platform.platform().split("-")[:2])

    try:
        if sys.platform == "darwin":
            result = subprocess.run(
                ["sysctl", "-n", "machdep.cpu.brand_string"],
                capture_output=True,
                text=True,
            )
            specs["cpu_model"] = (
                result.stdout.strip() if result.returncode == 0 else "Unknown"
            )
            result = subprocess.run(
                ["sysctl", "-n", "hw.physicalcpu"], capture_output=True, text=True
            )
            specs["cpu_cores"] = (
                result.stdout.strip() + " cores"
                if result.returncode == 0
                else "Unknown"
            )
            result = subprocess.run(
                ["sysctl", "-n", "hw.memsize"], capture_output=True, text=True
            )
            ram_bytes = int(result.stdout.strip()) if result.returncode == 0 else 0
            specs["ram_gb"] = f"{ram_bytes / (1024**3):.1f} GB"
        else:
            with open("/proc/cpuinfo", "r") as f:
                for line in f:
                    if "model name" in line or "Model" in line:
                        specs["cpu_model"] = line.split(":")[1].strip()
                        break
            with open("/proc/cpuinfo", "r") as f:
                for line in f:
                    if "cpu cores" in line:
                        specs["cpu_cores"] = line.split(":")[1].strip() + " cores"
                        break
            with open("/proc/meminfo", "r") as f:
                for line in f:
                    if "MemTotal" in line:
                        ram_kb = int(line.split()[1])
                        specs["ram_gb"] = f"{ram_kb / (1024**2):.1f} GB"
                        break
    except Exception:
        pass

    return specs


def generate_random_3d_array(edge_size, seed=42):
    """Generate random 3D array simulating global 2m temperature data (273K to 293K)."""
    np.random.seed(seed)
    return (273.0 + np.random.rand(edge_size, edge_size, edge_size) * 20.0).astype(
        np.float32
    )


def time_encode_only(data, codec, n_iterations=10):
    """Time only the bitround encode operation."""
    data_flat = data.flatten()

    times = []
    for _ in range(n_iterations):
        start = time.perf_counter()
        result = codec.encode(data_flat)
        end = time.perf_counter()
        times.append((end - start) * 1e6)

    return {
        "mean_us": float(np.mean(times)),
        "std_us": float(np.std(times)),
        "min_us": float(np.min(times)),
        "max_us": float(np.max(times)),
        "median_us": float(np.median(times)),
    }


def time_decode_only(encoded_data, codec, n_iterations=10):
    """Time only the bitround decode operation."""
    times = []
    for _ in range(n_iterations):
        start = time.perf_counter()
        result = codec.decode(encoded_data)
        end = time.perf_counter()
        times.append((end - start) * 1e6)

    return {
        "mean_us": float(np.mean(times)),
        "std_us": float(np.std(times)),
        "min_us": float(np.min(times)),
        "max_us": float(np.max(times)),
        "median_us": float(np.median(times)),
    }


def run_benchmarks(nbits=16, n_warmup=3, n_iterations=10):
    """Run benchmarks for all 3D array sizes."""
    edge_sizes = [1, 10, 100, 1000]
    results = {"python": {}, "machine_specs": get_machine_specs()}

    print("Python bitround benchmark", file=sys.stderr)
    print("=" * 60, file=sys.stderr)
    print(f"Machine: {results['machine_specs']['computer_family']}", file=sys.stderr)
    print(f"CPU: {results['machine_specs']['cpu_model']}", file=sys.stderr)
    print(f"Cores: {results['machine_specs']['cpu_cores']}", file=sys.stderr)
    print(f"RAM: {results['machine_specs']['ram_gb']}", file=sys.stderr)
    print(f"OS: {results['machine_specs']['os']}", file=sys.stderr)
    print(file=sys.stderr)
    print(f"nbits: {nbits}", file=sys.stderr)
    print(f"warmup iterations: {n_warmup}", file=sys.stderr)
    print(f"measured iterations: {n_iterations}", file=sys.stderr)
    print(file=sys.stderr)

    for edge_size in edge_sizes:
        size_str = f"{edge_size}x{edge_size}x{edge_size}"
        n_elements = edge_size**3
        data_mb = n_elements * 4 / (1024 * 1024)

        print(
            f"Benchmarking {size_str} ({n_elements} elements, {data_mb:.3f} MB)...",
            file=sys.stderr,
        )

        data = generate_random_3d_array(edge_size)
        codec = BitRound(keepbits=nbits)

        for _ in range(n_warmup):
            codec.encode(data.flatten())

        encode_stats = time_encode_only(data, codec, n_iterations)

        encoded = codec.encode(data.flatten())
        decode_stats = time_decode_only(encoded, codec, n_iterations)

        results["python"][size_str] = {
            "n_elements": n_elements,
            "encode_us": encode_stats,
            "decode_us": decode_stats,
        }

        print(
            f"  Encode: {encode_stats['mean_us']:.2f} ± {encode_stats['std_us']:.2f} us",
            file=sys.stderr,
        )
        print(
            f"  Decode: {decode_stats['mean_us']:.2f} ± {decode_stats['std_us']:.2f} us",
            file=sys.stderr,
        )

    return results


def format_markdown_report(results):
    """Format results as copy-pasteable markdown table."""
    md = []
    md.append("## Python bitround Benchmark Results")
    md.append("")
    md.append("### Machine Specifications")
    md.append(f"- Computer: {results['machine_specs']['computer_family']}")
    md.append(f"- CPU: {results['machine_specs']['cpu_model']}")
    md.append(f"- Cores: {results['machine_specs']['cpu_cores']}")
    md.append(f"- RAM: {results['machine_specs']['ram_gb']}")
    md.append(f"- OS: {results['machine_specs']['os']}")
    md.append("")
    md.append("### Timing Results (microseconds)")
    md.append("")
    md.append("| Array Size | Elements | Encode (μs) | Decode (μs) |")
    md.append("|------------|----------|-------------|-------------|")

    for size_str, data in results["python"].items():
        encode_mean = data["encode_us"]["mean_us"]
        encode_std = data["encode_us"]["std_us"]
        decode_mean = data["decode_us"]["mean_us"]
        decode_std = data["decode_us"]["std_us"]
        n_elements = data["n_elements"]
        md.append(
            f"| {size_str} | {n_elements:,} | {encode_mean:.2f} ± {encode_std:.2f} | {decode_mean:.2f} ± {decode_std:.2f} |"
        )

    md.append("")
    return "\n".join(md)


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Benchmark Python bitround implementation"
    )
    parser.add_argument("--nbits", type=int, default=16, help="Number of bits to keep")
    parser.add_argument(
        "--warmup", type=int, default=3, help="Number of warmup iterations"
    )
    parser.add_argument(
        "--iterations", type=int, default=10, help="Number of measured iterations"
    )
    parser.add_argument("--json", action="store_true", help="Output JSON results")
    parser.add_argument("--markdown", action="store_true", help="Output markdown table")
    args = parser.parse_args()

    results = run_benchmarks(
        nbits=args.nbits, n_warmup=args.warmup, n_iterations=args.iterations
    )

    if args.json:
        print(json.dumps(results, indent=2))
    elif args.markdown:
        print(format_markdown_report(results))
    else:
        print("\n" + format_markdown_report(results))

    return results


if __name__ == "__main__":
    main()
