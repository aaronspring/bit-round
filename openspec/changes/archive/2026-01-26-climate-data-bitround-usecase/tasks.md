## Core Implementation

### Task 1: Infrastructure
- [x] 1.1 Add zarrs dependency with zstd compression support
- [x] 1.2 Add reqwest for HTTP downloads
- [x] 1.3 Create zarr utility module (get_directory_size, format_size)
- [x] 1.4 Create download module with retry logic

### Task 2: Keff Calculation
- [x] 2.1 Implement entropy-based bit significance analysis
- [x] 2.2 Fix bit ordering (MSB→LSB for mantissa bits)
- [x] 2.3 Create keff module with f32/f64 support
- [x] 2.4 Add unit tests for keff calculation

### Task 3: CLI Tool
- [x] 3.1 Create climate-bitround binary
- [x] 3.2 Implement `info` command (show Zarr store info)
- [x] 3.3 Implement `compress` command with options:
  - [x] Input/output paths
  - [x] Significance level (default 0.99)
  - [x] Zstd compression level
  - [x] Optional nbits override
  - [x] Array name selection

### Task 4: Zarr Integration
- [x] 4.1 Use zarrs crate for proper Zarr v3 reading
- [x] 4.2 Use zarrs crate for proper Zarr v3 writing with zstd
- [x] 4.3 Handle f32/f64 data types
- [x] 4.4 Report compression ratios and sizes

## Documentation

### Task 5: README Updates
- [x] 5.1 Document CLI commands and options
- [x] 5.2 Add data download instructions
- [x] 5.3 Run compression on CMIP6 data and document results

## Completed

### Task 6: Test with Real Data
- [x] 6.1 Download CMIP6 zos dataset (882 MB, 17 chunks)
- [x] 6.2 Run compression workflow at 8-23 bits
- [x] 6.3 Document actual compression ratios in README (up to 3.2× at 8 bits)
- [x] 6.4 Verify output can be read by zarr-python v3.1.5

### Task 7: Optional Enhancements (deferred)
- [ ] NetCDF support (not needed - CMIP6 data is Zarr)
- [ ] Download command in CLI
- [ ] ERA5 data source support
