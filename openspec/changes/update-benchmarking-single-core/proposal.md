# Change: Single-Core Benchmarking Specification

## Why
The existing benchmarking spec does not properly isolate single-core performance, which is critical for fairly comparing Rust and Julia. Julia's multicore capabilities and different memory management models can obscure true single-core performance comparisons. Additionally, the spec lacks guidance on handling memory allocation and in-place operations fairly.

## What Changes
- Update benchmarking spec to require single-core execution with proper thread pinning
- Add research task investigating Matthew Rocklin's biased benchmarks principles
- Add requirements for fair memory allocation comparison (,allocation tracking in-place vs. copy semantics)
- Add requirements for CPU frequency scaling mitigation
- Document multicore vs. single-core trade-offs in benchmark methodology

## Impact
- Affected specs: `benchmarking`
- Affected code: Benchmark scripts for Rust, Python, Julia implementations
- Documentation: BENCHMARK_SETUP.md needs update
