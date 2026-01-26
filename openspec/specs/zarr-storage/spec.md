# zarr-storage Specification

## Purpose
TBD - created by archiving change climate-data-bitround-usecase. Update Purpose after archive.
## Requirements
### Requirement: Zarr Read
The system SHALL read multidimensional arrays from local Zarr stores.

#### Scenario: Open local Zarr directory
- **WHEN** a path to a Zarr directory is provided
- **THEN** the system SHALL open the Zarr store using zarrs crate
- **AND** detect array shape, dtype, and chunk configuration
- **AND** decode compressed chunks automatically

### Requirement: Zarr Write
The system SHALL write multidimensional arrays to Zarr stores with compression.

#### Scenario: Write with zstd compression
- **WHEN** an array is written with compression
- **THEN** the system SHALL apply zstd codec at configurable level (1-22)
- **AND** write Zarr v3 metadata
- **AND** store chunks in compressed form

### Requirement: Size Reporting
The system SHALL report directory sizes for compression ratio calculations.

#### Scenario: Calculate directory size
- **WHEN** a Zarr store path is provided
- **THEN** the system SHALL recursively calculate total size on disk
- **AND** format sizes in human-readable units (KB, MB, GB)

