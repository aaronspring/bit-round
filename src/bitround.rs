use std::fmt;

/// Error type for bitround encoding/decoding operations.
#[derive(Debug, Clone, PartialEq)]
pub struct BitroundError {
    message: String,
}

impl std::error::Error for BitroundError {}

impl fmt::Display for BitroundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl BitroundError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

/// IEEE Float types and complex numbers (mirroring Julia's IEEEFloat_and_complex)
pub trait IEEEFloat:
    Copy + std::ops::AddAssign + std::ops::SubAssign + std::cmp::PartialOrd + std::fmt::Debug
{
    fn to_bits(self) -> u64;
    fn from_bits(bits: u64) -> Self;
    fn sign_bits() -> u32;
    fn exponent_bits() -> u32;
    fn significand_bits() -> u32;
    fn sign_mask() -> u64;
    fn significand_mask() -> u64;
    fn exponent_mask() -> u64;
}

impl IEEEFloat for f32 {
    fn to_bits(self) -> u64 {
        self.to_bits() as u64
    }

    fn from_bits(bits: u64) -> Self {
        f32::from_bits(bits as u32)
    }

    fn sign_bits() -> u32 {
        1
    }
    fn exponent_bits() -> u32 {
        8
    }
    fn significand_bits() -> u32 {
        23
    }
    fn sign_mask() -> u64 {
        0x80000000u64
    }
    fn significand_mask() -> u64 {
        0x007FFFFFu64
    }
    fn exponent_mask() -> u64 {
        0x7F800000u64
    }
}

impl IEEEFloat for f64 {
    fn to_bits(self) -> u64 {
        self.to_bits()
    }

    fn from_bits(bits: u64) -> Self {
        f64::from_bits(bits)
    }

    fn sign_bits() -> u32 {
        1
    }
    fn exponent_bits() -> u32 {
        11
    }
    fn significand_bits() -> u32 {
        52
    }
    fn sign_mask() -> u64 {
        0x8000000000000000u64
    }
    fn significand_mask() -> u64 {
        0x007FFFFFFFFFFFFFu64
    }
    fn exponent_mask() -> u64 {
        0x7FF0000000000000u64
    }
}

/// Shift integer to push the mantissa in the right position. Used to determine
/// round up or down in the tie case. `keepbits` is the number of mantissa bits to
/// be kept (i.e. not zero-ed) after rounding. Special case: shift is -1 for
/// keepbits == significand_bits(T) to avoid a round away from 0 where no rounding
/// should be applied.
fn get_shift<T: IEEEFloat>(keepbits: u32) -> i32 {
    let shift = T::significand_bits() as i32 - keepbits as i32;
    shift - (keepbits == T::significand_bits()) as i32
}

/// Returns for a Float-type `T` and `keepbits`, the number of mantissa bits to be
/// kept/non-zeroed after rounding, half of the unit in the last place as unsigned integer.
/// Used in round (nearest) to add ulp/2 just before round down to achieve round nearest.
/// Technically ulp/2 here is just smaller than ulp/2 which rounds down the ties. For
/// a tie round up +1 is added in `round(T,keepbits)`.
fn get_ulp_half<T: IEEEFloat>(keepbits: u32) -> u64 {
    let significand_bits = T::significand_bits();
    if keepbits >= significand_bits {
        0
    } else {
        let remaining_bits = significand_bits - keepbits - 1;
        ((1u64 << (keepbits + 1)) - 1) >> remaining_bits
    }
}

/// Returns a mask that's 1 for all bits that are kept after rounding and 0 for the
/// discarded trailing bits.
fn get_keep_mask<T: IEEEFloat>(keepbits: u32) -> u64 {
    let significand_bits = T::significand_bits();
    if keepbits >= significand_bits {
        !T::significand_mask()
    } else {
        !((1 << (significand_bits - keepbits)) - 1)
    }
}

/// Returns a mask that's `1` for a given `mantissabit` and `0` else. Mantissa bits
/// are positive for the mantissa (`mantissabit = 1` is the first mantissa bit), `mantissa = 0`
/// is the last exponent bit, and negative for the other exponent bits.
fn get_bit_mask<T: IEEEFloat>(mantissabit: i32) -> u64 {
    T::sign_mask() >> (T::exponent_bits() as i32 + mantissabit)
}

#[allow(dead_code)]
fn to_unsigned_u64(x: u64) -> u64 {
    x
}

/// Bitround algorithm matching numcodecs BitRound for f32
fn bitround_numcodecs32(x: u32, nbits: u32) -> u32 {
    let mantissa_bits = 23u32;
    if nbits >= mantissa_bits {
        return x;
    }

    let maskbits = mantissa_bits - nbits;
    let all_set: u32 = !0;
    let mask = (all_set >> maskbits) << maskbits;
    let half_quantum1 = (1_u32 << (maskbits - 1)) - 1;

    let ui = x;
    let ui_add = ui.wrapping_add(((ui >> maskbits) & 1) + half_quantum1);
    ui_add & mask
}

/// Bitround algorithm matching numcodecs BitRound for f64
fn bitround_numcodecs64(x: u64, nbits: u32) -> u64 {
    let mantissa_bits = 52u32;
    if nbits >= mantissa_bits {
        return x;
    }

    let maskbits = mantissa_bits - nbits;
    let all_set: u64 = !0;
    let mask = (all_set >> maskbits) << maskbits;
    let half_quantum1 = (1_u64 << (maskbits - 1)) - 1;

    let ui = x;
    let ui_add = ui.wrapping_add(((ui >> maskbits) & 1) + half_quantum1);
    ui_add & mask
}

/// Scalar version of round that first obtains shift, ulp_half, keep_mask and then rounds

/// Bitshaving for floats. Sets trailing bits to 0 (round towards zero).
fn shave<T: IEEEFloat>(x: T, keepmask: u64) -> T {
    let ui = x.to_bits() & keepmask;
    T::from_bits(ui)
}

/// Halfshaving for floats. Replaces trailing bits with `1000...` a variant
/// of round nearest whereby the representable numbers are halfway between those
/// from shaving or IEEE's round nearest.
fn halfshave<T: IEEEFloat>(x: T, keepmask: u64, bitmask: u64) -> T {
    let ui = (x.to_bits() & keepmask) | bitmask;
    T::from_bits(ui)
}

/// Bitsetting for floats. Replace trailing bits with `1`s (round away from zero).
fn set_one<T: IEEEFloat>(x: T, setmask: u64) -> T {
    let ui = x.to_bits() | setmask;
    T::from_bits(ui)
}

/// Encoder for bitround compression algorithm.
///
/// Bitround reduces the precision of floating-point arrays by keeping only the
/// most significant `nbits` bits. This reduces storage size while preserving
/// the most significant information in the data.
///
/// # Example
///
/// ```
/// use bit_round::bitround::BitroundEncoder;
///
/// let data = vec![1.0f32, 2.5, 3.14159];
/// let encoder = BitroundEncoder::new(16).unwrap();
/// let encoded = encoder.encode_f32(&data).unwrap();
/// let decoded = encoder.decode_f32(&encoded).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct BitroundEncoder {
    nbits: u8,
}

impl BitroundEncoder {
    /// Creates a new `BitroundEncoder` with the specified number of bits.
    ///
    /// The encoder can be used for both f32 and f64 data, but the maximum
    /// effective precision is 24 bits for f32 and 53 bits for f64.
    ///
    /// # Arguments
    ///
    /// * `nbits` - Number of significant bits to preserve (1-53).
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` is 0 or greater than 53.
    pub fn new(nbits: u8) -> Result<Self, BitroundError> {
        if nbits == 0 || nbits > 53 {
            return Err(BitroundError::new(format!(
                "nbits must be between 1 and 53, got {}",
                nbits
            )));
        }
        Ok(Self { nbits })
    }

    /// Creates a new `BitroundEncoder` for f32 data.
    ///
    /// # Arguments
    ///
    /// * `nbits` - Number of significant bits to preserve (1-24).
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` is 0 or greater than 24.
    pub fn new_f32(nbits: u8) -> Result<Self, BitroundError> {
        if nbits == 0 || nbits > 24 {
            return Err(BitroundError::new(format!(
                "nbits must be between 1 and 24 for f32, got {}",
                nbits
            )));
        }
        Ok(Self { nbits })
    }

    /// Creates a new `BitroundEncoder` for f64 data.
    ///
    /// # Arguments
    ///
    /// * `nbits` - Number of significant bits to preserve (1-53).
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` is 0 or greater than 53.
    pub fn new_f64(nbits: u8) -> Result<Self, BitroundError> {
        if nbits == 0 || nbits > 53 {
            return Err(BitroundError::new(format!(
                "nbits must be between 1 and 53 for f64, got {}",
                nbits
            )));
        }
        Ok(Self { nbits })
    }

    /// Encodes an f32 array using IEEE round to nearest tie to even.
    ///
    /// Each f32 value is compressed to use exactly `nbits` significant bits.
    /// The output is stored as u32 values for efficient storage.
    ///
    /// # Arguments
    ///
    /// * `data` - Slice of f32 values to encode.
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` exceeds 24.
    pub fn encode_f32(&self, data: &[f32]) -> Result<Vec<u32>, BitroundError> {
        if self.nbits > 24 {
            return Err(BitroundError::new(format!(
                "nbits {} exceeds maximum 24 for f32",
                self.nbits
            )));
        }

        let mut output = Vec::with_capacity(data.len());

        for &value in data {
            let bits = bitround_numcodecs32(value.to_bits() as u32, self.nbits as u32);
            output.push(bits);
        }

        Ok(output)
    }

    /// Decodes an f32 array from bitround-encoded u32 values.
    ///
    /// Restores the original precision by zero-extending the mantissa.
    ///
    /// # Arguments
    ///
    /// * `data` - Slice of encoded u32 values.
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` exceeds 24.
    pub fn decode_f32(&self, data: &[u32]) -> Result<Vec<f32>, BitroundError> {
        if self.nbits > 24 {
            return Err(BitroundError::new(format!(
                "nbits {} exceeds maximum 24 for f32",
                self.nbits
            )));
        }

        let mantissa_bits = 23u32;
        let maskbits = mantissa_bits.saturating_sub(self.nbits as u32);
        let mask = if maskbits == 0 {
            !0u32
        } else {
            ((!0u32) >> maskbits) << maskbits
        };

        let mut output = Vec::with_capacity(data.len());

        for &value in data {
            let ui = value & mask;
            output.push(f32::from_bits(ui));
        }

        Ok(output)
    }

    /// Encodes an f64 array using IEEE round to nearest tie to even.
    ///
    /// Each f64 value is compressed to use exactly `nbits` significant bits.
    /// The output is stored as u64 values for efficient storage.
    ///
    /// # Arguments
    ///
    /// * `data` - Slice of f64 values to encode.
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` exceeds 53.
    pub fn encode_f64(&self, data: &[f64]) -> Result<Vec<u64>, BitroundError> {
        if self.nbits > 53 {
            return Err(BitroundError::new(format!(
                "nbits {} exceeds maximum 53 for f64",
                self.nbits
            )));
        }

        let mut output = Vec::with_capacity(data.len());

        for &value in data {
            let bits = bitround_numcodecs64(value.to_bits(), self.nbits as u32);
            output.push(bits);
        }

        Ok(output)
    }

    /// Decodes an f64 array from bitround-encoded u64 values.
    ///
    /// Restores the original precision by zero-extending the mantissa.
    ///
    /// # Arguments
    ///
    /// * `data` - Slice of encoded u64 values.
    ///
    /// # Errors
    ///
    /// Returns an error if `nbits` exceeds 53.
    pub fn decode_f64(&self, data: &[u64]) -> Result<Vec<f64>, BitroundError> {
        if self.nbits > 53 {
            return Err(BitroundError::new(format!(
                "nbits {} exceeds maximum 53 for f64",
                self.nbits
            )));
        }

        let mantissa_bits = 52u32;
        let maskbits = mantissa_bits.saturating_sub(self.nbits as u32);
        let mask = if maskbits == 0 {
            !0u64
        } else {
            ((!0u64) >> maskbits) << maskbits
        };

        let mut output = Vec::with_capacity(data.len());

        for &value in data {
            let ui = value & mask;
            output.push(f64::from_bits(ui));
        }

        Ok(output)
    }

    /// In-place version of shave for f32 arrays.
    pub fn shave_f32_inplace(&self, data: &mut [f32]) {
        let keep_mask = get_keep_mask::<f32>(self.nbits as u32);
        for value in data.iter_mut() {
            *value = shave(*value, keep_mask);
        }
    }

    /// In-place version of shave for f64 arrays.
    pub fn shave_f64_inplace(&self, data: &mut [f64]) {
        let keep_mask = get_keep_mask::<f64>(self.nbits as u32);
        for value in data.iter_mut() {
            *value = shave(*value, keep_mask);
        }
    }

    /// In-place version of halfshave for f32 arrays.
    pub fn halfshave_f32_inplace(&self, data: &mut [f32]) {
        let keep_mask = get_keep_mask::<f32>(self.nbits as u32);
        let bit_mask = get_bit_mask::<f32>(self.nbits as i32 + 1);
        for value in data.iter_mut() {
            *value = halfshave(*value, keep_mask, bit_mask);
        }
    }

    /// In-place version of halfshave for f64 arrays.
    pub fn halfshave_f64_inplace(&self, data: &mut [f64]) {
        let keep_mask = get_keep_mask::<f64>(self.nbits as u32);
        let bit_mask = get_bit_mask::<f64>(self.nbits as i32 + 1);
        for value in data.iter_mut() {
            *value = halfshave(*value, keep_mask, bit_mask);
        }
    }

    /// In-place version of set_one for f32 arrays.
    pub fn set_one_f32_inplace(&self, data: &mut [f32]) {
        let set_mask = !get_keep_mask::<f32>(self.nbits as u32);
        for value in data.iter_mut() {
            *value = set_one(*value, set_mask);
        }
    }

    /// In-place version of set_one for f64 arrays.
    pub fn set_one_f64_inplace(&self, data: &mut [f64]) {
        let set_mask = !get_keep_mask::<f64>(self.nbits as u32);
        for value in data.iter_mut() {
            *value = set_one(*value, set_mask);
        }
    }

    /// In-place bitgrooming for f32 arrays (shave/set alternatingly).
    pub fn groom_f32_inplace(&self, data: &mut [f32]) {
        let keep_mask = get_keep_mask::<f32>(self.nbits as u32);
        let set_mask = !keep_mask;

        for i in (0..data.len()).step_by(2) {
            if i < data.len() {
                data[i] = shave(data[i], keep_mask);
            }
            if i + 1 < data.len() {
                data[i + 1] = set_one(data[i + 1], set_mask);
            }
        }
    }

    /// In-place bitgrooming for f64 arrays (shave/set alternatingly).
    pub fn groom_f64_inplace(&self, data: &mut [f64]) {
        let keep_mask = get_keep_mask::<f64>(self.nbits as u32);
        let set_mask = !keep_mask;

        for i in (0..data.len()).step_by(2) {
            if i < data.len() {
                data[i] = shave(data[i], keep_mask);
            }
            if i + 1 < data.len() {
                data[i + 1] = set_one(data[i + 1], set_mask);
            }
        }
    }

    /// Returns the IEEE shift value for current nbits
    pub fn get_shift_f32(&self) -> i32 {
        get_shift::<f32>(self.nbits as u32)
    }

    /// Returns the IEEE shift value for current nbits
    pub fn get_shift_f64(&self) -> i32 {
        get_shift::<f64>(self.nbits as u32)
    }

    /// Returns the ULP half value for current nbits (f32)
    pub fn get_ulp_half_f32(&self) -> u64 {
        get_ulp_half::<f32>(self.nbits as u32)
    }

    /// Returns the ULP half value for current nbits (f64)
    pub fn get_ulp_half_f64(&self) -> u64 {
        get_ulp_half::<f64>(self.nbits as u32)
    }

    /// Returns the keep mask for current nbits (f32)
    pub fn get_keep_mask_f32(&self) -> u64 {
        get_keep_mask::<f32>(self.nbits as u32)
    }

    /// Returns the keep mask for current nbits (f64)
    pub fn get_keep_mask_f64(&self) -> u64 {
        get_keep_mask::<f64>(self.nbits as u32)
    }
}

/// Bit transformation functions (matching Julia's BitInformation.jl)
pub mod transformations {
    use super::*;

    /// Bit transpose (bit shuffle) - reverses the order of bits
    pub fn bittranspose<T: IEEEFloat>(x: T) -> T {
        let ui = x.to_bits();
        let nbits = 8 * std::mem::size_of::<u64>() as u32;
        let mut result: u64 = 0;

        for i in 0..nbits {
            if (ui >> i) & 1 == 1 {
                result |= 1 << (nbits - 1 - i);
            }
        }

        T::from_bits(result)
    }

    /// XOR delta transformation
    pub fn xor_delta<T: IEEEFloat>(data: &[T]) -> Vec<u64> {
        let mut output = Vec::with_capacity(data.len());
        if data.is_empty() {
            return output;
        }

        let mut prev = data[0].to_bits();
        for value in data {
            let current = value.to_bits();
            output.push(current ^ prev);
            prev = current;
        }

        output
    }

    /// Undo XOR delta transformation
    pub fn unxor_delta<T: IEEEFloat>(data: &[u64]) -> Vec<T> {
        let mut output = Vec::with_capacity(data.len());
        if data.is_empty() {
            return output;
        }

        let mut prev = 0u64;
        for &delta in data {
            let current = prev ^ delta;
            output.push(T::from_bits(current));
            prev = current;
        }

        output
    }

    /// Signed exponent transformation
    pub fn signed_exponent<T: IEEEFloat>(x: T) -> i32 {
        let ui = x.to_bits();
        let exp_bits = T::exponent_bits();
        let biased_exp = ((ui >> T::significand_bits()) & ((1 << exp_bits) - 1)) as i32;
        let exponent_bias = (1 << (exp_bits - 1)) - 1;

        biased_exp as i32 - exponent_bias
    }

    /// Biased exponent transformation
    pub fn biased_exponent<T: IEEEFloat>(x: T) -> u32 {
        let ui = x.to_bits();
        ((ui >> T::significand_bits()) & ((1 << T::exponent_bits()) - 1)) as u32
    }
}

/// Information theory functions (matching Julia's BitInformation.jl)
pub mod information {
    use super::*;

    /// Counts the occurrences of the 1-bit in bit position i across all elements of A
    pub fn bitcount_u32(data: &[u32], bit_position: usize) -> usize {
        let nbits = 32usize;
        assert!(
            bit_position < nbits,
            "Bit position {} out of range for 32-bit type",
            bit_position
        );

        let shift = nbits - bit_position;
        let mask = 1u32 << shift;

        data.iter().filter(|&&x| (x & mask) >> shift == 1).count()
    }

    /// Counts the occurrences of the 1-bit in bit position i across all elements of A
    pub fn bitcount_u64(data: &[u64], bit_position: usize) -> usize {
        let nbits = 64usize;
        assert!(
            bit_position < nbits,
            "Bit position {} out of range for 64-bit type",
            bit_position
        );

        let shift = nbits - bit_position;
        let mask = 1u64 << shift;

        data.iter().filter(|&&x| (x & mask) >> shift == 1).count()
    }

    /// Counts occurrences of 1-bit in every bit position across all elements
    pub fn bitcount_array_f32(data: &[f32]) -> Vec<usize> {
        let nbits = 32usize;
        let uint_data: Vec<u32> = data.iter().map(|&x| x.to_bits()).collect();

        (0..nbits).map(|i| bitcount_u32(&uint_data, i)).collect()
    }

    /// Counts occurrences of 1-bit in every bit position across all elements
    pub fn bitcount_array_f64(data: &[f64]) -> Vec<usize> {
        let nbits = 64usize;
        let uint_data: Vec<u64> = data.iter().map(|&x| x.to_bits()).collect();

        (0..nbits).map(|i| bitcount_u64(&uint_data, i)).collect()
    }

    /// Returns entropy for occurrence of 0,1 in every bit position
    pub fn bitcount_entropy_f32(data: &[f32], base: f64) -> Vec<f64> {
        let counts = bitcount_array_f32(data);
        let nelements = data.len();
        let nbits = 32usize;

        (0..nbits)
            .map(|i| {
                let p = counts[i] as f64 / nelements as f64;
                entropy_binary(p, base)
            })
            .collect()
    }

    /// Returns entropy for occurrence of 0,1 in every bit position
    pub fn bitcount_entropy_f64(data: &[f64], base: f64) -> Vec<f64> {
        let counts = bitcount_array_f64(data);
        let nelements = data.len();
        let nbits = 64usize;

        (0..nbits)
            .map(|i| {
                let p = counts[i] as f64 / nelements as f64;
                entropy_binary(p, base)
            })
            .collect()
    }

    /// Binary entropy function
    fn entropy_binary(p: f64, base: f64) -> f64 {
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        let q = 1.0 - p;
        let entropy = -p * p.log(base) - q * q.log(base);
        entropy / base.log(2.0)
    }

    /// Bitpair counter for mutual information
    pub fn bitpair_count_f32(data1: &[f32], data2: &[f32]) -> Vec<[[usize; 2]; 2]> {
        assert_eq!(data1.len(), data2.len(), "Arrays must have same length");

        let nbits = 32usize;
        let uint_data1: Vec<u32> = data1.iter().map(|&x| x.to_bits()).collect();
        let uint_data2: Vec<u32> = data2.iter().map(|&x| x.to_bits()).collect();

        let mut counters: Vec<[[usize; 2]; 2]> = vec![[[0; 2]; 2]; nbits];

        for (a, b) in uint_data1.iter().zip(uint_data2.iter()) {
            for i in 0..nbits {
                let shift = nbits - i;
                let mask = 1u32 << shift;
                let bit_a = ((a & mask) >> shift) as usize;
                let bit_b = ((b & mask) >> shift) as usize;
                counters[i][bit_a][bit_b] += 1;
            }
        }

        counters
    }

    /// Bitpair counter for mutual information
    pub fn bitpair_count_f64(data1: &[f64], data2: &[f64]) -> Vec<[[usize; 2]; 2]> {
        assert_eq!(data1.len(), data2.len(), "Arrays must have same length");

        let nbits = 64usize;
        let uint_data1: Vec<u64> = data1.iter().map(|&x| x.to_bits()).collect();
        let uint_data2: Vec<u64> = data2.iter().map(|&x| x.to_bits()).collect();

        let mut counters: Vec<[[usize; 2]; 2]> = vec![[[0; 2]; 2]; nbits];

        for (a, b) in uint_data1.iter().zip(uint_data2.iter()) {
            for i in 0..nbits {
                let shift = nbits - i;
                let mask = 1u64 << shift;
                let bit_a = ((a & mask) >> shift) as usize;
                let bit_b = ((b & mask) >> shift) as usize;
                counters[i][bit_a][bit_b] += 1;
            }
        }

        counters
    }

    /// Calculate mutual information from joint probability mass function
    pub fn mutual_information_from_pmf(pmf: &[[f64; 2]; 2], base: f64) -> f64 {
        let mut mutual_info = 0.0f64;
        let nx = 2;
        let ny = 2;

        let px: [f64; 2] = [pmf[0][0] + pmf[0][1], pmf[1][0] + pmf[1][1]];
        let py: [f64; 2] = [pmf[0][0] + pmf[1][0], pmf[0][1] + pmf[1][1]];

        for j in 0..ny {
            for i in 0..nx {
                if pmf[i][j] > 0.0 {
                    mutual_info += pmf[i][j] * (pmf[i][j] / (px[i] * py[j])).log(base);
                }
            }
        }

        mutual_info / base.log(2.0)
    }

    /// Mutual bitwise information of elements in input arrays
    pub fn mutual_information_f32(data1: &[f32], data2: &[f32], base: f64) -> Vec<f64> {
        let counters = bitpair_count_f32(data1, data2);
        let nelements = data1.len();

        counters
            .iter()
            .map(|counter| {
                let pmf: [[f64; 2]; 2] = [
                    [
                        counter[0][0] as f64 / nelements as f64,
                        counter[0][1] as f64 / nelements as f64,
                    ],
                    [
                        counter[1][0] as f64 / nelements as f64,
                        counter[1][1] as f64 / nelements as f64,
                    ],
                ];
                mutual_information_from_pmf(&pmf, base)
            })
            .collect()
    }

    /// Mutual bitwise information of elements in input arrays
    pub fn mutual_information_f64(data1: &[f64], data2: &[f64], base: f64) -> Vec<f64> {
        let counters = bitpair_count_f64(data1, data2);
        let nelements = data1.len();

        counters
            .iter()
            .map(|counter| {
                let pmf: [[f64; 2]; 2] = [
                    [
                        counter[0][0] as f64 / nelements as f64,
                        counter[0][1] as f64 / nelements as f64,
                    ],
                    [
                        counter[1][0] as f64 / nelements as f64,
                        counter[1][1] as f64 / nelements as f64,
                    ],
                ];
                mutual_information_from_pmf(&pmf, base)
            })
            .collect()
    }

    /// Redundancy of two arrays (normalized mutual information)
    pub fn redundancy_f32(data1: &[f32], data2: &[f32], base: f64) -> Vec<f64> {
        let mi = mutual_information_f32(data1, data2, base);
        let ha = bitcount_entropy_f32(data1, base);
        let hb = bitcount_entropy_f32(data2, base);

        mi.iter()
            .enumerate()
            .map(|(i, &m)| {
                let hab = ha[i] + hb[i];
                if hab > 0.0 { 2.0 * m / hab } else { 0.0 }
            })
            .collect()
    }

    /// Redundancy of two arrays (normalized mutual information)
    pub fn redundancy_f64(data1: &[f64], data2: &[f64], base: f64) -> Vec<f64> {
        let mi = mutual_information_f64(data1, data2, base);
        let ha = bitcount_entropy_f64(data1, base);
        let hb = bitcount_entropy_f64(data2, base);

        mi.iter()
            .enumerate()
            .map(|(i, &m)| {
                let hab = ha[i] + hb[i];
                if hab > 0.0 { 2.0 * m / hab } else { 0.0 }
            })
            .collect()
    }

    /// Bitwise information content of array (from mutual information in adjacent entries)
    pub fn bitinformation_f32(data: &[f32], base: f64) -> Vec<f64> {
        if data.len() < 2 {
            return vec![0.0; 32];
        }

        let data1 = &data[..data.len() - 1];
        let data2 = &data[1..];

        mutual_information_f32(data1, data2, base)
    }

    /// Bitwise information content of array (from mutual information in adjacent entries)
    pub fn bitinformation_f64(data: &[f64], base: f64) -> Vec<f64> {
        if data.len() < 2 {
            return vec![0.0; 64];
        }

        let data1 = &data[..data.len() - 1];
        let data2 = &data[1..];

        mutual_information_f64(data1, data2, base)
    }

    /// Bit pattern entropy (entropy from occurrences of every possible bit pattern)
    pub fn bitpattern_entropy_f32(data: &mut [f32], base: f64) -> f64 {
        let mut uint_data: Vec<u32> = data.iter().map(|&x| x.to_bits()).collect();
        uint_data.sort();

        let n = uint_data.len();
        let mut entropy = 0.0f64;
        let mut count = 1usize;

        for i in 0..n - 1 {
            if uint_data[i] == uint_data[i + 1] {
                count += 1;
            } else {
                let p = count as f64 / n as f64;
                entropy -= p * p.log(base);
                count = 1;
            }
        }

        let p = count as f64 / n as f64;
        entropy -= p * p.log(base);
        entropy / base.log(2.0)
    }

    /// Bit pattern entropy (entropy from occurrences of every possible bit pattern)
    pub fn bitpattern_entropy_f64(data: &mut [f64], base: f64) -> f64 {
        let mut uint_data: Vec<u64> = data.iter().map(|&x| x.to_bits()).collect();
        uint_data.sort();

        let n = uint_data.len();
        let mut entropy = 0.0f64;
        let mut count = 1usize;

        for i in 0..n - 1 {
            if uint_data[i] == uint_data[i + 1] {
                count += 1;
            } else {
                let p = count as f64 / n as f64;
                entropy -= p * p.log(base);
                count = 1;
            }
        }

        let p = count as f64 / n as f64;
        entropy -= p * p.log(base);
        entropy / base.log(2.0)
    }

    /// Binomial confidence interval for p=0.5
    pub fn binom_confidence(n: usize, confidence: f64) -> f64 {
        let z = normal_quantile(1.0 - (1.0 - confidence) / 2.0);
        let p = 0.5 + z / (2.0 * (n as f64).sqrt());
        p.min(1.0)
    }

    /// Normal distribution quantile (approximation)
    fn normal_quantile(p: f64) -> f64 {
        // Approximation of inverse error function
        let a = [
            -3.969683028665376e+01,
            2.209460984245205e+02,
            -2.759285104469687e+02,
            1.383577518672690e+02,
            -3.066479806614716e+01,
            2.506628277459239e+00,
        ];
        let b = [
            -5.447609879822406e+01,
            1.615858368580409e+02,
            -1.556989798598866e+02,
            6.680131188771972e+01,
            -1.328068155288572e+01,
        ];
        let c = [
            -7.784894002430293e-03,
            -3.223964580411365e-01,
            -2.400758277161838e+00,
            -2.549732539343734e+00,
            4.374664141464968e+00,
            2.938163982698783e+00,
        ];
        let d = [
            7.784695709041462e-03,
            3.224671290700398e-01,
            2.445134137142996e+00,
            3.754408661907416e+00,
        ];

        let p_low = 0.02425;
        let p_high = 1.0 - p_low;

        if p < p_low {
            let q = (-2.0 * p).sqrt();
            let num = ((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5];
            let den = (((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0;
            return (num / den).exp().ln();
        } else if p <= p_high {
            let q = p - 0.5;
            let r = q * q;
            let num = (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q;
            let den = ((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0;
            return num / den;
        } else {
            let q = (-2.0 * (1.0 - p)).sqrt();
            let num = ((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5];
            let den = (((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0;
            return -(num / den).exp().ln();
        }
    }

    /// Binomial free entropy associated with binom_confidence
    pub fn binom_free_entropy(n: usize, confidence: f64, base: f64) -> f64 {
        let p = binom_confidence(n, confidence);
        1.0 - entropy_binary(p, base)
    }

    /// Set insignificant information to zero based on confidence level
    pub fn set_zero_insignificant(entropy: &mut [f64], nelements: usize, confidence: f64) {
        let free_entropy = binom_free_entropy(nelements, confidence, 2.0);
        for h in entropy.iter_mut() {
            if *h <= free_entropy {
                *h = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_f32_roundtrip() {
        let data = vec![1.0f32, 2.5, 3.14159, -1.5, 0.0];
        let encoder = BitroundEncoder::new(16).unwrap();
        let encoded = encoder.encode_f32(&data).unwrap();
        let decoded = encoder.decode_f32(&encoded).unwrap();

        for (original, recovered) in data.iter().zip(decoded.iter()) {
            let diff = (original - recovered).abs();
            let tolerance = 2.0f32.powi(-(16));
            assert!(diff < tolerance, "diff {} > tolerance {}", diff, tolerance);
        }
    }

    #[test]
    fn test_encode_decode_f64_roundtrip() {
        let data = vec![1.0f64, 2.5, 3.14159265358979, -1.5, 0.0];
        let encoder = BitroundEncoder::new(32).unwrap();
        let encoded = encoder.encode_f64(&data).unwrap();
        let decoded = encoder.decode_f64(&encoded).unwrap();

        for (original, recovered) in data.iter().zip(decoded.iter()) {
            let abs_original = original.abs();
            let tolerance = if abs_original > 0.0 {
                abs_original * 2.0f64.powi(-(32))
            } else {
                2.0f64.powi(-(32))
            };
            let diff = (original - recovered).abs();
            assert!(
                diff < tolerance,
                "diff {} > tolerance {} for value {}",
                diff,
                tolerance,
                original
            );
        }
    }

    #[test]
    fn test_invalid_nbits_f32() {
        let encoder = BitroundEncoder::new_f32(25);
        assert!(encoder.is_err());
    }

    #[test]
    fn test_invalid_nbits_f64() {
        let encoder = BitroundEncoder::new_f64(54);
        assert!(encoder.is_err());
    }

    #[test]
    fn test_nbits_1() {
        let data = vec![1.0f32, 2.0, 3.0];
        let encoder = BitroundEncoder::new(1).unwrap();
        let encoded = encoder.encode_f32(&data).unwrap();
        assert_eq!(encoded.len(), 3);
    }
}
