## 1. Core Algorithm Implementation
- [x] 1.1 Create `src/lib.rs` and `src/bitround.rs` module
- [x] 1.2 Implement `BitroundEncoder` struct with `nbits` field
- [x] 1.3 Implement `new()` constructor (validates 1-53 nbits)
- [x] 1.4 Implement `new_f32()` constructor (validates 1-24 nbits)
- [x] 1.5 Implement `new_f64()` constructor (validates 1-53 nbits)
- [x] 1.6 Implement `encode_f32` function
- [x] 1.7 Implement `decode_f32` function
- [x] 1.8 Implement `encode_f64` function
- [x] 1.9 Implement `decode_f64` function

## 2. Testing and Verification
- [x] 2.1 Write roundtrip tests (encode → decode preserves values)
- [x] 2.2 Add tests with various `nbits` values (4, 8, 16, 24, 32 for f32)
- [x] 2.3 Add tests with various `nbits` values (4, 8, 16, 32, 48, 53 for f64)
- [x] 2.4 Python/Julia verification moved to `add-python-julia-verification`
- [x] 2.5 Benchmark vs Python/Julia moved to `add-python-julia-verification`

## 3. Benchmarking
- [x] 3.1 Set up benchmarks using `criterion`
- [x] 3.2 Rust-only benchmarks: encode_f32 ~793 MB/s, decode_f32 ~1.0 GB/s
- [x] 3.3 Rust-only benchmarks: encode_f64 ~695 MB/s, decode_f64 ~1.4 GB/s

## 4. Code Quality
- [x] 4.1 Run `cargo fmt`
- [x] 4.2 Run `cargo clippy`
- [x] 4.3 Add doc comments for public API
