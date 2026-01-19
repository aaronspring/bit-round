# Benchmark Research Findings

## 1. Matthew Rocklin's Biased Benchmarks Analysis

Source: https://matthewrocklin.com/blog/work/2017/03/09/biased-benchmarks

### Key Bias Categories Relevant to Rust vs Julia Comparison

#### 1.1 Skewed Objectives
- Each language/project is developed with different applications in mind
- Julia optimized for scientific computing with JIT
- Rust optimized for systems programming with zero-cost abstractions
- **Mitigation**: Define specific, neutral benchmarks that both languages handle equivalently

#### 1.2 Skewed Experience
- Authors are more adept at their own language's optimization patterns
- Julia: understanding @inbounds, @simd, @views
- Rust: understanding ownership, lifetimes, allocator choice
- **Mitigation**: Collaborate with experts from both communities for review

#### 1.3 Preference Towards Strengths
- Natural tendency to explore cases where own project excels
- **Mitigation**: Define benchmark suite upfront, stick to it, document cases where one language struggles

#### 1.4 Tuning During Experimentation
- Optimizing code during benchmarking gives unfair advantage
- **Mitigation**: Freeze implementations before benchmark period begins

#### 1.5 Omission
- No motivation to publish negative results
- **Mitigation**: Commit to publishing all results, even unfavorable ones

## 2. Single-Core Benchmarking Best Practices

Source: https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux

### 2.1 CPU Frequency Scaling Mitigation

**Commands to disable turbo boost:**
```bash
# Intel
echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo

# AMD
echo 0 > /sys/devices/system/cpu/cpufreq/boost
```

**Set scaling governor to performance:**
```bash
for i in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
do
  echo performance > $i
done
```

### 2.2 Thread Pinning

**Using taskset:**
```bash
taskset -c 0 ./benchmark  # Run on CPU 0 only
```

**Using cset shield for better isolation:**
```bash
cset shield -c N1,N2 -k on
cset shield --exec -- ./benchmark
```

### 2.3 Hyper-Threading

**Disable sibling threads:**
```bash
echo 0 > /sys/devices/system/cpu/cpu1/online
echo 0 > /sys/devices/system/cpu/cpu2/online
# etc.
```

### 2.4 Other Mitigations

**Process priority:**
```bash
sudo nice -n -5 taskset -c 1 ./benchmark
```

**Drop file system cache:**
```bash
echo 3 | sudo tee /proc/sys/vm/drop_caches
sync
```

**Disable ASLR per-process:**
```bash
setarch -R ./benchmark
```

## 3. Memory Allocation Fairness

### 3.1 Julia Allocation Tracking

```julia
using BenchmarkTools

# Track allocations
@allocated expression

# Track allocations and GC time
@btime expression

# Full GC before timing to get clean state
GC.gc()
@time @noinline function bench()
    # code
end
```

**Disable GC during timing:**
```julia
GC.disable()
try
    @elapsed code
finally
    GC.enable()
end
```

### 3.2 Rust Allocation Tracking

**Using jemallocator for fair comparison:**
```rust
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
```

**Custom allocator tracking:**
```rust
use std::alloc::System;

#[global_allocator]
static ALLOC: Trallocator = Trallocator::new(System);

fn main() {
    ALLOC.reset();
    // run benchmark
    let bytes = ALLOC.get();
}
```

### 3.3 Allocation Metrics to Track

| Metric | Julia | Rust |
|--------|-------|------|
| Total allocations | `@allocated` | Custom allocator |
| Allocation count | Included | Custom allocator |
| Peak memory | `peak_memory` | `/proc/[pid]/status` |
| GC time | `@btime` GC% | N/A |

## 4. In-Place vs Copy Operation Fairness

### 4.1 Julia Views vs Copies

```julia
# Creates a copy (allocates)
@views sum(x[2:end-1])

# Creates a view (no allocation)
y = view(x, 2:end-1)
sum(y)
```

### 4.2 Benchmarking Both Semantics

**Benchmark in-place operations:**
```julia
function bench_inplace!(a)
    for i in eachindex(a)
        a[i] *= 2
    end
end
```

**Benchmark copy-based operations:**
```julia
function bench_copy(a)
    b = copy(a)
    for i in eachindex(b)
        b[i] *= 2
    end
    b
end
```

### 4.3 Rust Mutable vs Immutable

```rust
// Mutable (potential in-place)
fn bench_inplace(v: &mut [f32]) {
    for x in v.iter_mut() {
        *x *= 2.0;
    }
}

// Immutable (allocates new vector)
fn bench_copy(v: &[f32]) -> Vec<f32> {
    v.iter().map(|x| x * 2.0).collect()
}
```

## 5. Julia GC Handling Strategy

### 5.1 Disable GC During Measurement

```julia
function benchmark_code()
    GC.gc()
    GC.disable()
    
    start = time_ns()
    # code to benchmark
    result = compute()
    
    elapsed = (time_ns() - start) / 1e9
    
    GC.enable()
    return result, elapsed
end
```

### 5.2 Warmup with GC Enabled

```julia
# Warmup (with JIT compilation)
for _ in 1:3
    compute()
end

# Measure (with or without GC depending on goal)
GC.gc()
@btime compute()
```

### 5.3 GC Statistics

```julia
using GC
GC.enable_logging(true)
GC.gc()
# Logs GC timing information
```

## 6. Statistical Methods for Results

### 6.1 Minimum vs Average

From Kevin Moddel's research:
- Benchmark results are right-skewed (frequently see slower results)
- Minimum values are more representative than averages

**Recommended: Report minimum time**

### 6.2 Multiple Runs

```julia
# BenchmarkTools.jl default
@btime compute()  # 10000 samples with 1000 evaluations

# For more stable results
@benchmark compute() samples=10 evals=3
```

### 6.3 Delta Measurement

```julia
# Time N iterations
t1 = @elapsed for _ in 1:N
    compute()
end

# Time 2N iterations
t2 = @elapsed for _ in 1:2N
    compute()
end

# Per-iteration time
(t2 - t1) / N
```

## 7. macOS-Specific Considerations

### 7.1 CPU Affinity on macOS

```bash
# Use taskset equivalent via renice or cgroups
# Not directly available, consider Docker or VMs
```

### 7.2 Turbo Boost Detection

```bash
# Check if turbo is active
sysctl machdep.cpu.brand_string
pmset -g therm
```

## 8. Checklist for Fair Benchmarking

### Environment Setup
- [ ] Disable turbo boost
- [ ] Set CPU frequency governor to performance
- [ ] Pin process to single core
- [ ] Disable hyper-threading if possible
- [ ] Set process priority to high
- [ ] Disable ASLR

### Julia-Specific
- [ ] Warmup runs for JIT compilation
- [ ] Disable GC during timing OR report GC time separately
- [ ] Track allocations with `@allocated`
- [ ] Use `@views` vs `copy` appropriately

### Rust-Specific
- [ ] Use jemalloc or mimalloc for fair comparison
- [ ] Track allocations with custom allocator
- [ ] Ensure release mode with optimizations

### General
- [ ] Run multiple iterations
- [ ] Report minimum, median, and standard deviation
- [ ] Document all environment settings
- [ ] Version all implementations
- [ ] Share raw data for reproducibility

## 9. References

- [Biased Benchmarks - Matthew Rocklin](https://matthewrocklin.com/blog/work/2017/03/09/biased-benchmarks)
- [EasyPerf - Consistent Benchmarking on Linux](https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux)
- [BenchmarkTools.jl Documentation](https://juliaci.github.io/BenchmarkTools.jl/stable/)
- [Criterion.rs - Rust Benchmarking](https://bheisler.github.io/criterion.rs/book/)
- [LLVM Benchmarking Guide](https://llvm.org/docs/Benchmarking.html)
- [SPEC CPU2017 Tuning Guide](https://www.spec.org/cpu2017/flags/)
