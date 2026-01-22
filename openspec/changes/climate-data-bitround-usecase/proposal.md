# Change: Climate Data Bitround Use Case

## Why

Climate data typically arrives in NetCDF format but is often stored for analysis in Zarr format for efficient chunked access. This change demonstrates a complete real-world workflow that:

1. Downloads actual climate data from a public repository: 1.6GB `gs://cmip6/CMIP6/ScenarioMIP/NOAA-GFDL/GFDL-ESM4/ssp585/r1i1p1f1/Omon/zos/gn/v20180701/`
3. Applies keff bit analysis to determine optimal compression levels for 99% preserving bitinfo
4. Uses bitround compression at the calculated precision level
5. Saves compressed data with Zarr compression codecs
6. Compares storage sizes before and after

This use case serves as both an integration test and documentation of the bitround workflow for climate data applications.

## What Changes

- Add zarr-storage capability for Zarr format handling (read/write)
- Add keff-calculation capability for determining information-preserving bit depths
- Add bitround-compression capability with compression ratio reporting
- Create CLI tool demonstrating the complete workflow
- Add size comparison output at each processing stage

## Impact

- New dependencies: netcdf, zarrs (with compression), http download
- New binary: `climate-bitround` demonstrating the workflow
- New library modules for data download and keff calculation
- Documentation in README.md for the use case

## Permissions

```json
{
  "skill": {
    "*": "allow",
    "pr-review": "allow",
    "internal-*": "deny",
    "experimental-*": "ask"
  }
}
```
