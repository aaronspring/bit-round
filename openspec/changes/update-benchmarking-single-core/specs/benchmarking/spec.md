## MODIFIED Requirements

### Requirement: Single-Core Execution
The system SHALL execute all benchmarks using a single CPU core to enable fair comparison of language runtime performance.

#### Scenario: Thread pinning
- **WHEN** running benchmarks
- **THEN** the system SHALL pin the benchmark process to a single physical CPU core
- **AND** the system SHALL use taskset (Linux) or similar mechanism for core isolation

#### Scenario: CPU frequency scaling mitigation
- **WHEN** running benchmarks
- **THEN** the system SHALL either disable turbo boost or document its state
- **AND** the system SHALL use cpupower or similar to set CPU frequency to a fixed value
- **AND** the system SHALL run benchmarks long enough to reach thermal equilibrium

#### Scenario: Julia JIT warmup
- **WHEN** running Julia benchmarks
- **THEN** the system SHALL execute a warmup run before measurement
- **AND** the warmup SHALL use the same workload as the measured run
- **AND** warmup iterations SHALL be documented (minimum 3 iterations recommended)

### Requirement: Memory Allocation Fairness
The system SHALL track and report memory allocations to enable fair comparison across languages with different memory management models.

#### Scenario: Allocation counting
- **WHEN** running benchmarks
- **THEN** the system SHALL count total allocations during the measured operation
- **AND** the system SHALL count deallocations during the measured operation
- **AND** the system SHALL report net memory change

#### Scenario: Julia GC handling
- **WHEN** running Julia benchmarks
- **THEN** the system SHALL disable GC during timing to isolate compute performance
- **AND** the system SHALL run GC before benchmark to ensure clean state
- **AND** the system SHALL document GC state in results

#### Scenario: Rust allocation tracking
- **WHEN** running Rust benchmarks
- **THEN** the system SHALL use mimalloc or jemalloc for fair comparison with Julia
- **AND** the system SHALL report allocator overhead

### Requirement: In-Place vs Copy Operation Fairness
The system SHALL ensure fair comparison between in-place operations and copy-based operations.

#### Scenario: Operation type documentation
- **WHEN** documenting benchmark operations
- **THEN** the system SHALL clearly state whether operations mutate input data
- **AND** the system SHALL document allocation behavior (heap/stack, temporary allocations)

#### Scenario: Fair comparison baseline
- **WHEN** comparing Rust and Julia implementations
- **THEN** both implementations SHALL use equivalent operation semantics
- **AND** if one language supports in-place operations and the other does not, this SHALL be documented
- **AND** separate benchmarks SHALL measure the cost of array copying

### Requirement: Multicore Disclosure
The system SHALL document and disclose when multicore parallelism is used, to avoid confusion with single-core results.

#### Scenario: Multicore detection
- **WHEN** running benchmarks
- **THEN** the system SHALL verify single-core execution by checking thread count
- **AND** the system SHALL fail with an error if multiple cores are detected during measurement
- **AND** the system SHALL document the verification method in results

#### Scenario: Parallel benchmark separation
- **WHEN** parallel benchmarks are desired
- **THEN** the system SHALL run them as separate benchmarks with distinct names
- **AND** parallel benchmarks SHALL be clearly labeled (e.g., "encoding_parallel")
- **AND** parallel results SHALL NOT be compared directly with single-core results

## ADDED Requirements

### Requirement: Benchmark Environment Reproducibility
The system SHALL ensure benchmark environment is reproducible across runs and machines.

#### Scenario: System state capture
- **WHEN** running benchmarks
- **THEN** the system SHALL capture CPU model, core count, and frequency
- **AND** the system SHALL capture memory size and NUMA configuration
- **AND** the system SHALL capture OS version and kernel

#### Scenario: Core isolation verification
- **WHEN** running benchmarks
- **THEN** the system SHALL verify only one core is used via operating system APIs
- **AND** the system SHALL report any deviation from single-core execution
