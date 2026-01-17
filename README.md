# bitround

Rust crate for bitround compression, inspired by [bitround.jl](https://github.com/bicycleben5/bitround.jl) and [numcodecs.bitround](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py).

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

```rust
use bitround::BitroundEncoder;

let data = vec![1.0f32, 2.0, 3.0, 4.0];
let encoder = BitroundEncoder::new(16);
let encoded = encoder.encode_f32(&data).unwrap();
let decoded = encoder.decode_f32(&encoded).unwrap();
```

## References

- [Python implementation](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py)
- [Julia implementation](https://github.com/bicycleben5/bitround.jl)
- [Information-based bit allocation](https://github.com/observingClouds/xbitinfo)
