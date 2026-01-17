## 1. Docker Rust Benchmark Image
- [ ] 1.1 Create `docker/Dockerfile.rust-bench` for Rust benchmarking
- [ ] 1.2 Build Rust release binary in Docker
- [ ] 1.3 Include benchmark data generation in Rust image

## 2. Benchmark Data Preparation
- [ ] 2.1 Generate test arrays of various sizes (1KB, 10KB, 100KB, 1MB, 10MB)
- [ ] 2.2 Generate test arrays with different data patterns (zeros, random, climate-like)
- [ ] 2.3 Save benchmark input data to `testdata/benchmarks/`

## 3. Python Benchmarking via Docker
- [ ] 3.1 Create Python benchmark script for encoding throughput
- [ ] 3.2 Create Python benchmark script for decoding throughput
- [ ] 3.3 Run benchmarks via Docker with consistent warmup
- [ ] 3.4 Save results to `benchmark_results/python/`

## 4. Julia Benchmarking via Docker
- [ ] 4.1 Create Julia benchmark script for encoding throughput
- [ ] 4.2 Create Julia benchmark script for decoding throughput
- [ ] 4.3 Run benchmarks via Docker with consistent warmup
- [ ] 4.4 Save results to `benchmark_results/julia/`

## 5. Rust Benchmarking via Docker
- [ ] 5.1 Add Rust benchmark suite using criterion or custom timing
- [ ] 5.2 Benchmark encoding throughput for f32/f64
- [ ] 5.3 Benchmark decoding throughput for f32/f64
- [ ] 5.4 Run Rust benchmarks in Docker for consistency
- [ ] 5.5 Save results to `benchmark_results/rust/`

## 6. Benchmark Runner Scripts
- [ ] 6.1 Create `scripts/bench_all.sh` to run all benchmarks via Docker
- [ ] 6.2 Create `scripts/bench_python.sh` for Python benchmarks
- [ ] 6.3 Create `scripts/bench_julia.sh` for Julia benchmarks
- [ ] 6.4 Create `scripts/bench_rust.sh` for Rust benchmarks
- [ ] 6.5 Add warmup iterations to reduce JIT/cache effects

## 7. Results Analysis
- [ ] 7.1 Create benchmark comparison script
- [ ] 7.2 Calculate speedup ratios (Rust/Python, Rust/Julia)
- [ ] 7.3 Generate ASCII/JSON reports
- [ ] 7.4 Document results in `BENCHMARK_RESULTS.md`
