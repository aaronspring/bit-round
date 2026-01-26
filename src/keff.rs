#[derive(Debug, Clone, PartialEq)]
pub struct KeffResult {
    pub keff: f64,
    pub nbits_preserved: usize,
    pub information_preserved: f64,
    pub bit_significance: Vec<f64>,
}

impl KeffResult {
    pub fn new(keff: f64, nbits: usize, info: f64, significance: Vec<f64>) -> Self {
        Self {
            keff,
            nbits_preserved: nbits,
            information_preserved: info,
            bit_significance: significance,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeffError {
    message: String,
}

impl std::error::Error for KeffError {}

impl std::fmt::Display for KeffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl KeffError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

/// Calculate effective number of bits for f32 data.
/// Analyzes only mantissa bits (23 bits for f32), from MSB to LSB.
pub fn calculate_keff_f32(
    data: &[f32],
    significance_level: f64,
    max_nbits: usize,
) -> Result<KeffResult, KeffError> {
    if data.is_empty() {
        return Err(KeffError::new("Input data is empty".to_string()));
    }
    if significance_level <= 0.0 || significance_level > 1.0 {
        return Err(KeffError::new(
            "Significance level must be between 0 and 1".to_string(),
        ));
    }

    const MANTISSA_BITS: usize = 23;
    let valid_max_nbits = max_nbits.clamp(1, MANTISSA_BITS);
    let bits_data: Vec<u32> = data.iter().map(|&x| x.to_bits()).collect();
    let bit_significance = calculate_mantissa_significance_u32(&bits_data);

    let (keff, nbits_preserved, info_preserved) =
        find_optimal_nbits(&bit_significance, significance_level, valid_max_nbits);

    Ok(KeffResult::new(
        keff,
        nbits_preserved,
        info_preserved,
        bit_significance,
    ))
}

/// Calculate effective number of bits for f64 data.
/// Analyzes only mantissa bits (52 bits for f64), from MSB to LSB.
pub fn calculate_keff_f64(
    data: &[f64],
    significance_level: f64,
    max_nbits: usize,
) -> Result<KeffResult, KeffError> {
    if data.is_empty() {
        return Err(KeffError::new("Input data is empty".to_string()));
    }
    if significance_level <= 0.0 || significance_level > 1.0 {
        return Err(KeffError::new(
            "Significance level must be between 0 and 1".to_string(),
        ));
    }

    const MANTISSA_BITS: usize = 52;
    let valid_max_nbits = max_nbits.clamp(1, MANTISSA_BITS);
    let bits_data: Vec<u64> = data.iter().map(|&x| x.to_bits()).collect();
    let bit_significance = calculate_mantissa_significance_u64(&bits_data);

    let (keff, nbits_preserved, info_preserved) =
        find_optimal_nbits(&bit_significance, significance_level, valid_max_nbits);

    Ok(KeffResult::new(
        keff,
        nbits_preserved,
        info_preserved,
        bit_significance,
    ))
}

/// Calculate bit significance for f32 mantissa bits (bits 0-22).
/// Returns significance ordered from MSB (bit 22) to LSB (bit 0).
/// Index 0 = most significant mantissa bit, index 22 = least significant.
fn calculate_mantissa_significance_u32(bits: &[u32]) -> Vec<f64> {
    const MANTISSA_BITS: usize = 23;
    let n = bits.len() as f64;
    let mut significance = Vec::with_capacity(MANTISSA_BITS);

    // Iterate from MSB (bit 22) down to LSB (bit 0) of mantissa
    for bit_idx in (0..MANTISSA_BITS).rev() {
        let mask = 1u32 << bit_idx;
        let ones_count: usize = bits.iter().filter(|&&b| (b & mask) != 0).count();
        let p_ones = ones_count as f64 / n;
        let p_zeros = 1.0 - p_ones;

        let mut entropy = 0.0f64;
        if p_ones > 0.0 {
            entropy -= p_ones * p_ones.log2();
        }
        if p_zeros > 0.0 {
            entropy -= p_zeros * p_zeros.log2();
        }

        significance.push(if entropy.is_nan() { 0.0 } else { entropy });
    }

    significance
}

/// Calculate bit significance for f64 mantissa bits (bits 0-51).
/// Returns significance ordered from MSB (bit 51) to LSB (bit 0).
/// Index 0 = most significant mantissa bit, index 51 = least significant.
fn calculate_mantissa_significance_u64(bits: &[u64]) -> Vec<f64> {
    const MANTISSA_BITS: usize = 52;
    let n = bits.len() as f64;
    let mut significance = Vec::with_capacity(MANTISSA_BITS);

    // Iterate from MSB (bit 51) down to LSB (bit 0) of mantissa
    for bit_idx in (0..MANTISSA_BITS).rev() {
        let mask = 1u64 << bit_idx;
        let ones_count: usize = bits.iter().filter(|&&b| (b & mask) != 0).count();
        let p_ones = ones_count as f64 / n;
        let p_zeros = 1.0 - p_ones;

        let mut entropy = 0.0f64;
        if p_ones > 0.0 {
            entropy -= p_ones * p_ones.log2();
        }
        if p_zeros > 0.0 {
            entropy -= p_zeros * p_zeros.log2();
        }

        significance.push(if entropy.is_nan() { 0.0 } else { entropy });
    }

    significance
}

/// Find optimal number of bits to preserve based on accumulated significance.
/// Accumulates from MSB to LSB (index 0 to end of significance array).
fn find_optimal_nbits(
    significance: &[f64],
    target_significance: f64,
    max_nbits: usize,
) -> (f64, usize, f64) {
    let total_info: f64 = significance.iter().sum();
    if total_info == 0.0 {
        return (0.0, 1, 0.0);
    }

    let mut accumulated_info = 0.0f64;
    let mut nbits = 0;

    // Accumulate significance from MSB to LSB
    for (i, &sig) in significance.iter().enumerate().take(max_nbits) {
        accumulated_info += sig;
        nbits = i + 1;
        let info_fraction = accumulated_info / total_info;
        if info_fraction >= target_significance {
            break;
        }
    }

    let info_fraction = accumulated_info / total_info;
    let keff = accumulated_info;
    (keff, nbits, info_fraction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keff_f32_constant() {
        // All identical values should have zero entropy (all bits same)
        let data = vec![1.0f32; 1000];
        let result = calculate_keff_f32(&data, 0.99, 23).unwrap();
        // Constant data has no entropy, so keff should be 0
        assert_eq!(result.keff, 0.0);
        assert!(result.nbits_preserved <= 23);
    }

    #[test]
    fn test_keff_f64_constant() {
        // All identical values should have zero entropy
        let data = vec![1.0f64; 1000];
        let result = calculate_keff_f64(&data, 0.99, 52).unwrap();
        assert_eq!(result.keff, 0.0);
        assert!(result.nbits_preserved <= 52);
    }

    #[test]
    fn test_keff_f32_varying_data() {
        // Data with variation should have non-zero entropy
        let data: Vec<f32> = (0..1000).map(|i| i as f32 * 0.001).collect();
        let result = calculate_keff_f32(&data, 0.99, 23).unwrap();
        assert!(result.keff > 0.0);
        assert!(result.nbits_preserved >= 1);
        assert!(result.nbits_preserved <= 23);
    }

    #[test]
    fn test_keff_f64_varying_data() {
        // Data with variation should have non-zero entropy
        let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.001).collect();
        let result = calculate_keff_f64(&data, 0.99, 52).unwrap();
        assert!(result.keff > 0.0);
        assert!(result.nbits_preserved >= 1);
        assert!(result.nbits_preserved <= 52);
    }

    #[test]
    fn test_keff_empty_data() {
        let data: Vec<f32> = vec![];
        let result = calculate_keff_f32(&data, 0.99, 23);
        assert!(result.is_err());
    }

    #[test]
    fn test_keff_invalid_significance_level() {
        let data = vec![1.0f32; 10];
        assert!(calculate_keff_f32(&data, 0.0, 23).is_err());
        assert!(calculate_keff_f32(&data, 1.5, 23).is_err());
        assert!(calculate_keff_f32(&data, -0.1, 23).is_err());
    }

    #[test]
    fn test_bit_significance_ordering() {
        // Verify that significance is ordered MSB to LSB
        // For random-ish data, MSB bits should generally have lower entropy
        // than LSB bits (more structure in high bits)
        let data: Vec<f64> = (0..1000).map(|i| (i as f64).sqrt()).collect();
        let result = calculate_keff_f64(&data, 0.99, 52).unwrap();
        assert_eq!(result.bit_significance.len(), 52);
    }

    #[test]
    fn test_mantissa_bits_only_f32() {
        let data: Vec<f32> = (0..100).map(|i| i as f32 * 0.1).collect();
        let result = calculate_keff_f32(&data, 0.99, 23).unwrap();
        // Should only have 23 significance values (mantissa bits only)
        assert_eq!(result.bit_significance.len(), 23);
    }

    #[test]
    fn test_mantissa_bits_only_f64() {
        let data: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let result = calculate_keff_f64(&data, 0.99, 52).unwrap();
        // Should only have 52 significance values (mantissa bits only)
        assert_eq!(result.bit_significance.len(), 52);
    }
}
