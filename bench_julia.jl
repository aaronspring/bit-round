#!/usr/bin/env julia
"""
Benchmark bitround for 3D arrays with edge lengths 10^n where n=0 to 3.
Tests encoding and decoding with random input data.
Uses BitInformation.jl for correct IEEE round-to-nearest-ties-to-even.
"""

using Random
using Printf
using Statistics
using JSON
using BitInformation

function bitround_array!(result::Vector{UInt32}, x::Vector{Float32}, keepbits::Int)
    @inbounds for i in eachindex(x)
        result[i] = reinterpret(UInt32, round(x[i], keepbits))
    end
    return result
end

function bitround_array(x::Vector{Float32}, keepbits::Int)
    return bitround_array!(Vector{UInt32}(undef, length(x)), x, keepbits)
end

function decode_bitround_array!(result::Vector{Float32}, x::Vector{UInt32}, keepbits::Int)
    @inbounds for i in eachindex(x)
        result[i] = reinterpret(Float32, x[i])
    end
    return result
end

function decode_bitround_array(x::Vector{UInt32}, keepbits::Int)
    return decode_bitround_array!(Vector{Float32}(undef, length(x)), x, keepbits)
end

function get_machine_specs()
    specs = Dict{String, String}(
        "computer_family" => "Unknown",
        "cpu_model" => "Unknown",
        "cpu_cores" => "Unknown",
        "ram_gb" => "Unknown",
        "os" => string(Sys.KERNEL, " ", Sys.MACHINE)
    )

    specs["computer_family"] = string(Sys.MACHINE)
    specs["cpu_model"] = first(Sys.cpu_info()).model
    specs["cpu_cores"] = string(Threads.nthreads()) * " threads"

    total_mem = Sys.total_memory()
    specs["ram_gb"] = string(round(Int, total_mem / (1024^3))) * " GB"

    return specs
end

function generate_random_3d_array(edge_size::Int, seed::Int=42)
    Random.seed!(seed)
    return 273.0f0 .+ rand(Float32, edge_size, edge_size, edge_size) .* 20.0f0
end

function time_encode_only(data::Array{Float32,3}, keepbits::Int, n_iterations::Int)
    data_flat = vec(data)
    times = Float64[]
    result = Vector{UInt32}(undef, length(data_flat))

    for _ in 1:n_iterations
        t = @elapsed begin
            bitround_array!(result, data_flat, keepbits)
        end
        push!(times, t * 1e6)
    end

    return Dict(
        "mean_us" => mean(times),
        "std_us" => std(times),
        "min_us" => minimum(times),
        "max_us" => maximum(times),
        "median_us" => median(times)
    )
end

function time_decode_only(encoded_data::Vector{UInt32}, keepbits::Int, n_iterations::Int)
    times = Float64[]
    result = Vector{Float32}(undef, length(encoded_data))

    for _ in 1:n_iterations
        t = @elapsed begin
            decode_bitround_array!(result, encoded_data, keepbits)
        end
        push!(times, t * 1e6)
    end

    return Dict(
        "mean_us" => mean(times),
        "std_us" => std(times),
        "min_us" => minimum(times),
        "max_us" => maximum(times),
        "median_us" => median(times)
    )
end

function run_benchmarks(; keepbits::Int=16, n_warmup::Int=3, n_iterations::Int=10)
    edge_sizes = [1, 10, 100, 1000]
    results = Dict{String, Any}(
        "julia" => Dict{String, Any}(),
        "machine_specs" => get_machine_specs()
    )

    println(stderr, "Julia bitround benchmark (using BitInformation.jl)")
    println(stderr, "=" ^ 60)
    println(stderr, "Machine: ", results["machine_specs"]["computer_family"])
    println(stderr, "CPU: ", results["machine_specs"]["cpu_model"])
    println(stderr, "Cores: ", results["machine_specs"]["cpu_cores"])
    println(stderr, "RAM: ", results["machine_specs"]["ram_gb"])
    println(stderr, "OS: ", results["machine_specs"]["os"])
    println(stderr)
    println(stderr, "keepbits: ", keepbits)
    println(stderr, "warmup iterations: ", n_warmup)
    println(stderr, "measured iterations: ", n_iterations)
    println(stderr)

    for edge_size in edge_sizes
        size_str = string(edge_size) * "x" * string(edge_size) * "x" * string(edge_size)
        n_elements = edge_size ^ 3
        data_mb = n_elements * 4 / (1024 * 1024)

        println(stderr, "Benchmarking ", size_str, " (", n_elements, " elements, ", round(data_mb; digits=3), " MB)...")

        data = generate_random_3d_array(edge_size)

        for _ in 1:n_warmup
            bitround_array(vec(data), keepbits)
        end

        encode_stats = time_encode_only(data, keepbits, n_iterations)

        encoded = bitround_array(vec(data), keepbits)
        decode_stats = time_decode_only(encoded, keepbits, n_iterations)

        results["julia"][size_str] = Dict(
            "n_elements" => n_elements,
            "encode_us" => encode_stats,
            "decode_us" => decode_stats
        )

        @printf(stderr, "  Encode: %.2f ± %.2f us\n", encode_stats["mean_us"], encode_stats["std_us"])
        @printf(stderr, "  Decode: %.2f ± %.2f us\n", decode_stats["mean_us"], decode_stats["std_us"])
    end

    return results
end

function format_markdown_report(results::Dict)
    md = String[]
    push!(md, "## Julia bitround Benchmark Results")
    push!(md, "")
    push!(md, "### Machine Specifications")
    push!(md, "- Computer: $(results["machine_specs"]["computer_family"])")
    push!(md, "- CPU: $(results["machine_specs"]["cpu_model"])")
    push!(md, "- Cores: $(results["machine_specs"]["cpu_cores"])")
    push!(md, "- RAM: $(results["machine_specs"]["ram_gb"])")
    push!(md, "- OS: $(results["machine_specs"]["os"])")
    push!(md, "")
    push!(md, "### Timing Results (microseconds)")
    push!(md, "")
    push!(md, "| Array Size | Elements | Encode (μs) | Decode (μs) |")
    push!(md, "|------------|----------|-------------|-------------|")

    for size_str in sort(collect(keys(results["julia"])))
        data = results["julia"][size_str]
        encode_mean = data["encode_us"]["mean_us"]
        encode_std = data["encode_us"]["std_us"]
        decode_mean = data["decode_us"]["mean_us"]
        decode_std = data["decode_us"]["std_us"]
        n_elements = data["n_elements"]
        push!(md, "| $size_str | $n_elements | $(round(encode_mean; digits=2)) ± $(round(encode_std; digits=2)) | $(round(decode_mean; digits=2)) ± $(round(decode_std; digits=2)) |")
    end

    push!(md, "")
    return join(md, "\n")
end

function main()
    keepbits = 16
    n_warmup = 3
    n_iterations = 10
    output_json = false
    output_markdown = false

    for i in 1:length(ARGS)
        if ARGS[i] == "--keepbits" && i < length(ARGS)
            keepbits = parse(Int, ARGS[i+1])
        elseif ARGS[i] == "--warmup" && i < length(ARGS)
            n_warmup = parse(Int, ARGS[i+1])
        elseif ARGS[i] == "--iterations" && i < length(ARGS)
            n_iterations = parse(Int, ARGS[i+1])
        elseif ARGS[i] == "--json"
            output_json = true
        elseif ARGS[i] == "--markdown"
            output_markdown = true
        end
    end

    results = run_benchmarks(keepbits=keepbits, n_warmup=n_warmup, n_iterations=n_iterations)

    if output_json
        println(JSON.json(results, 2))
    elseif output_markdown
        println(format_markdown_report(results))
    else
        println("\n" * format_markdown_report(results))
    end

    return results
end

main()
