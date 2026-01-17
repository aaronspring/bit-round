# bitround

Rust crate for bitwise information analysis and compression, inspired by [BitInformation.jl](https://github.com/milankl/BitInformation.jl). This implementation provides comprehensive bitwise operations including IEEE rounding modes, bit transformations, and information theory functions.

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

## Reference Implementation Status

This implementation has been verified against two reference implementations:

1. **numcodecs (Python)** - The canonical reference implementation for bitround encoding
2. **BitInformation.jl (Julia)** - The original inspiration from the Nature paper

### Important Note on Julia Reference

During verification, we discovered a **sign handling bug** in the Julia `BitRound` reference implementation (`bitinformation.jl`). Negative numbers, -0.0, and infinities are incorrectly encoded (e.g., -1.0 becomes +1.0). See [POTENTIAL_BITINFORMATIONJL_BUG.md](./POTENTIAL_BITINFORMATIONJL_BUG.md) for full details, evidence, and root cause analysis.

## Reference

This implementation is based on the methodology from:

> M Klöwer, M Razinger, JJ Dominguez, PD Düben and TN Palmer, 2021. *Compressing atmospheric data into its real information content*. **Nature Computational Science** 1, 713–724. [10.1038/s43588-021-00156-2](https://doi.org/10.1038/s43588-021-00156-2)

## License

MIT License

## References

- [Julia implementation](https://github.com/milankl/BitInformation.jl)
- [Python implementation](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py)
- [Original bitround.jl](https://github.com/bicycleben5/bitround.jl)
- [Information-based bit allocation](https://github.com/observingClouds/xbitinfo)
