# Change: Add Python and Julia Implementation Verification

## Why
The Rust bitround implementation must produce bit-identical output to reference implementations to ensure correctness and enable fair performance comparisons. Verification against Python numcodecs and Julia bitround.jl ensures the algorithm is correctly implemented.

## What Changes
- Add verification tests comparing Rust output to Python numcodecs bitround
- Add verification tests comparing Rust output to Julia bitround.jl
- Create reference test data with known expected outputs
- Document verification methodology and test cases

## Impact
- Affected specs: `verification`, `bitround`
- Affected code: New verification tests in `tests/`, test data in `testdata/`
- Reference implementations:
  - Python: https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py
  - Julia: https://github.com/bicycleben5/bitround.jl
