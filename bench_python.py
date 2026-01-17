#!/usr/bin/env python3
"""
Benchmark bitround 11 for 1000x1000 Float32 array.
"""

import numpy as np
from numcodecs import BitRound
import time


def benchmark_bitround():
    size = 1000
    nbits = 11
    n_warmup = 3
    n_iterations = 10

    print("Python bitround benchmark")
    print("========================")
    print(f"Array size: {size}x{size} = {size * size} Float32 elements")
    print(f"nbits: {nbits}")
    print()

    # Generate random data
    np.random.seed(42)
    data = np.random.rand(size, size).astype(np.float32)
    data_vec = data.flatten()

    # Create codec
    codec = BitRound(keepbits=nbits)

    # Warmup
    print("Warming up...")
    for i in range(n_warmup):
        result = codec.encode(data_vec)

    # Benchmark
    print(f"Running benchmark ({n_iterations} iterations)...")
    times = []
    for i in range(n_iterations):
        start = time.perf_counter()
        result = codec.encode(data_vec)
        end = time.perf_counter()
        t = end - start
        times.append(t)
        print(f"  Iteration {i + 1}: {t * 1000:.4f} ms")

    # Calculate statistics
    mean_time = np.mean(times)
    std_time = np.std(times)
    min_time = np.min(times)
    max_time = np.max(times)

    # Calculate throughput
    data_mb = (size * size * 4) / (1024 * 1024)  # 4 bytes per float32
    throughput_mb_s = data_mb / mean_time

    print("\nResults:")
    print(f"  Mean:   {mean_time * 1000:.4f} ms ({throughput_mb_s:.2f} MB/s)")
    print(f"  Std:    {std_time * 1000:.4f} ms")
    print(f"  Min:    {min_time * 1000:.4f} ms")
    print(f"  Max:    {max_time * 1000:.4f} ms")
    print()

    # Return mean time for comparison
    return mean_time * 1000


if __name__ == "__main__":
    benchmark_bitround()
