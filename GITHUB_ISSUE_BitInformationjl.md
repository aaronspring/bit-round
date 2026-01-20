## Bug Report: Sign Bit Corruption in BitRound Algorithm for Float32

### Summary

The `bitround_ieee_encode` function in BitInformation.jl incorrectly handles negative floating-point numbers, special values like `-0.0`, and infinities. The implementation corrupts the sign bit and fails to preserve IEEE 754 special values.

### Affected Code

**File**: `src/bitround.jl` (or `scripts/generate_reference.jl`)

**Buggy function** (Float32, lines 32-48):
```julia
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

    ui_new = (ui + ulp_half + tie_bit) & keepmask  # BUG HERE

    return ui_new
end
```

Same bug exists in the Float64 version (lines 54-70). Apply the same fix pattern:

```julia
function bitround_ieee_encode(x::Float64, nbits::Int)
    mantissa_bits = 52
    if nbits >= mantissa_bits
        return reinterpret(UInt64, x)
    end

    maskbits = mantissa_bits - nbits
    all_set = ~UInt64(0)  # 0xFFFFFFFFFFFFFFFF
    mask = (all_set >> maskbits) << maskbits  # Full 64-bit mask
    half_quantum1 = (UInt64(1) << (maskbits - 1)) - 1

    ui = reinterpret(UInt64, x)
    ui_add = ui + ((ui >> maskbits) & UInt64(1)) + half_quantum1

    return ui_add & mask
end
```

### Evidence

| Input Value | Input Bits   | Expected (numcodecs) | Julia (buggy) |
|-------------|--------------|----------------------|---------------|
| -0.0        | 0x80000000   | 0x80000000 (-0.0)    | 0x00000000 (+0.0) |
| -1.0        | 0xBF800000   | 0xBF800000 (-1.0)    | 0x3F800000 (+1.0) |
| +inf        | 0x7F800000   | 0x7F800000 (inf)     | 0x3F800000 (1.0) |
| -inf        | 0xFF800000   | 0xFF800000 (-inf)    | 0x3F800000 (1.0) |

### Root Cause

**Issue 1: Incorrect mask construction**

Julia uses a mask that only preserves mantissa bits:
```julia
keepmask = UInt32(0x007fffff) << shift  # Only mantissa (bits 0-22)
```

This clears the sign bit (bit 31) and exponent bits (bits 23-30).

The correct approach (from numcodecs/Zarr) preserves all bits:
```python
mask = (all_set >> maskbits) << maskbits  # Full 32-bit mask
```

**Issue 2: Missing wrapping arithmetic**

Julia uses regular integer addition:
```julia
ui_new = (ui + ulp_half + tie_bit) & keepmask
```

When adding to a negative number (sign bit = 1), the addition can overflow. For example:
- `-0.0` = 0x80000000
- With nbits=16: shift=7, ulp_half=64, tie_bit=0
- Addition: 0x80000000 + 0x40 = 0x80000040
- Mask with 0x007fffff << 7 = 0x3FFFFF80 clears upper bits → 0x00000000

The correct approach uses wrapping arithmetic:
```rust
let ui_add = ui.wrapping_add(((ui >> maskbits) & 1) + half_quantum1);
ui_add & mask
```

### Reference Implementations

**Rust implementation** (correct, matching numcodecs):
```rust
fn bitround_numcodecs32(x: u32, nbits: u32) -> u32 {
    let mantissa_bits = 23u32;
    if nbits >= mantissa_bits {
        return x;
    }

    let maskbits = mantissa_bits - nbits;
    let all_set: u32 = !0;
    let mask = (all_set >> maskbits) << maskbits;  // Keeps ALL bits
    let half_quantum1 = (1_u32 << (maskbits - 1)) - 1;

    let ui = x;
    let ui_add = ui.wrapping_add(((ui >> maskbits) & 1) + half_quantum1);
    ui_add & mask
}
```

**Python/numcodecs**: Uses the same algorithm as Rust. See [numcodecs BitRound source](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py).

### Impact

This bug affects:
- Any Float32 data containing negative numbers
- Data containing `-0.0`, infinities, or NaN
- Negative values are incorrectly converted to positive

Float64 may also be affected but edge cases may not manifest as frequently.

### Suggested Fix

Replace the bitround algorithm with the numcodecs-compatible version:

```julia
function bitround_ieee_encode(x::Float32, nbits::Int)
    mantissa_bits = 23
    if nbits >= mantissa_bits
        return reinterpret(UInt32, x)
    end

    maskbits = mantissa_bits - nbits
    all_set = ~UInt32(0)  # 0xFFFFFFFF
    mask = (all_set >> maskbits) << maskbits  # Full 32-bit mask
    half_quantum1 = (UInt32(1) << (maskbits - 1)) - 1

    ui = reinterpret(UInt32, x)
    ui_add = ui + ((ui >> maskbits) & UInt32(1)) + half_quantum1
    # Julia's + with UInt32 already wraps

    return ui_add & mask
end
```

### References

- [numcodecs BitRound source](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py)
- [Rust bitround implementation](https://github.com/anomalyco/bit-round)
- [Nature paper: Compressing atmospheric data](https://doi.org/10.1038/s43588-021-00156-2)
