# Change: Add Core Bitround Encoding Algorithm

## Why
Climate data (NetCDF, Zarr) contains floating-point arrays with more precision than needed for scientific analysis. Bitround compression reduces storage size by keeping only the most significant bits while preserving information content, enabling smaller files without significant accuracy loss. The goal is to achieve performance comparable to or better than the Julia and Python reference implementations.

## What Changes
- Add `BitroundEncoder` struct with configurable `nbits` parameter
- Implement encoding algorithm for f32 and f64 arrays
- Create `encode` and `decode` functions
- Verify numerical accuracy against both Python numcodecs and Julia bitround.jl
- Benchmark performance against both Python numcodecs and Julia bitround.jl

## Impact
- Affected specs: `bitround`
- Affected code: New module `src/bitround.rs`, tests, benchmarks
- Reference implementations:
  - Python: https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py
  - Julia: https://github.com/bicycleben5/bitround.jl
