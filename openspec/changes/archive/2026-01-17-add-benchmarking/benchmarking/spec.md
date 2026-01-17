## ADDED Requirements

### Requirement: 3D Array Benchmarking
The system SHALL benchmark 3D arrays with edge lengths of 10^n where n ranges from 0 to 3.

#### Scenario: Edge length 10^0 (1x1x1)
- **WHEN** encoding/decoding a 1x1x1 random f32 array with nbits=16
- **THEN** the system SHALL record the timing

#### Scenario: Edge length 10^1 (10x10x10)
- **WHEN** encoding/decoding a 10x10x10 random f32 array with nbits=16
- **THEN** the system SHALL record the timing

#### Scenario: Edge length 10^2 (100x100x100)
- **WHEN** encoding/decoding a 100x100x100 random f32 array with nbits=16
- **THEN** the system SHALL record the timing

#### Scenario: Edge length 10^3 (1000x1000x1000)
- **WHEN** encoding/decoding a 1000x1000x1000 random f32 array with nbits=16
- **THEN** the system SHALL record the timing

### Requirement: Random Input Data
The system SHALL use random input data for all benchmarks.

#### Scenario: Random data generation
- **WHEN** generating benchmark input arrays
- **THEN** values SHALL be drawn from a random uniform distribution
- **AND** a fixed seed SHALL be used for reproducibility

### Requirement: Bitround-Only Timing
The system SHALL only time the bitround encoding/decoding operation itself.

#### Scenario: Isolated timing
- **WHEN** timing benchmarks
- **THEN** the system SHALL exclude data generation time
- **AND** exclude memory allocation/deallocation time
- **AND** exclude data transfer time
- **AND** only measure the bitround encode/decode function call duration

### Requirement: Benchmark Results Reporting
The system SHALL provide a reporting function that generates copy-pasteable benchmark results.

#### Scenario: Copy-pasteable report format
- **WHEN** benchmarks complete
- **THEN** the system SHALL generate a report with machine specifications including:
  - Computer family (e.g., "MacBook M2 16GB")
  - CPU model and cores
  - Total RAM
  - Operating system
- **AND** the report SHALL include timing results in a format suitable for comparison across machines

#### Scenario: Markdown report output
- **WHEN** generating a report
- **THEN** the system SHALL output results in markdown format
- **AND** results SHALL include array dimensions, nbits, and timing in microseconds
- **AND** the output SHALL be directly copy-pasteable for documentation or comparison
- **AND** results SHALL be reported in the order: Python, Julia, Rust

### Requirement: Encoding Throughput Benchmark
The system SHALL benchmark encoding throughput for Rust, Python, and Julia implementations.

#### Scenario: f32 encoding throughput
- **WHEN** encoding a 1MB f32 array with nbits=16
- **THEN** the system SHALL report throughput in MB/s for Rust, Python, and Julia
- **AND** results SHALL be comparable across implementations

#### Scenario: f64 encoding throughput
- **WHEN** encoding a 1MB f64 array with nbits=32
- **THEN** the system SHALL report throughput in MB/s for Rust, Python, and Julia

#### Scenario: Various nbits values
- **WHEN** encoding with nbits values 8, 16, 24 (f32) or 16, 32, 53 (f64)
- **THEN** throughput SHALL be measured for each nbits value

### Requirement: Decoding Throughput Benchmark
The system SHALL benchmark decoding throughput for Rust, Python, and Julia implementations.

#### Scenario: f32 decoding throughput
- **WHEN** decoding a 1MB encoded f32 array with nbits=16
- **THEN** the system SHALL report throughput in MB/s for Rust, Python, and Julia

#### Scenario: f64 decoding throughput
- **WHEN** decoding a 1MB encoded f64 array with nbits=32
- **THEN** the system SHALL report throughput in MB/s for Rust, Python, and Julia

### Requirement: Data Pattern Benchmarking
The system SHALL benchmark with various data patterns to assess algorithm efficiency.

#### Scenario: Zero array encoding
- **WHEN** encoding an array of zeros
- **THEN** the system SHALL measure throughput and compare across implementations

#### Scenario: Climate-like data encoding
- **WHEN** encoding arrays with climate model-like patterns (correlated values)
- **THEN** the system SHALL measure throughput and compare across implementations

### Requirement: Docker-Based Benchmarking
All benchmarks SHALL run in Docker containers for fair comparison.

#### Scenario: Consistent benchmark environment
- **WHEN** running benchmarks via Docker
- **THEN** each implementation SHALL run in its own container
- **AND** containers SHALL have equivalent resource limits

#### Scenario: Warmup iterations
- **WHEN** running benchmarks
- **THEN** each implementation SHALL run warmup iterations before measurement
- **AND** JIT compilation effects SHALL be minimized

#### Scenario: Multiple runs
- **WHEN** running benchmarks
- **THEN** each benchmark SHALL run multiple times (e.g., 5 iterations)
- **AND** median time SHALL be reported to reduce variance

### Requirement: Benchmark Results Comparison
The system SHALL generate comparative results showing Rust speedup.

#### Scenario: Rust vs Python speedup
- **WHEN** benchmarks complete
- **THEN** the system SHALL report Rust/Python speedup ratio
- **AND** report speedup for encoding and decoding separately

#### Scenario: Rust vs Julia speedup
- **WHEN** benchmarks complete
- **THEN** the system SHALL report Rust/Julia speedup ratio
- speedup for encoding **AND** report and decoding separately

#### Scenario: Benchmark report generation
- **WHEN** benchmarks complete
- **THEN** the system SHALL generate a report in JSON format
- **AND** generate an ASCII table summary
- **AND** save results to `benchmark_results/comparison/`

### Requirement: Reproducible Benchmarks
Benchmarks SHALL be reproducible across runs and machines.

#### Scenario: Seeded random data
- **WHEN** generating random test data
- **THEN** a fixed seed SHALL be used for reproducibility

#### Scenario: Version tracking
- **WHEN** running benchmarks
- **THEN** implementation versions SHALL be recorded
- **AND** Docker image tags SHALL be pinned

#### Scenario: Hardware specification
- **WHEN** running benchmarks
- **THEN** the system SHALL record CPU and memory information
- **AND** results SHALL be tagged with hardware metadata
