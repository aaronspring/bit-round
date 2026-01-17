#!/usr/bin/env julia
"""
Generate reference bitround outputs for verification testing.

This script runs the Julia bitround.jl implementation and saves
the encoded outputs as binary files for comparison with Rust implementation.
"""

using LibGit2
using Random

const INPUT_DIR = "/data/inputs"
const OUTPUT_DIR = "/data/outputs"

const NBITS_F32 = [1, 8, 16, 24]
const NBITS_F64 = [1, 16, 32, 53]

function save_binary(data::Vector{T}, filepath::String) where T
    open(filepath, "w") do io
        write(io, data)
    end
end

function load_input(filepath::String)
    name = basename(filepath)

    if endswith(name, "_f32.bin")
        data = Vector{Float32}(undef, filesize(filepath) ÷ sizeof(Float32))
        open(filepath, "r") do io
            read!(io, data)
        end
        return data
    elseif endswith(name, "_f64.bin")
        data = Vector{Float64}(undef, filesize(filepath) ÷ sizeof(Float64))
        open(filepath, "r") do io
            read!(io, data)
        end
        return data
    else
        error("Unknown file type: $(name)")
    end
end

"""
Julia bitround implementation based on bitround.jl
"""
function bitround(x::Float32, nbits::Int)
    shift = 23 - nbits
    keepmask = (0x007fffff >> nbits) << shift

    ui = reinterpret(UInt32, x)
    ui_new = ui & keepmask

    return reinterpret(Float32, ui_new)
end

function bitround(x::Float64, nbits::Int)
    shift = 52 - nbits
    keepmask = (0x007fffffffffffff >> nbits) << shift

    ui = reinterpret(UInt64, x)
    ui_new = ui & keepmask

    return reinterpret(Float64, ui_new)
end

function bitround_array(x::Vector{Float32}, nbits::Int)
    return bitround.(x, nbits)
end

function bitround_array(x::Vector{Float64}, nbits::Int)
    return bitround.(x, nbits)
end

function run_julia_verification()
    mkpath(OUTPUT_DIR)

    input_files = filter(f -> endswith(f, ".bin"), readdir(INPUT_DIR))

    println("Julia bitround reference output generation")
    println("=" * 60)

    for filename in sort(input_files)
        input_path = joinpath(INPUT_DIR, filename)
        data = load_input(input_path)

        if eltype(data) == Float32
            nbits_values = NBITS_F32
        else
            nbits_values = NBITS_F64
        end

        for nbits in nbits_values
            encoded = bitround_array(data, nbits)

            if eltype(data) == Float32
                encoded_uint = reinterpret.(UInt32, encoded)
            else
                encoded_uint = reinterpret.(UInt64, encoded)
            end

            base_name = replace(filename, ".bin" => "")
            output_path = joinpath(OUTPUT_DIR, "$(base_name)_nbits$(nbits).bin")
            save_binary(encoded_uint, output_path)
            println("Saved: $(output_path)")
        end
    end

    println("=" * 60)
    println("Reference outputs saved to $(OUTPUT_DIR)")
end

if abspath(PROGRAM_FILE) == @__FILE__
    run_julia_verification()
end
