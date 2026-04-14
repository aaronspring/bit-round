# Verification Strategy

This document describes the verification strategy for the Rust bitround implementation against Python numcodecs and Julia BitInformation.jl reference implementations.

## Overview

Verification ensures the Rust implementation produces **bit-identical output** to reference implementations. This is critical for:
- Correctness guarantee
- Fair benchmarking comparisons
- Algorithm validation

## Quick equivalence check (no Docker)

For a fast end-to-end check against numcodecs and BitInformation.jl on
byte-identical input, use the standalone driver:

```bash
cargo build --release --bin encode-file
venv/bin/python scripts/verify_equivalence.py --sizes 10,100 --keepbits 16
# add --no-julia if julia / BitInformation.jl is not installed
```

The driver generates one reference `f32` array, encodes it via:

- `numcodecs.BitRound(keepbits=...).encode(...)` (Python)
- `target/release/encode-file` ([`src/bin/encode_file.rs`](./src/bin/encode_file.rs))
- `julia scripts/encode_file.jl` (uses BitInformation.jl's `round(x, keepbits)`)

…then asserts the resulting `u32` streams are bitwise identical. This is the
fastest path to validate "Rust ≡ numcodecs ≡ BitInformation.jl" and is
used by the README's *Cross-implementation equivalence* section. Result on
Apple M2 Pro / macOS 26.3 at sizes 10³ and 100³, keepbits=16: **PASS**.

The Docker-based reference-data flow described below is a more thorough
fixture-driven path and remains the canonical approach for `cargo test`
verification.

## Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Rust Tests    │────▶│  Reference Data │────▶│   Python/Julia  │
│ (verification)  │     │   (generated)   │     │  (via Docker)   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Test Data

Test inputs are generated locally using `cargo run --bin generate_test_inputs`:

| File | Description | Size |
|------|-------------|------|
| `testdata/inputs/zeros_f32.bin` | Array of zeros | 1000 elements |
| `testdata/inputs/constants_f32.bin` | Array of 1.5 | 1000 elements |
| `testdata/inputs/random_f32.bin` | Random values (seed=42) | 1000 elements |
| `testdata/inputs/edge_f32.bin` | Edge cases (NaN, Inf, etc.) | 100 elements |
| `testdata/inputs/zeros_f64.bin` | Array of zeros | 1000 elements |
| `testdata/inputs/constants_f64.bin` | Array of 1.5 | 1000 elements |
| `testdata/inputs/random_f64.bin` | Random values (seed=42) | 1000 elements |
| `testdata/inputs/edge_f64.bin` | Edge cases (NaN, Inf, etc.) | 100 elements |

## Reference Data Generation

Reference outputs must be generated via Docker to ensure reproducibility:

```bash
# Generate all reference data (Python and Julia)
./scripts/generate_reference_data.sh

# Or individually:
./scripts/verify_python.sh
./scripts/verify_julia.sh
```

This builds Docker images and runs them to generate reference outputs:
- `testdata/python/` - Python numcodecs bitround outputs
- `testdata/julia/` - Julia bitround.jl outputs

## Running Tests

### Generate test inputs (one-time)
```bash
cargo run --bin generate_test_inputs
```

### Generate reference data (requires Docker)
```bash
./scripts/generate_reference_data.sh
```

### Run verification tests
```bash
# Python verification
cargo test --test verification_python

# Julia verification
cargo test --test verification_julia

# All tests
cargo test --test verification_python --test verification_julia
```

## Test Coverage

### nbits values tested
- **f32**: 1, 8, 16, 24
- **f64**: 1, 16, 32, 53

### Data patterns tested
- Zeros (tests handling of all-zero arrays)
- Constants (tests uniform value encoding)
- Random (tests typical data patterns)
- Edge cases (tests NaN, Inf, -Inf, subnormals)

## Docker Images

### Python (`docker/Dockerfile.python`)
- Python 3.11-slim
- numpy
- numcodecs

### Julia (`docker/Dockerfile.julia`)
- Julia 1.10
- Custom bitround implementation

## CI/CD

In CI, the workflow is:
1. Generate test inputs locally
2. Generate reference data via Docker
3. Run verification tests
4. Archive results

## Troubleshooting

### "Failed to open testdata/inputs/..."
Run: `cargo run --bin generate_test_inputs`

### "Failed to open testdata/python/..."
Run: `./scripts/generate_reference_data.sh` (requires Docker)

### Docker build fails
Ensure Docker is running and has sufficient memory allocated.

## Reproducibility

- Test inputs use fixed seed (42) for random generation
- Docker images are rebuilt each time to get latest reference code
- Reference data is committed to repository for historical comparison
