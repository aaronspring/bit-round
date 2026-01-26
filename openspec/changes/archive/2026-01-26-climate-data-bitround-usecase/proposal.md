# Change: Climate Data Bitround Use Case

## Why

Demonstrate a complete workflow for compressing climate data using bitround:
1. Read Zarr-format climate data (e.g., CMIP6 from Google Cloud Storage)
2. Calculate optimal bit precision using keff (entropy-based analysis)
3. Apply bitround compression at the calculated precision
4. Save with zstd compression
5. Report compression ratios

## What Changes

- Add `climate-bitround` CLI tool with `info` and `compress` commands
- Add keff calculation module for entropy-based bit significance analysis
- Use zarrs crate for proper Zarr v3 reading/writing
- Document workflow in README

## Impact

- New binary: `climate-bitround`
- New modules: `src/keff.rs` (calculation), `src/zarr.rs` (utilities)
- Dependencies: zarrs, zstd, reqwest
- Documentation: README updated with usage examples
