# bitround Specification

## Purpose
TBD - created by archiving change add-bitround-encoder. Update Purpose after archive.
## Requirements
### Requirement: Bitround Encoding
The system SHALL provide a bitround encoder that reduces the bit width of floating-point arrays.

#### Scenario: Encode f32 array with 16 bits
- **WHEN** an f32 array is encoded with nbits=16
- **THEN** the output SHALL use exactly 16 bits per value
- **AND** decoding SHALL recover values within numerical tolerance

#### Scenario: Encode f64 array with 32 bits
- **WHEN** an f64 array is encoded with nbits=32
- **THEN** the output SHALL use exactly 32 bits per value
- **AND** decoding SHALL recover values within numerical tolerance

### Requirement: Bitround Decoding
The system SHALL provide a bitround decoder that restores original precision.

#### Scenario: Roundtrip f32 preserves values
- **WHEN** an f32 array is encoded then decoded
- **THEN** the decoded values SHALL match the original within 2^-(nbits-1) relative error
- **AND** the output array SHALL have the same shape as the input

#### Scenario: Roundtrip f64 preserves values
- **WHEN** an f64 array is encoded then decoded
- **THEN** the decoded values SHALL match the original within 2^-(nbits-1) relative error
- **AND** the output array SHALL have the same shape as the input

### Requirement: Configurable Bit Width
The encoder SHALL accept a configurable `nbits` parameter controlling output precision.

#### Scenario: Reject invalid nbits for f32
- **WHEN** nbits is greater than 24 for f32 input
- **THEN** the encoder SHALL return an error

#### Scenario: Reject invalid nbits for f64
- **WHEN** nbits is greater than 53 for f64 input
- **THEN** the encoder SHALL return an error

#### Scenario: Accept valid nbits range
- **WHEN** nbits is between 1 and 24 (f32) or 1 and 53 (f64)
- **THEN** the encoder SHALL accept the value and produce output

### Requirement: Cross-Implementation Verification
The Rust implementation SHALL produce identical output to reference implementations.

#### Scenario: Match Python numcodecs output
- **WHEN** the same input array and nbits are used
- **THEN** the Rust encoder SHALL produce bit-identical output to Python numcodecs bitround

#### Scenario: Match Julia bitround.jl output
- **WHEN** the same input array and nbits are used
- **THEN** the Rust encoder SHALL produce bit-identical output to Julia bitround.jl

### Requirement: Performance Comparison
The system SHALL provide benchmarks comparing Rust performance to Python and Julia implementations.

#### Scenario: Benchmark against Python numcodecs
- **WHEN** running the benchmark suite
- **THEN** the results SHALL report Rust vs Python encode/decode throughput

#### Scenario: Benchmark against Julia bitround.jl
- **WHEN** running the benchmark suite
- **THEN** the results SHALL report Rust vs Julia encode/decode throughput

