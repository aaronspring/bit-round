#!/usr/bin/env julia
"""
Generate reference bitround outputs for Julia.
Reads input data from testdata/inputs/ and writes encoded outputs to testdata/julia/
"""

using Random
using Printf
using Libdl

TESTDATA_DIR = joinpath(dirname(@__FILE__), "..", "testdata")
INPUTS_DIR = joinpath(TESTDATA_DIR, "inputs")
OUTPUT_DIR = joinpath(TESTDATA_DIR, "julia")

INPUT_FILES_F32 = [
    "zeros_f32.bin",
    "constants_f32.bin",
    "random_f32.bin",
    "edge_f32.bin",
]

INPUT_FILES_F64 = [
    "zeros_f64.bin",
    "constants_f64.bin",
    "random_f64.bin",
    "edge_f64.bin",
]

NBITS_F32 = [1, 8, 16, 23]
NBITS_F64 = [1, 16, 32, 52]

function bitround_ieee_encode(x::Float32, nbits::Int)
    mantissa_bits = 23
    if nbits >= mantissa_bits
        return reinterpret(UInt32, x)
    end

    shift = mantissa_bits - nbits
    keepmask = UInt32(0x007fffff) << shift

    ui = reinterpret(UInt32, x)
    ulp_half = UInt32(1) << (shift - 1)
    tie_bit = (ui >> shift) & UInt32(1)

    ui_new = (ui + ulp_half + tie_bit) & keepmask

    return ui_new
end

function bitround_array_f32(x::Vector{Float32}, nbits::Int)
    return bitround_ieee_encode.(x, nbits)
end

function bitround_ieee_encode(x::Float64, nbits::Int)
    mantissa_bits = 52
    if nbits >= mantissa_bits
        return reinterpret(UInt64, x)
    end

    shift = mantissa_bits - nbits
    keepmask = UInt64(0x007fffffffffffff) << shift

    ui = reinterpret(UInt64, x)
    ulp_half = UInt64(1) << (shift - 1)
    tie_bit = (ui >> shift) & UInt64(1)

    ui_new = (ui + ulp_half + tie_bit) & keepmask

    return ui_new
end

function bitround_array_f64(x::Vector{Float64}, nbits::Int)
    return bitround_ieee_encode.(x, nbits)
end

function load_f32_from_binary(filepath::String)::Vector{Float32}
    open(filepath, "r") do io
        data = read(io)
        num_floats = length(data) ÷ 4
        reinterpret(Float32, data)[1:num_floats]
    end
end

function load_f64_from_binary(filepath::String)::Vector{Float64}
    open(filepath, "r") do io
        data = read(io)
        num_floats = length(data) ÷ 8
        reinterpret(Float64, data)[1:num_floats]
    end
end

function save_u32_to_binary(data::Vector{UInt32}, filepath::String)
    open(filepath, "w") do io
        write(io, reinterpret(UInt8, data))
    end
end

function save_u64_to_binary(data::Vector{UInt64}, filepath::String)
    open(filepath, "w") do io
        write(io, reinterpret(UInt8, data))
    end
end

function process_f32(input_file::String, nbits::Int)
    input_path = joinpath(INPUTS_DIR, input_file)
    output_file = replace(input_file, ".bin" => "_nbits$(nbits).bin")
    output_path = joinpath(OUTPUT_DIR, output_file)

    data = load_f32_from_binary(input_path)
    encoded = bitround_array_f32(data, nbits)

    save_u32_to_binary(encoded, output_path)
    println("  Created $output_file")
end

function process_f64(input_file::String, nbits::Int)
    input_path = joinpath(INPUTS_DIR, input_file)
    output_file = replace(input_file, ".bin" => "_nbits$(nbits).bin")
    output_path = joinpath(OUTPUT_DIR, output_file)

    data = load_f64_from_binary(input_path)
    encoded = bitround_array_f64(data, nbits)

    save_u64_to_binary(encoded, output_path)
    println("  Created $output_file")
end

function main()
    println("=== Generating Julia Reference Data ===")
    println()

    mkpath(OUTPUT_DIR)

    println("Processing f32 inputs...")
    for input_file in INPUT_FILES_F32
        for nbits in NBITS_F32
            process_f32(input_file, nbits)
        end
    end

    println()
    println("Processing f64 inputs...")
    for input_file in INPUT_FILES_F64
        for nbits in NBITS_F64
            process_f64(input_file, nbits)
        end
    end

    println()
    println("Reference outputs saved to $OUTPUT_DIR/")
end

main()
