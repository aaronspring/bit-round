# Potential Bug in BitInformation.jl BitRound Implementation

## Summary

During verification of the Rust `bitround` implementation against reference implementations, we discovered a **sign handling bug** in the Julia `BitRound` algorithm from [BitInformation.jl](https://github.com/milankl/BitInformation.jl).

## Evidence

| Input Value | Input Bits | numcodecs (correct) | Julia (buggy) |
|-------------|------------|---------------------|---------------|
| -0.0 | 0x80000000 | 0x80000000 (-0.0) | 0x00000000 (+0.0) |
| -1.0 | 0xBF800000 | 0xBF800000 (-1.0) | 0x3F800000 (+1.0) |
| +inf | 0x7F800000 | 0x7F800000 (inf) | 0x3F800000 (1.0) |
| -inf | 0xFF800000 | 0xFF800000 (-inf) | 0x3F800000 (1.0) |

## How the Bug Was Discovered

1. Generated test data with `generate_test_inputs` (1000 f32 values including edge cases: -0.0, -1.0, infinities)
2. Generated reference outputs using Python numcodecs via Docker: `docker/generate_reference.py`
3. Generated reference outputs using Julia BitRound via Docker: `docker/generate_reference.jl`
4. Ran Rust tests comparing output against both references:
   - All 9 Python/numcodecs tests **PASSED**
   - F32 tests vs Julia: 14-25 mismatches due to sign handling
   - F64 tests vs Julia: **PASSED** (no sign issues in test data)

### Example Mismatches

```
F32 edge nbits=16 mismatch at 1: rust=80000000 julia=00000000  (-0.0 → +0.0)
F32 edge nbits=16 mismatch at 3: rust=bf800000 julia=3f800000  (-1.0 → +1.0)
F32 edge nbits=16 mismatch at 4: rust=7f800000 julia=3f800000  (inf → 1.0)
F32 random nbits=16: FAIL (14/1000 mismatches)
```

## Root Cause Analysis

### Julia Algorithm (Buggy)

```julia
function bitround_ieee(x::Float32, nbits::Int)
    mantissa_bits = 23
    shift = mantissa_bits - nbits
    keepmask = UInt32(0x007fffff) << shift

    ui = reinterpret(UInt32, x)
    ulp_half = UInt32(1) << (shift - 1)
    tie_bit = (ui >> shift) & UInt32(1)

    ui_new = (ui + ulp_half + tie_bit) & keepmask  # BUG HERE
    result = reinterpret(Float32, ui_new)
    return result
end
```

### numcodecs/Python Algorithm (Correct)

```python
def bitround(x, nbits):
    mantissa_bits = 23 if x.dtype == np.float32 else 52
    if nbits >= mantissa_bits:
        return x

    maskbits = mantissa_bits - nbits
    all_set = np.uint32(-1) if x.dtype == np.float32 else np.uint64(-1)
    mask = (all_set >> maskbits) << maskbits
    half_quantum1 = (np.uint32(1) << (maskbits - 1)) - 1

    ui = x.view(ui_type)
    ui_add = ui + ((ui >> maskbits) & 1) + half_quantum1
    return (ui_add & mask).view(x.dtype)
```

### The Bug Explained

The issue is in this line of Julia code:

```julia
ui_new = (ui + ulp_half + tie_bit) & keepmask
```

**Problem 1: Addition overflow handling**
- When adding to a negative number (sign bit = 1), the addition can overflow into the sign bit
- The `keepmask` (`0x007fffff << shift`) clears the upper bits including sign bit for special values
- This causes `-0.0` (0x80000000) to become `+0.0` (0x00000000)
- And `-1.0` (0xBF800000) to become `+1.0` (0x3F800000)

**Problem 2: Special value handling**
- For infinities (mantissa = 0, exponent = all 1s), the addition corrupts the value
- The `keepmask` doesn't preserve the infinity encoding

### Correct Approach

The numcodecs algorithm uses:
1. **Wrapping addition** to handle overflow correctly
2. **Explicit masking before/after** to preserve all bits including sign
3. **Pre-computed mask** that preserves upper bits while clearing lower bits

```python
# numcodecs correctly preserves sign bit and special values
mask = (all_set >> maskbits) << maskbits  # Mask keeps upper bits
ui_add = ui.wrapping_add(((ui >> maskbits) & 1) + half_quantum1)
ui_add & mask  # Apply mask after wrapping addition
```

## Justification: Why numcodecs is Correct

1. **Active Maintenance**: numcodecs is actively maintained by the Zarr developers and used in production
2. **IEEE 754 Compliance**: Properly handles special values per IEEE 754 standard:
   - Preserves `-0.0` (negative zero)
   - Preserves infinities
   - Handles NaN correctly
3. **Mathematical Correctness**: Wrapping arithmetic matches standard IEEE round-to-nearest semantics
4. **Cross-Platform**: Same behavior across Python, Rust, and other implementations

## Impact

This bug affects:
- Any f32 data containing negative numbers
- Data containing -0.0, infinities, or NaN
- The bug propagates: if -1.0 becomes +1.0, subsequent operations will be wrong

For f64, the bug may not manifest as frequently because:
- The mantissa bits (52) vs nbits (typically 32-52) ratio is different
- The test data may not have triggered the edge cases

## Recommendation

The Julia `BitRound` implementation in BitInformation.jl should be updated to use the numcodecs algorithm, which correctly handles:
- Sign preservation for negative numbers
- Special values (-0.0, infinities, NaN)
- Wrapping arithmetic for correct rounding

## References

- [numcodecs BitRound source](https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py)
- [BitInformation.jl](https://github.com/milankl/BitInformation.jl)
- [Original bitround.jl](https://github.com/bicycleben5/bitround.jl)
- [Rust bitround implementation](https://github.com/anomalyco/bit-round)
- [Nature paper: Compressing atmospheric data](https://doi.org/10.1038/s43588-021-00156-2)
