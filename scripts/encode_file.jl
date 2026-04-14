#!/usr/bin/env julia
# Read raw little-endian f32 array, bitround with keepbits, write u32 output.
# Mirrors src/bin/encode_file.rs and scripts/verify_equivalence.py's numcodecs path.

using BitInformation

function parse_args()
    input = nothing
    output = nothing
    keepbits = 16
    i = 1
    while i <= length(ARGS)
        if ARGS[i] == "--input"
            input = ARGS[i+1]; i += 2
        elseif ARGS[i] == "--output"
            output = ARGS[i+1]; i += 2
        elseif ARGS[i] == "--keepbits"
            keepbits = parse(Int, ARGS[i+1]); i += 2
        else
            error("unknown arg: $(ARGS[i])")
        end
    end
    @assert input !== nothing "--input required"
    @assert output !== nothing "--output required"
    return input, output, keepbits
end

function main()
    input, output, keepbits = parse_args()

    bytes = read(input)
    @assert length(bytes) % 4 == 0 "input size must be multiple of 4 bytes"
    n = length(bytes) ÷ 4
    data = reinterpret(Float32, bytes) |> collect

    encoded = Vector{UInt32}(undef, n)
    @inbounds for i in 1:n
        encoded[i] = reinterpret(UInt32, round(data[i], keepbits))
    end

    open(output, "w") do io
        write(io, encoded)
    end
end

main()
