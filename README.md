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

## Performance Benchmarks

**Fairness fixes applied**:
- Julia benchmark now uses allocating semantics (matching Python/Rust) instead of pre-allocated in-place buffers
- Julia GC disabled during timed iterations to isolate compute performance
- Python iteration count is now consistent across all array sizes (was reduced to 2 for 1000³)
- Rust decode no longer applies a redundant mask (encode already zeroes trailing bits)
- Algorithm difference documented: Julia uses IEEE round-to-nearest (BitInformation.jl), Python/Rust use numcodecs-style rounding

**TODO**: Further improvements for single-core execution as specified in `openspec/changes/update-benchmarking-single-core/specs/benchmarking/spec.md`:
- Pin to single CPU core for fair language comparison
- Consistent allocator (jemalloc/mimalloc for Rust)
- CPU frequency locked to base clock

See [BENCHMARK_SETUP.md](./BENCHMARK_SETUP.md) for methodology and [openspec/changes/update-benchmarking-single-core/RESEARCH.md](./openspec/changes/update-benchmarking-single-core/RESEARCH.md) for benchmarking best practices.

Benchmarks measure encode/decode performance for 3D arrays (Float32) with edge lengths 10^n.

**Machine**: Apple M2 Pro (10 cores), 16 GB RAM, macOS 26.2
**Configuration**: 16 bits, 10 iterations (2 for 1000³), 3 warmup iterations

| Array Size | Elements | Implementation | Encode (μs) | Decode (μs) |
|------------|----------|----------------|-------------|-------------|
| 1×1×1 | 1 | Python | 10.89 ± 1.26 | 1.80 ± 1.79 |
| | | **Julia** | **0.04 ± 0.01** | **0.03 ± 0.04** |
| | | Rust | 0.06 ± 0.03 | 0.09 ± 0.08 |
| 10×10×10 | 1,000 | Python | 10.44 ± 0.69 | 1.10 ± 0.20 |
| | | **Julia** | **0.17 ± 0.05** | **0.11 ± 0.06** |
| | | Rust | 0.84 ± 0.02 | 0.96 ± 0.97 |
| 100×100×100 | 1,000,000 | Python | 1264.83 ± 121.83 | 1.35 ± 0.79 |
| | | **Julia** | **134.47 ± 2.85** | 126.58 ± 41.67 |
| | | Rust | 833.18 ± 24.35 | 629.72 ± 71.18 |
| 1000×1000×1000 | 1,000,000,000 | Python | 32,906,771 ± 353,711 | 1,823 ± 1,523 |
| | | Julia | 6,083,679 ± 2,928,600 | 4,573,157 ± 487,245 |
| | | **Rust** | **835,025 ± 2,165** | **3,087,767 ± 3,578,593** |

> **Note**: These benchmark results predate the fairness fixes above and should be re-run.
> Julia times in particular are expected to increase now that allocation is included in timing.
>
> **Algorithm caveat**: Julia uses `BitInformation.jl` (IEEE round-to-nearest-ties-to-even),
> while Python and Rust use numcodecs-style rounding. The benchmark compares each ecosystem's
> canonical implementation rather than identical algorithms.

> **Warning**: See [A Warning on Mechanical Sympathy](https://matthewrocklin.com/blog/work/2017/03/09/biased-benchmarks)
> by Matthew Rocklin for important caveats when comparing performance across
> implementations.
>
> For full methodology and running benchmarks, see [BENCHMARK_SETUP.md](./BENCHMARK_SETUP.md).

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
