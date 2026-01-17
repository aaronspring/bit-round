# Bitround Benchmark Setup

This document describes how to set up and run benchmarks for the bitround compression algorithm across Python, Julia, and Rust implementations.

## Benchmark Results

See [README.md](./README.md#performance-benchmarks) for the latest benchmark results on Apple M2 Pro hardware.

## Overview

The benchmarks measure encoding and decoding performance for 3D arrays with edge lengths of 10^n (n=0 to 3), i.e., 1x1x1, 10x10x10, 100x100x100, and 1000x1000x1000. Input data simulates global 2m temperature readings with values ranging from 273K to 293K.

## Python Setup

### Prerequisites

- Python 3.10+
- pip package manager

### Installation

```bash
# Create a virtual environment (recommended)
python3 -m venv bitround-bench
source bitround-bench/bin/activate

# Install required packages
python3 -m pip install numpy numcodecs --break-system-packages
```

Note: The `--break-system-packages` flag is required on some systems (like macOS with Homebrew Python) to install packages system-wide.

### Running Python Benchmarks

```bash
# Run with default settings (16 bits, 3 warmup, 10 iterations)
python3 bench_python.py

# Run with custom settings
python3 bench_python.py --nbits 16 --warmup 3 --iterations 10

# Output as JSON for programmatic use
python3 bench_python.py --nbits 16 --iterations 10 --json

# Output as markdown table
python3 bench_python.py --nbits 16 --iterations 10 --markdown
```

### Python Output

The Python benchmark outputs timing results in microseconds for encoding and decoding operations, along with machine specifications (CPU model, cores, RAM, OS).

## Julia Setup

### Prerequisites

- Julia 1.10+
- Julia package manager

### Installation

Julia is installed via the official installer from https://julialang.org/downloads/

### Package Setup

The Julia benchmark requires the `JSON` package for output formatting:

```bash
# Start Julia and add the JSON package
julia

julia> import Pkg
julia> Pkg.add("JSON")
julia> exit()
```

### Running Julia Benchmarks

```bash
# Run with default settings
julia --project=. bench_julia.jl

# Run with custom settings
julia --project=. bench_julia.jl --nbits 16 --warmup 3 --iterations 10

# Output as JSON
julia --project=. bench_julia.jl --nbits 16 --iterations 10 --json

# Output as markdown
julia --project=. bench_julia.jl --nbits 16 --iterations 10 --markdown
```

## Rust Setup

### Prerequisites

- Rust 1.70+ (installed via rustup from https://rustup.rs/)
- Cargo (comes with Rust)

### Building

The Rust benchmark is built as a separate binary from the main library:

```bash
# Build in release mode for accurate benchmarks
cargo build --release --bin bench
```

### Running Rust Benchmarks

```bash
# Run with default settings
./target/release/bench

# Run with custom settings
./target/release/bench --nbits 16 --warmup 3 --iterations 10

# Output as JSON
./target/release/bench --nbits 16 --iterations 10 --json

# Output as markdown
./target/release/bench --nbits 16 --iterations 10 --markdown
```

For full optimization, build with:

```bash
cargo build --release --bin bench
```

## Combined Benchmark Runner

A Python script orchestrates all three benchmarks and generates a unified report:

```bash
# Run all benchmarks
python3 scripts/run_benchmarks.py

# Run with custom settings
python3 scripts/run_benchmarks.py --nbits 16 --warmup 3 --iterations 10

# Save results to file
python3 scripts/run_benchmarks.py --nbits 16 --output benchmark_results.md

# Run only specific implementations
python3 scripts/run_benchmarks.py --python-only
python3 scripts/run_benchmarks.py --julia-only
python3 scripts/run_benchmarks.py --rust-only
```

## Input Data

All benchmarks use random 3D arrays simulating global 2m temperature data:

- **Value range**: 273K to 293K (typical global surface temperature range)
- **Data type**: Float32
- **Random seed**: 42 (for reproducibility)

The data is generated with values scaled to the temperature range:

```python
# Python
np.random.seed(42)
data = (273 + np.random.rand(edge_size, edge_size, edge_size) * 20).astype(np.float32)
```

```julia
# Julia
Random.seed!(42)
data = 273 .+ rand(Float32, edge_size, edge_size, edge_size) .* 20
```

```rust
// Rust
let mut rng = fastrand::Rng::new();
rng.seed(42);
let data: Vec<f32> = (0..n_elements)
    .map(|_| 273.0 + rng.f32() * 20.0)
    .collect();
```

## Benchmark Methodology

1. **Warmup**: Each implementation runs 3 warmup iterations (configurable) to stabilize CPU caches and JIT compilation
2. **Measurement**: Timing measures only the bitround encode/decode function call, excluding:
   - Data generation time
   - Memory allocation/deallocation
   - Data transfer time
3. **Iterations**: Default 10 measured iterations (configurable)
4. **Statistics**: Mean, standard deviation, min, max, and median times are recorded

## Output Format

All benchmarks output:

1. **Console output**: Progress and per-iteration timing
2. **JSON format**: Machine-readable results with full statistics
3. **Markdown format**: Copy-pasteable tables for documentation

### JSON Output Structure

```json
{
  "implementation": {
    "1x1x1": {
      "n_elements": 1,
      "encode_us": {
        "mean_us": 0.02,
        "std_us": 0.03,
        "min_us": 0.01,
        "max_us": 0.05,
        "median_us": 0.02
      },
      "decode_us": {...}
    },
    ...
  },
  "machine_specs": {
    "computer_family": "MacBook M2 Pro",
    "cpu_model": "Apple M2 Pro",
    "cpu_cores": "10 cores",
    "ram_gb": "16.0 GB",
    "os": "macOS 26.2"
  }
}
```

## Reproducibility

All benchmarks use a fixed random seed (42) for input data generation, ensuring consistent results across runs and machines. To run identical benchmarks on different systems:

1. Use the same `nbits`, `warmup`, and `iterations` values
2. Ensure all implementations use the same random seed (hardcoded to 42)
3. Run in a consistent environment (same CPU frequency, power mode, etc.)

## Troubleshooting

### Python

- **ModuleNotFoundError**: Install required packages with pip
- **Slow first run**: NumPy and numcodecs may need to compile on first use

### Julia

- **Package not found**: Run `Pkg.add("JSON")` in Julia
- **Slow first run**: Julia JIT compiles functions on first execution

### Rust

- **Build errors**: Run `cargo build` to see detailed errors
- **Slow benchmarks**: Ensure release mode is used (`cargo build --release`)

## Machine Specifications

The benchmarks automatically detect and report:

- **Computer family**: e.g., "MacBook M2 Pro 16GB"
- **CPU model**: e.g., "Apple M2 Pro"
- **CPU cores**: Number of physical cores
- **RAM**: Total system memory
- **OS**: Operating system and version

## Comparison Notes

When comparing results across machines:

1. Normalize by clock speed if CPU frequencies differ
2. Consider the impact of different memory bandwidth
3. Note that Python/numcodecs may have different optimization levels
4. Julia's single-threaded performance may differ from multi-threaded
5. Rust's performance is typically the baseline for comparison
