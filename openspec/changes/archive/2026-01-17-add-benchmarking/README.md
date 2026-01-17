# Archived: Add Benchmarking Infrastructure

**Date**: 2026-01-17
**Status**: Archived - to be revisited

## Summary

Added comprehensive benchmarking infrastructure for comparing Rust bitround implementation against Python (numcodecs) and Julia (BitInformation.jl) reference implementations.

## Changes

- `scripts/run_benchmarks.sh` - Robust shell script for running all benchmarks
- `bench_python.py` - Python benchmark script with JSON output
- `bench_julia.jl` - Julia benchmark script with JSON output
- `src/bin/bench.rs` - Rust benchmark binary with JSON output
- `BENCHMARK_SETUP.md` - Benchmark methodology documentation
- `README.md` - Updated with n=0,1,2,3 benchmark results

## Results Summary

| Array Size | Elements | Rust Encode | Rust Decode | Python Encode | Python Decode |
|------------|----------|-------------|-------------|---------------|---------------|
| 1×1×1 | 1 | 0.06 μs | 0.09 μs | 10.89 μs | 1.80 μs |
| 10×10×10 | 1,000 | 0.84 μs | 0.96 μs | 10.44 μs | 1.10 μs |
| 100×100×100 | 1,000,000 | 833 μs | 630 μs | 1,265 μs | 1.35 μs |
| 1000×1000×1000 | 1,000,000,000 | 835,025 μs | 3,087,767 μs | 32,906,771 μs | 1,823 μs |

## Key Findings

- Rust is 12-181× faster for encoding small arrays
- Rust is 1.5-39× faster for encoding large arrays
- Python decode is anomalously fast (~1-2 μs) due to numcodecs vectorization
- Julia benchmarks not yet integrated (sign handling bug in reference)

## Reasons for Archiving

1. Julia reference implementation has sign handling bug (negative numbers encoded incorrectly)
2. Python decode performance anomaly needs investigation
3. Need more comprehensive benchmarking methodology

## TODO: Revisit

- [ ] Re-run benchmarks after Julia bug fix
- [ ] Investigate Python decode performance anomaly
- [ ] Add more array sizes (non-power-of-10)
- [ ] Add memory usage benchmarks
- [ ] Add multi-threaded benchmarks
- [ ] Compare against additional implementations
