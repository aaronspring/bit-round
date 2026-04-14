#!/usr/bin/env julia
# One-shot bitround for a single 3D array. Prints `encode_seconds=<float>`.

using Random
using BitInformation

function bitround_array!(result::Vector{UInt32}, x::Vector{Float32}, keepbits::Int)
    @inbounds for i in eachindex(x)
        result[i] = reinterpret(UInt32, round(x[i], keepbits))
    end
    return result
end

function main()
    edge = parse(Int, ARGS[1])
    keepbits = parse(Int, ARGS[2])

    Random.seed!(42)
    data = (273.0f0 .+ rand(Float32, edge, edge, edge) .* 20.0f0)
    flat = vec(data)
    out = Vector{UInt32}(undef, length(flat))

    bitround_array!(out, flat, keepbits)  # warmup / JIT
    t0 = time_ns()
    bitround_array!(out, flat, keepbits)
    t1 = time_ns()
    println("encode_seconds=", (t1 - t0) / 1e9)
    println("out_bytes=", sizeof(out))
end

main()
