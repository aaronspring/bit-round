# NetCDF Download Specification

## Purpose
Enable downloading climate data from public NetCDF repositories for processing.

## ADDED Requirements

### Requirement: HTTP Download
The system SHALL download NetCDF files from public climate data repositories via HTTP.

#### Scenario: Download from OpenDAP server
- **WHEN** a valid OpenDAP URL is provided
- **THEN** the system SHALL download the NetCDF data
- **AND** the download progress SHALL be reported
- **AND** incomplete downloads SHALL be resumable

#### Scenario: Download from HTTP endpoint
- **WHEN** a direct NetCDF file URL is provided
- **THEN** the system SHALL download the file
- **AND** validate the file header as NetCDF format

### Requirement: Climate Data Source
The system SHALL support common climate data sources used in research.

#### Scenario: CMIP6 data access
- **WHEN** a CMIP6 dataset identifier is provided
- **THEN** the system SHALL construct the appropriate download URL
- **AND** fetch data from the ESGF node

#### Scenario: ERA5 reanalysis access
- **WHEN** ERA5 data is requested
- **THEN** the system SHALL provide download instructions for CDS API
- **OR** download from available public mirrors

### Requirement: Data Validation
The system SHALL validate downloaded data integrity.

#### Scenario: Checksum verification
- **WHEN** a download completes
- **THEN** the system SHALL verify MD5/SHA256 checksum if provided
- **AND** report validation failure if checksum mismatches

#### Scenario: Format validation
- **WHEN** download completes
- **THEN** the system SHALL verify the file is valid NetCDF
- **AND** reject files with malformed headers

## Cross-References
- Related to: [zarr-storage](../zarr-storage/spec.md) - Downloaded data is converted to Zarr
