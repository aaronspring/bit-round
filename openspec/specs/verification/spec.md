# verification Specification

## Purpose
Verify that the Rust bitround implementation produces bit-identical output to Python numcodecs and Julia bitround.jl reference implementations using locally-installed tools.

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

### Requirement: Local Reference Implementation Verification
Reference implementations SHALL run using locally installed Python and Julia to generate verification data.

#### Scenario: Python verification via local installation
- **WHEN** running `scripts/verify_python.sh`
- **THEN** Python numcodecs bitround outputs SHALL be generated and saved to testdata/python/
- **AND** Python with numcodecs SHALL be installed via pip

#### Scenario: Julia verification via local installation
- **WHEN** running `scripts/verify_julia.sh`
- **THEN** Julia bitround.jl outputs SHALL be generated and saved to testdata/julia/
- **AND** Julia with required packages SHALL be available locally

#### Scenario: Prerequisites for verification
- **WHEN** running verification scripts
- **THEN** Python 3 with numpy and numcodecs SHALL be available
- **AND** Julia with LibGit2 and standard libraries SHALL be available
- **OR** scripts SHALL fail with clear error messages indicating missing dependencies

