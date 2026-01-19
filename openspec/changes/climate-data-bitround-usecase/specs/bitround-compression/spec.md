# Bitround Compression Specification

## Purpose
Apply bitround compression to climate data with keff-based precision selection and report compression ratios.

## ADDED Requirements

### Requirement: Keff Calculation
The system SHALL calculate the effective number of bits (keff) needed to preserve information at a given significance level.

#### Scenario: Calculate keff at 0.99 significance level
- **WHEN** keff calculation is requested at 0.99 inf level
- **THEN** the system SHALL analyze the bit significance distribution
- **AND** return the minimum nbits that preserve 99% of information
- **AND** report keff value for the input array

#### Scenario: Calculate keff for floating point data
- **WHEN** an f32 or f64 array is provided
- **THEN** the system SHALL compute keff using the entropy-based method
- **AND** clamp result to valid range (1-24 for f32, 1-53 for f64)

### Requirement: Bitround Application
The system SHALL apply bitround compression at specified or calculated precision.

#### Scenario: Apply bitround with calculated keff
- **WHEN** keff is calculated from input data
- **THEN** the system SHALL apply bitround at that nbits value
- **AND** return both original and compressed arrays

#### Scenario: Apply bitround with specified nbits
- **WHEN** nbits is explicitly provided
- **THEN** the system SHALL apply bitround at that precision
- **AND** reject nbits exceeding the native type precision

### Requirement: Size Comparison
The system SHALL report storage sizes at each processing stage.

#### Scenario: Report uncompressed size
- **WHEN** processing begins
- **THEN** the system SHALL calculate and report raw data size in bytes
- **AND** report size per element (bits/bytes per value)

#### Scenario: Report bitround compressed size
- **WHEN** bitround compression is applied
- **THEN** the system SHALL calculate theoretical size reduction
- **AND** report compression ratio (original/compressed)

#### Scenario: Report final compressed size
- **WHEN** Zarr write completes with compression
- **THEN** the system SHALL report final file size on disk
- **AND** report overall compression ratio from original

### Requirement: Compression Quality
The system SHALL preserve numerical accuracy within bitround specifications.

#### Scenario: Roundtrip preserves information
- **WHEN** data is bitround compressed then decompressed
- **THEN** the decoded values SHALL match within 2^-(nbits-1) relative error
- **AND** NaN and Inf values SHALL be preserved

## Cross-References
- Related to: [netcdf-download](../netcdf-download/spec.md) - Input data source
- Related to: [zarr-storage](../zarr-storage/spec.md) - Output format with compression
