# Bitround Compression Specification

## Purpose
Apply bitround compression to climate data with keff-based precision selection.

## ADDED Requirements

### Requirement: Keff Calculation
The system SHALL calculate effective bits (keff) needed to preserve information at a given significance level.

#### Scenario: Calculate keff from data
- **WHEN** keff calculation is requested with a significance level (e.g., 0.99)
- **THEN** the system SHALL analyze mantissa bit entropy from MSB to LSB
- **AND** return the minimum nbits that preserve the target fraction of information
- **AND** clamp result to valid range (1-23 for f32, 1-52 for f64)

### Requirement: Bitround Application
The system SHALL apply bitround compression at specified or calculated precision.

#### Scenario: Apply bitround with calculated keff
- **WHEN** input data is provided without explicit nbits
- **THEN** the system SHALL calculate keff and apply bitround at that precision

#### Scenario: Apply bitround with explicit nbits
- **WHEN** nbits is explicitly provided via CLI flag
- **THEN** the system SHALL skip keff calculation and use provided value

### Requirement: CLI Interface
The system SHALL provide a command-line interface for compression workflow.

#### Scenario: Compress command
- **WHEN** user runs `climate-bitround compress -i <input> -o <output>`
- **THEN** the system SHALL read input Zarr, apply bitround + zstd, and write output
- **AND** report original size, compressed size, and compression ratio

#### Scenario: Info command
- **WHEN** user runs `climate-bitround info <path>`
- **THEN** the system SHALL display arrays, shapes, dtypes, and sizes
