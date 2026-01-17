# verification Specification

## Purpose
TBD - created by archiving change add-python-julia-verification. Update Purpose after archive.
## Requirements
### Requirement: Python numcodecs Verification
The Rust implementation SHALL produce bit-identical output to Python numcodecs bitround.

#### Scenario: f32 encoding matches Python
- **WHEN** the same f32 input array is encoded with nbits=16
- **THEN** the Rust output SHALL be bit-identical to Python numcodecs bitround output

#### Scenario: f64 encoding matches Python
- **WHEN** the same f64 input array is encoded with nbits=32
- **THEN** the Rust output SHALL be bit-identical to Python numcodecs bitround output

#### Scenario: f32 decoding matches Python
- **WHEN** the same encoded f32 array is decoded
- **THEN** the Rust output SHALL be bit-identical to Python numcodecs bitround output

#### Scenario: f64 decoding matches Python
- **WHEN** the same encoded f64 array is decoded
- **THEN** the Rust output SHALL be bit-identical to Python numcodecs bitround output

### Requirement: Julia bitround.jl Verification
The Rust implementation SHALL produce bit-identical output to Julia bitround.jl.

#### Scenario: f32 encoding matches Julia
- **WHEN** the same f32 input array is encoded with nbits=16
- **THEN** the Rust output SHALL be bit-identical to Julia bitround.jl output

#### Scenario: f64 encoding matches Julia
- **WHEN** the same f64 input array is encoded with nbits=32
- **THEN** the Rust output SHALL be bit-identical to Julia bitround.jl output

#### Scenario: f32 decoding matches Julia
- **WHEN** the same encoded f32 array is decoded
- **THEN** the Rust output SHALL be bit-identical to Julia bitround.jl output

#### Scenario: f64 decoding matches Julia
- **WHEN** the same encoded f64 array is decoded
- **THEN** the Rust output SHALL be bit-identical to Julia bitround.jl output

### Requirement: Edge Case Handling
The Rust implementation SHALL handle edge cases identically to reference implementations.

#### Scenario: Zero values
- **WHEN** encoding an array of zeros
- **THEN** the output SHALL match reference implementations

#### Scenario: Special values (NaN, Inf, -Inf)
- **WHEN** encoding arrays containing NaN, Inf, or -Inf
- **THEN** the output SHALL match reference implementations

#### Scenario: Subnormal numbers
- **WHEN** encoding subnormal floating-point numbers
- **THEN** the output SHALL match reference implementations

### Requirement: Verification Test Coverage
The verification tests SHALL cover a range of nbits values and data patterns.

#### Scenario: All valid nbits values for f32
- **WHEN** testing with nbits values 1, 8, 16, 24
- **THEN** all tests SHALL pass with bit-identical output

#### Scenario: All valid nbits values for f64
- **WHEN** testing with nbits values 1, 16, 32, 53
- **THEN** all tests SHALL pass with bit-identical output

#### Scenario: Various data patterns
- **WHEN** testing with zeros, constants, random values, and edge values
- **THEN** all tests SHALL pass with bit-identical output

### Requirement: Docker-Based Verification
Reference implementations SHALL run in Docker containers to ensure reproducibility and eliminate local environment dependency.

#### Scenario: Python verification via Docker
- **WHEN** running `docker run --rm -v $(pwd)/testdata:/data python-verification`
- **THEN** Python numcodecs bitround outputs SHALL be generated and saved to testdata/
- **AND** Docker image SHALL contain Python with numcodecs installed

#### Scenario: Julia verification via Docker
- **WHEN** running `docker run --rm -v $(pwd)/testdata:/data julia-verification`
- **THEN** Julia bitround.jl outputs SHALL be generated and saved to testdata/
- **AND** Docker image SHALL contain Julia with bitround.jl installed

#### Scenario: Reproducible verification
- **WHEN** running Docker verification on any machine
- **THEN** outputs SHALL be identical to previously generated reference outputs
- **AND** Docker image versions SHALL be pinned for reproducibility

### Requirement: Reference Test Data
Verification SHALL use pre-generated reference test data committed to the repository.

#### Scenario: Reference data for Python comparison
- **WHEN** Rust verification tests run
- **THEN** they SHALL compare against reference outputs in `testdata/python/`
- **AND** reference outputs SHALL be generated via Docker Python verification

#### Scenario: Reference data for Julia comparison
- **WHEN** Rust verification tests run
- **THEN** they SHALL compare against reference outputs in `testdata/julia/`
- **AND** reference outputs SHALL be generated via Docker Julia verification

#### Scenario: Test data formats
- **WHEN** generating reference data
- **THEN** outputs SHALL be saved as binary files (.bin) with metadata
- **AND** input arrays SHALL be saved alongside expected outputs

