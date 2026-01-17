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

    /// Encodes an f32 array using bitround compression.
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
        let mask = u32::MAX << (32 - self.nbits);
        let rounding_mask = 1u32 << (32 - self.nbits - 1);
        let _ = mask; // Reserved for potential future use
        let _ = rounding_mask;

        for &value in data {
            let bits = value.to_bits();
            let sign = bits >> 31;
            let exp = (bits >> 23) & 0xFF;
            let mantissa = bits & 0x7FFFFF;

            if exp == 0 && mantissa == 0 {
                output.push(0);
                continue;
            }

            if exp == 255 {
                output.push(bits);
                continue;
            }

            let shift = 24 - self.nbits;
            let masked_mantissa = mantissa >> shift;
            let rounding_bit = (mantissa >> (shift - 1)) & 1;

            let new_mantissa = if rounding_bit == 1 {
                let _add = (masked_mantissa + 1) << shift;
                masked_mantissa + 1
            } else {
                masked_mantissa
            };

            if new_mantissa >= 0x800000 {
                let new_exp = exp + 1;
                output.push((sign << 31) | (new_exp << 23) | (new_mantissa >> 1));
            } else {
                output.push((sign << 31) | (exp << 23) | new_mantissa);
            }
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

        let shift = 24 - self.nbits;
        let mut output = Vec::with_capacity(data.len());

        for &value in data {
            let sign = value >> 31;
            let exp = (value >> 23) & 0xFF;
            let mantissa = value & 0x7FFFFF;

            if exp == 0 && mantissa == 0 {
                output.push(0.0);
                continue;
            }

            if exp == 255 {
                output.push(f32::from_bits(value));
                continue;
            }

            let new_mantissa = mantissa << shift;
            output.push(f32::from_bits((sign << 31) | (exp << 23) | new_mantissa));
        }

        Ok(output)
    }

    /// Encodes an f64 array using bitround compression.
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
        let shift = 53 - self.nbits;

        for &value in data {
            let bits = value.to_bits();
            let sign = bits >> 63;
            let exp = (bits >> 52) & 0x7FF;
            let mantissa = bits & 0xFFFFFFFFFFFFF;

            if exp == 0 && mantissa == 0 {
                output.push(0);
                continue;
            }

            if exp == 2047 {
                output.push(bits);
                continue;
            }

            let masked_mantissa = mantissa >> shift;
            let rounding_bit = (mantissa >> (shift - 1)) & 1;

            let new_mantissa = if rounding_bit == 1 {
                masked_mantissa + 1
            } else {
                masked_mantissa
            };

            if new_mantissa >= 0x10000000000000 {
                let new_exp = exp + 1;
                output.push((sign << 63) | (new_exp << 52) | (new_mantissa >> 1));
            } else {
                output.push((sign << 63) | (exp << 52) | new_mantissa);
            }
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

        let shift = 53 - self.nbits;
        let mut output = Vec::with_capacity(data.len());

        for &value in data {
            let sign = value >> 63;
            let exp = (value >> 52) & 0x7FF;
            let mantissa = value & 0xFFFFFFFFFFFFF;

            if exp == 0 && mantissa == 0 {
                output.push(0.0);
                continue;
            }

            if exp == 2047 {
                output.push(f64::from_bits(value));
                continue;
            }

            let new_mantissa = mantissa << shift;
            output.push(f64::from_bits((sign << 63) | (exp << 52) | new_mantissa));
        }

        Ok(output)
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
            let tolerance = 2.0f32.powi(-(16 - 1));
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
            let diff = (original - recovered).abs();
            let tolerance = 2.0f64.powi(-(32 - 1));
            assert!(diff < tolerance, "diff {} > tolerance {}", diff, tolerance);
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
