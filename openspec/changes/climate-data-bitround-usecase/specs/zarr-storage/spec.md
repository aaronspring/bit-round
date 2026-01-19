# Zarr Storage Specification

## Purpose
Enable reading and writing climate data in Zarr format with optional compression.

## ADDED Requirements

### Requirement: Zarr Read
The system SHALL read multidimensional arrays from Zarr stores.

#### Scenario: Open local Zarr directory
- **WHEN** a path to a Zarr directory is provided
- **THEN** the system SHALL open the Zarr store
- **AND** load metadata for all arrays
- **AND** provide access to array data via slice operations

#### Scenario: Open remote Zarr store
- **WHEN** an HTTP URL to a Zarr store is provided
- **THEN** the system SHALL connect to the store
- **AND** lazy-load array chunks on demand

### Requirement: Zarr Write
The system SHALL write multidimensional arrays to Zarr stores.

#### Scenario: Write with compression
- **WHEN** an array is written with compression parameters
- **THEN** the system SHALL apply the specified codec
- **AND** store chunks in compressed form
- **AND** write correct metadata for codec configuration

#### Scenario: Write with bitround codec
- **WHEN** bitround compression is applied
- **THEN** the system SHALL store nbits parameter in codec metadata
- **AND** use zstd or blosc for container compression

### Requirement: NetCDF to Zarr Conversion
The system SHALL convert NetCDF data to Zarr format.

#### Scenario: Convert NetCDF variable to Zarr array
- **WHEN** a NetCDF variable is selected
- **THEN** the system SHALL read the data with correct dtype
- **AND** preserve dimension ordering
- **AND** write to Zarr with chunk sizes appropriate for access patterns

### Requirement: Compression Configuration
The system SHALL support configurable compression settings.

#### Scenario: Configure zstd compression level
- **WHEN** writing Zarr with zstd codec
- **THEN** the compression level SHALL be configurable (1-22)
- **AND** default to level 10 for balanced speed/ratio

#### Scenario: Configure chunk size
- **WHEN** writing Zarr arrays
- **THEN** chunk dimensions SHALL be configurable
- **AND** default to chunks matching typical access patterns
