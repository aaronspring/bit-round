## 1. Research Phase

### Task 1.1: Research Matthew Rocklin's Biased Benchmarks
- [x] 1.1.1 Fetch and read https://matthewrocklin.com/blog/work/2017/03/09/biased-benchmarks
- [x] 1.1.2 Document key bias categories relevant to Rust vs Julia comparison
- [x] 1.1.3 Create checklist of bias mitigation strategies

### Task 1.2: Research Single-Core Benchmarking Best Practices
- [x] 1.2.1 Research CPU frequency scaling (turbo boost) impact on benchmarks
- [x] 1.2.2 Research thread pinning techniques for fair comparison
- [x] 1.2.3 Research NUMA effects on memory access patterns

### Task 1.3: Research Memory Allocation Fairness
- [x] 1.3.1 Research how to fairly compare Julia's GC vs Rust's manual memory management
- [x] 1.3.2 Document allocation tracking methods for each language
- [x] 1.3.3 Research in-place operation benchmarking (mutating vs. non-mutating)

## 2. Specification Updates

### Task 2.1: Update Single-Core Requirements
- [ ] 2.1.1 Add requirement for CPU isolation and thread pinning
- [ ] 2.1.2 Add requirement for CPU frequency governor configuration
- [ ] 2.1.3 Add requirement for warmup iterations to account for JIT compilation

### Task 2.2: Add Memory Allocation Requirements
- [ ] 2.2.1 Add requirement for allocation counting methodology
- [ ] 2.2.2 Add requirement for distinguishing heap vs. stack allocations
- [ ] 2.2.3 Add requirement for tracking peak memory usage

### Task 2.3: Add In-Place vs. Copy Operation Requirements
- [ ] 2.3.1 Add requirement for documenting mutability semantics
- [ ] 2.3.2 Add requirement for benchmarking both mutable and immutable operations
- [ ] 2.3.3 Add requirement for fair comparison of array copy costs

## 3. Documentation Updates

### Task 3.1: Update BENCHMARK_SETUP.md
- [ ] 3.1.1 Document single-core benchmark commands
- [ ] 3.1.2 Document environment setup for fair comparison
- [ ] 3.1.3 Add troubleshooting section for common issues

### Task 3.2: Create Benchmark Methodology Documentation
- [ ] 3.2.1 Document why single-core matters for language comparison
- [ ] 3.2.2 Document how to interpret results fairly
- [ ] 3.2.3 Add examples of common benchmarking pitfalls
