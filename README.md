# bit-round

> **Note**: This project is a personal learning exercise exploring Rust programming. The entire codebase was written by [opencode](https://opencode.ai) using the [MinMax M2.1 model](https://opencode.ai/docs/models/) under [spec-driven development](https://youtu.be/8rABwKRsec4?si=ZDUrifwn3xAJPmkU&t=380) with [openspec](https://github.com/Fission-AI/OpenSpec)

Rust crate for bitwise information analysis and compression, inspired by [BitInformation.jl](https://github.com/milankl/BitInformation.jl). This implementation provides comprehensive bitwise operations including IEEE rounding modes, bit transformations, and information theory functions.

## Installation

### macOS

```bash
brew install rust
```

Then build and test:

```bash
cargo build
cargo test
cargo bench
```

### Linux

Install via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Real-World Use Case: Climate Data Compression

This project includes a complete workflow for compressing climate data, demonstrated in the `climate-bitround` CLI tool. The use case is specified in `openspec/changes/climate-data-bitround-usecase/`.

### Data Source

**NOAA-GFDL GFDL-ESM4 ssp585 r1i1p1f1 (zos - sea surface height)**

- CMIP6 ScenarioMIP experiment
- 1.6 GB dataset (2015-2100 monthly data)
- Source: Google Cloud Storage (gs://cmip6)

### Downloading the Data

The CMIP6 data is available as Zarr format on Google Cloud Storage. Download using curl:

```bash
BASE_URL="https://storage.googleapis.com/cmip6/CMIP6/ScenarioMIP/NOAA-GFDL/GFDL-ESM4/ssp585/r1i1p1f1/Omon/zos/gn/v20180701"

mkdir -p /tmp/cmip6_zarr/zos

# Download metadata files
curl -s "$BASE_URL/zos/.zarray" -o /tmp/cmip6_zarr/zos/.zarray
curl -s "$BASE_URL/zos/.zattrs" -o /tmp/cmip6_zarr/zos/.zattrs
curl -s "$BASE_URL/zos/.zgroup" -o /tmp/cmip6_zarr/zos/.zgroup

# Download data chunks (17 chunks, ~90MB each)
for i in {0..16}; do
    echo "Downloading chunk $i..."
    curl -s "$BASE_URL/zos/$i.0.0" -o /tmp/cmip6_zarr/zos/$i.0.0
done
```

Then run the compression:

```bash
cargo run --release --bin climate-bitround -- compress \
  -i /tmp/cmip6_zarr \
  -o /tmp/cmip6_compressed \
  --significance 0.99
```

### CLI Commands

```bash
# Show info about a Zarr store
cargo run --bin climate-bitround -- info /tmp/cmip6_zarr

# Compress with automatic keff calculation (99% information preserved)
cargo run --bin climate-bitround -- compress -i /path/to/input -o /path/to/output

# Compress with explicit number of bits
cargo run --bin climate-bitround -- compress -i /path/to/input -o /path/to/output --nbits 16
```

### Compression Results

**Dataset**: GFDL-ESM4 zos (sea surface height), 1032×576×720 float32 array

| Info | Bits | Compressed | vs 100% | Max Rel. Error |
|------|------|------------|---------|----------------|
| 100% | 23 | 933 MB | 1.00× | 1.2e-7 |
| 99% | 23 | 933 MB | 1.00× | 1.2e-7 |
| 95% | 22 | 921 MB | 1.01× | 2.4e-7 |
| 90% | 21 | 902 MB | 1.03× | 4.8e-7 |

*Note: This dataset has very high information content across all mantissa bits, resulting in minimal compression gains from keff-based bitrounding. Try with noisier datasets (e.g., temperature, precipitation) for more significant compression.*

**Key insight**: Compression effectiveness depends on the data's inherent information structure. Sea surface height data is highly structured with information distributed across most bits.


### Workflow

1. Download Zarr format climate data from ESGF
2. Calculate `keff` (effective bits) using entropy-based analysis
3. Apply bitround at calculated precision level
4. Save with zstd compression
5. Compare storage sizes before/after

## Usage

### Basic Rounding (IEEE Round to Nearest Tie to Even)

```rust
use bitround::BitroundEncoder;

let data = vec![1.0f32, 2.0, 3.0, 4.0];
let encoder = BitroundEncoder::new(16).unwrap();
let encoded = encoder.encode_f32(&data).unwrap();
let decoded = encoder.decode_f32(&encoded).unwrap();
```

### Shaving Functions

```rust
use bitround::BitroundEncoder;

let mut data = vec![1.5f32, 2.7, 3.14];
let encoder = BitroundEncoder::new(10).unwrap();
encoder.shave_f32_inplace(&mut data);
```

### Information Analysis

```rust
use bitround::information::{bitcount_array_f32, bitinformation_f32, mutual_information_f32};

let data = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];

// Count 1-bits in each position
let bit_counts = bitcount_array_f32(&data);

// Calculate bitwise information content
let bit_info = bitinformation_f32(&data, 2.0);

// Compute mutual information between arrays
let data2 = vec![1.1f32, 2.1, 3.1, 4.1, 5.1];
let mutual_info = mutual_information_f32(&data, &data2, 2.0);
```

### Bit Transformations

```rust
use bitround::transformations::{bittranspose, xor_delta, signed_exponent};

let value = 3.14f32;
let transposed = bittranspose(value);

// Compute XOR delta for array
let data = vec![1.0f32, 2.0, 3.0, 4.0];
let deltas = xor_delta(&data);

// Get signed exponent
let signed_exp = signed_exponent(value);
```

## Features

### Rounding Functions
- **IEEE Round to Nearest Tie to Even**: Proper IEEE 754 rounding for Float32/Float64
- **Shave**: Round towards zero by setting trailing bits to 0
- **Halfshave**: Round to halfway between shave and IEEE round
- **Set One**: Round away from zero by setting trailing bits to 1
- **Groom**: Alternating shave/set pattern for bit optimization

### Bit Transformations
- **Bit Transpose**: Reverse bit order (bit shuffle)
- **XOR Delta**: Compute successive XOR differences
- **Signed/Biased Exponent**: Extract and transform exponent bits

### Information Theory Functions
- **Bit Count**: Count 1-bits across all positions
- **Bit Count Entropy**: Calculate entropy per bit position
- **Mutual Information**: Bitwise mutual information between arrays
- **Redundancy**: Normalized mutual information (0-1 scale)
- **Bit Information**: Information content from adjacent array entries
- **Bit Pattern Entropy**: Entropy from unique bit patterns
- **Statistical Functions**: Binomial confidence intervals, free entropy

## Cross-implementation equivalence

This crate's bitround output is **bitwise-identical** to both reference
implementations on byte-identical input:

- [numcodecs.BitRound](https://github.com/zarr-developers/numcodecs/blob/main/src/numcodecs/bitround.py) (Python)
- [BitInformation.jl](https://github.com/milankl/BitInformation.jl)'s `round(x, keepbits)` (Julia)

To verify locally:

```bash
cargo build --release --bin encode-file
venv/bin/python scripts/verify_equivalence.py --sizes 10,100 --keepbits 16
```

The script generates one reference `f32` array, encodes it with all three
implementations on the same raw bytes, and asserts the resulting `u32`
streams are bitwise equal. Skip Julia with `--no-julia`.

How it works:

- Python path: `numcodecs.BitRound(keepbits=...).encode(flat)`
- Rust path: `target/release/encode-file --input ... --keepbits ... --output ...`
  ([`src/bin/encode_file.rs`](./src/bin/encode_file.rs))
- Julia path: `julia scripts/encode_file.jl --input ... --keepbits ... --output ...`
  ([`scripts/encode_file.jl`](./scripts/encode_file.jl))

Result on Apple M2 Pro / macOS 26.3 (sizes 10³ and 100³, keepbits=16):
**PASS — all three implementations bitwise identical**.

## Performance benchmarks

Two harnesses live in this repo:

1. **In-process timing** (`bench_python.py`, `bench_julia.jl`,
   `cargo run --release --bin bench`) — wraps `encode`/`decode` only,
   matched 3D array sizes and seed.
2. **Subprocess time + peak memory** (`scripts/bench_memory.py`) — runs each
   implementation as a fresh process under `/usr/bin/time -l`/`-v`, captures
   wall time and peak RSS plus the in-process encode time. This is the
   harness used for the table below.

```bash
cargo build --release --bin bench-oneshot
venv/bin/python scripts/bench_memory.py --sizes 10,100 --keepbits 16 --repeats 3
```

### Time + peak RSS — keepbits=16, 3 repeats

Encode time is measured inside each process around the encode call only
(JIT/imports excluded). Wall and peak RSS are whole-process and therefore
include runtime startup — Julia's RSS in particular is dominated by the JIT
and the BitInformation.jl dependency tree.

**Machine**: Apple M2 Pro, macOS 26.3 arm64. **Single-core pinning is not
yet applied** (still TODO — see [BENCHMARK_SETUP.md](./BENCHMARK_SETUP.md)),
so numbers are indicative.

| Implementation | Size | Encode (median) | Wall (median) | Peak RSS |
|----------------|------|------------------|----------------|----------|
| python (numcodecs 0.16.5)     | 10³  | 24.7 μs  | 150 ms  | 37.8 MB  |
| python (numcodecs 0.16.5)     | 100³ | 1.25 ms  | 160 ms  | 57.4 MB  |
| rust (this repo)              | 10³  | 1.6 μs   | 10 ms   | 5.6 MB   |
| rust (this repo)              | 100³ | 815.9 μs | 10 ms   | 13.3 MB  |
| julia (BitInformation.jl 0.6.3) | 10³  | 0.3 μs   | 1.06 s  | 309.5 MB |
| julia (BitInformation.jl 0.6.3) | 100³ | 259.6 μs | 1.08 s  | 321.0 MB |

How to read the columns:

- **Encode**: pure codec speed after warmup. Julia is fastest (tight
  `@inbounds` loop JIT'd to vectorized native), Rust second, Python slowest.
- **Wall**: cost of one CLI invocation. Rust dominates because Python and
  Julia pay heavy startup; for batch/long-running jobs the encode column is
  closer to what you'll see.
- **Peak RSS**: includes the runtime. Rust ~5–13 MB, Python ~38–57 MB, Julia
  ~310–321 MB.

> **Warning**: These benchmarks are not yet single-core-pinned and noise can
> be ±20% at sub-millisecond timings. See [A Warning on Mechanical
> Sympathy](https://matthewrocklin.com/blog/work/2017/03/09/biased-benchmarks)
> for caveats. For methodology, see [BENCHMARK_SETUP.md](./BENCHMARK_SETUP.md).

## Information-based keepbits selection

`keff::get_keepbits_f32` / `get_keepbits_f64` implements the
[BitInformation.jl](https://github.com/milankl/BitInformation.jl) algorithm
for picking how many mantissa bits to keep at a target information level:

1. compute mutual information between adjacent array entries, per bit position
2. zero out bits whose MI sits below the binomial free-entropy noise floor
   (confidence 0.99) — this separates real information from sampling noise
3. return the smallest number of mantissa bits whose cumulative MI ≥
   `inflevel × total`

```rust
use bit_round::keff::get_keepbits_f32;

let data: Vec<f32> = /* ... */;
let keepbits = get_keepbits_f32(&data, 0.99).unwrap();
let encoder = bit_round::bitround::BitroundEncoder::new(keepbits as u8).unwrap();
let compressed = encoder.encode_f32(&data).unwrap();
```

The older entropy-based `calculate_keff_f32` / `calculate_keff_f64` are kept
for backwards compatibility but do **not** match BitInformation.jl. Prefer
`get_keepbits_*` for new code.

## Reference Implementation

This implementation is based on the methodology from:

> M Klöwer, M Razinger, JJ Dominguez, PD Düben and TN Palmer, 2021. *Compressing atmospheric data into its real information content*. **Nature Computational Science** 1, 713–724. [10.1038/s43588-021-00156-2](https://doi.org/10.1038/s43588-021-00156-2)

## License

MIT License

## References

- [BitInformation.jl](https://github.com/milankl/BitInformation.jl)
- [Python implementation](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py)
- [Binary rounding implementation](https://github.com/dynamical-org/reformatters/blob/b92207bf3f585a27582214840ae9d2e416fcb2d4/src/reformatters/common/binary_rounding.py)
- [Information-based bit allocation](https://github.com/observingClouds/xbitinfo)
