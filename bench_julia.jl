#!/usr/bin/env julia
"""
Benchmark bitround 11 for 1000x1000 Float32 array.
"""

using Random
using Printf
using Statistics

function bitround_ieee(x::Float32, nbits::Int)
    mantissa_bits = 23
    if nbits >= mantissa_bits
        return x
    end

    shift = mantissa_bits - nbits
    keepmask = UInt32(0x007fffff) << shift

    ui = reinterpret(UInt32, x)
    ulp_half = UInt32(1) << (shift - 1)
    tie_bit = (ui >> shift) & UInt32(1)

    ui_new = (ui + ulp_half + tie_bit) & keepmask

    return reinterpret(Float32, ui_new)
end

function bitround_array(x::Vector{Float32}, nbits::Int)
    return bitround_ieee.(x, nbits)
end

# Create 1000x1000 array of random Float32
const size = 1000
const nbits = 11
const n_warmup = 10
const n_iterations = 20

println("Julia bitround benchmark")
println("========================")
println("Array size: $(size)x$(size) = $(size*size) Float32 elements")
println("nbits: $nbits")
println()

# Generate random data
Random.seed!(42)
data = rand(Float32, size, size)
data_vec = vec(data)

# Warmup
println("Warming up...")
for i in 1:n_warmup
    result = bitround_array(data_vec, nbits)
end

# Benchmark
println("Running benchmark ($n_iterations iterations)...")
times = Float64[]
for i in 1:n_iterations
    # Force recompilation timing
    result = bitround_array(data_vec, nbits)
    
    t = @elapsed begin
        result = bitround_array(data_vec, nbits)
    end
    push!(times, t)
    @printf("  Iteration %d: %.4f ms\n", i, t * 1000)
end

# Calculate statistics
mean_time = mean(times)
std_time = std(times)
min_time = minimum(times)
max_time = maximum(times)

# Calculate throughput
data_mb = (size * size * sizeof(Float32)) / (1024 * 1024)
throughput_mb_s = data_mb / mean_time

@printf("\nResults:\n")
@printf("  Mean:   %.4f ms (%.2f MB/s)\n", mean_time * 1000, throughput_mb_s)
@printf("  Std:    %.4f ms\n", std_time * 1000)
@printf("  Min:    %.4f ms\n", min_time * 1000)
@printf("  Max:    %.4f ms\n", max_time * 1000)
@printf("\n")

# Return mean time for comparison
println("Mean time (ms): ", mean_time * 1000)
