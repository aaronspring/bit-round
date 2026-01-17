function bitround_ieee(x::Float32, nbits::Int)
    mantissa_bits = 23
    println("mantissa_bits=$mantissa_bits, nbits=$nbits")

    if nbits >= mantissa_bits
        println("nbits >= mantissa_bits, returning x")
        return x
    end

    shift = mantissa_bits - nbits
    println("shift=$shift")

    # keepmask: 1s in positions we keep, 0s where we clear
    # For nbits=16, keep bits 7-22 (16 bits), clear bits 0-6 (7 bits)
    keepmask = UInt32(0x007fffff) << shift
    println("keepmask=$(string(keepmask, base=16))")

    ui = reinterpret(UInt32, x)
    println("ui=$(string(ui, base=16))")

    ulp_half = UInt32(1) << (shift - 1)
    println("ulp_half=$(string(ulp_half, base=16))")

    tie_bit = (ui >> shift) & UInt32(1)
    println("tie_bit=$tie_bit")

    ui_new = (ui + ulp_half + tie_bit) & keepmask
    println("ui + ulp_half + tie_bit = $(string(ui + ulp_half + tie_bit, base=16))")
    println("ui_new after mask = $(string(ui_new, base=16))")

    result = reinterpret(Float32, ui_new)
    println("result=$result")

    return result
end

data = Float32[1.5]
result = bitround_ieee(data[1], 16)
println("Final: ", repr(result), " = ", string(reinterpret(UInt32, result), base=16))
