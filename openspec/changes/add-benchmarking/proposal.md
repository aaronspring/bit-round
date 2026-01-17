# Change: Add Benchmarking Comparison (Rust vs Python vs Julia)

## Why
Performance benchmarking is essential to demonstrate Rust's speed advantages over reference implementations and to identify optimization opportunities. Fair benchmarking requires running all implementations under equivalent conditions.

## What Changes
- Add benchmark comparison of Rust bitround vs Python numcodecs vs Julia bitround.jl
- Run all implementations in Docker containers for fair comparison
- Benchmark encoding and decoding throughput for f32 and f64
- Test various array sizes and nbits values
- Generate benchmark reports with speedup comparisons

## Impact
- Affected specs: `benchmarking`, `bitround`
- Affected code: New benchmark scripts in `benches/`, results in `benchmark_results/`
- Docker images: Same images used for verification (`python-verification`, `julia-verification`, add `rust-verification`)
- Reference implementations:
  - Python: https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py
  - Julia: https://github.com/bicycleben5/bitround.jl
