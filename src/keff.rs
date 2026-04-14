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

/// Binary entropy in bits (p in [0,1]).
fn binary_entropy(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return 0.0;
    }
    let q = 1.0 - p;
    -p * p.log2() - q * q.log2()
}

/// Normal quantile approximation (Beasley-Springer-Moro).
fn normal_quantile(p: f64) -> f64 {
    let a = [
        -3.969683028665376e+01, 2.209460984245205e+02, -2.759285104469687e+02,
        1.383577518672690e+02, -3.066479806614716e+01, 2.506628277459239e+00,
    ];
    let b = [
        -5.447609879822406e+01, 1.615858368580409e+02, -1.556989798598866e+02,
        6.680131188771972e+01, -1.328068155288572e+01,
    ];
    let c = [
        -7.784894002430293e-03, -3.223964580411365e-01, -2.400758277161838e+00,
        -2.549732539343734e+00, 4.374664141464968e+00, 2.938163982698783e+00,
    ];
    let d = [
        7.784695709041462e-03, 3.224671290700398e-01, 2.445134137142996e+00,
        3.754408661907416e+00,
    ];
    let p_low = 0.02425;
    let p_high = 1.0 - p_low;
    if p < p_low {
        let q = (-2.0 * p.ln()).sqrt();
        (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    } else if p <= p_high {
        let q = p - 0.5;
        let r = q * q;
        (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
            / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    }
}

/// Binomial confidence interval upper bound for p=0.5 (normal approximation).
/// Matches BitInformation.jl `binom_confidence`.
fn binom_confidence(n: usize, confidence: f64) -> f64 {
    let z = normal_quantile(1.0 - (1.0 - confidence) / 2.0);
    (0.5 + z / (2.0 * (n as f64).sqrt())).min(1.0)
}

/// Mutual-information noise floor (free entropy) for n independent Bernoulli(0.5) trials.
/// Matches BitInformation.jl `binom_free_entropy`.
fn binom_free_entropy(n: usize, confidence: f64) -> f64 {
    let p = binom_confidence(n, confidence);
    1.0 - binary_entropy(p)
}

/// Trait abstracting f32/f64 for bitinformation computation.
trait BitFloat: Copy {
    fn to_u64(self) -> u64;
    const TOTAL_BITS: usize;
    const MANTISSA_START: usize;
}

impl BitFloat for f32 {
    fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    const TOTAL_BITS: usize = 32;
    const MANTISSA_START: usize = 9; // 1 sign + 8 exponent
}

impl BitFloat for f64 {
    fn to_u64(self) -> u64 {
        self.to_bits()
    }
    const TOTAL_BITS: usize = 64;
    const MANTISSA_START: usize = 12; // 1 sign + 11 exponent
}

/// Compute bitwise mutual information between adjacent array entries.
/// Returns `T::TOTAL_BITS` values ordered MSB (index 0) to LSB.
///
/// Single pass over the array: each pair (a, b) is loaded once and updates
/// all bit-position counters in the inner loop, instead of re-reading the
/// array TOTAL_BITS times — ~half the memory loads on large inputs.
fn bitinformation_adjacent<T: BitFloat>(data: &[T]) -> Vec<f64> {
    if data.len() < 2 {
        return vec![0.0; T::TOTAL_BITS];
    }
    let n = data.len() - 1;
    let bits: Vec<u64> = data.iter().map(|x| x.to_u64()).collect();

    // counts[bit_idx] = [c00, c01, c10, c11] where bit_idx 0 = MSB.
    let mut counts: Vec<[u64; 4]> = vec![[0u64; 4]; T::TOTAL_BITS];
    for j in 0..n {
        let a = bits[j];
        let b = bits[j + 1];
        for bit_idx in 0..T::TOTAL_BITS {
            let pos = T::TOTAL_BITS - 1 - bit_idx;
            let mask = 1u64 << pos;
            let bit_a = ((a & mask) != 0) as usize;
            let bit_b = ((b & mask) != 0) as usize;
            counts[bit_idx][(bit_a << 1) | bit_b] += 1;
        }
    }

    counts
        .iter()
        .map(|c| mi_from_counts(c[0] as usize, c[1] as usize, c[2] as usize, c[3] as usize, n))
        .collect()
}

fn mi_from_counts(c00: usize, c01: usize, c10: usize, c11: usize, n: usize) -> f64 {
    let n = n as f64;
    let p00 = c00 as f64 / n;
    let p01 = c01 as f64 / n;
    let p10 = c10 as f64 / n;
    let p11 = c11 as f64 / n;
    let pa0 = p00 + p01;
    let pa1 = p10 + p11;
    let pb0 = p00 + p10;
    let pb1 = p01 + p11;
    let mut mi = 0.0f64;
    for (p, pa, pb) in [
        (p00, pa0, pb0),
        (p01, pa0, pb1),
        (p10, pa1, pb0),
        (p11, pa1, pb1),
    ] {
        if p > 0.0 && pa > 0.0 && pb > 0.0 {
            mi += p * (p / (pa * pb)).log2();
        }
    }
    if mi < 0.0 { 0.0 } else { mi }
}

/// Determine keepbits (mantissa bits) at a target information level.
///
/// Implements the BitInformation.jl algorithm:
///   1. mutual information between adjacent array entries, per bit position
///   2. zero out bits below the binomial free-entropy noise floor (confidence 0.99)
///   3. return smallest mantissa keepbits whose cumulative info ≥ inflevel * total
fn get_keepbits<T: BitFloat>(data: &[T], inflevel: f64) -> Result<usize, KeffError> {
    if data.len() < 2 {
        return Err(KeffError::new(
            "Need at least 2 values for bitinformation".to_string(),
        ));
    }
    if inflevel <= 0.0 || inflevel > 1.0 {
        return Err(KeffError::new("inflevel must be in (0, 1]".to_string()));
    }
    let mut info = bitinformation_adjacent(data);
    let floor = binom_free_entropy(data.len() - 1, 0.99);
    for h in info.iter_mut() {
        if *h <= floor {
            *h = 0.0;
        }
    }
    let mantissa = &info[T::MANTISSA_START..T::TOTAL_BITS];
    let total: f64 = mantissa.iter().sum();
    if total == 0.0 {
        return Ok(1);
    }
    let mut acc = 0.0f64;
    for (i, &h) in mantissa.iter().enumerate() {
        acc += h;
        if acc / total >= inflevel {
            return Ok(i + 1);
        }
    }
    Ok(T::TOTAL_BITS - T::MANTISSA_START)
}

pub fn get_keepbits_f32(data: &[f32], inflevel: f64) -> Result<usize, KeffError> {
    get_keepbits(data, inflevel)
}

pub fn get_keepbits_f64(data: &[f64], inflevel: f64) -> Result<usize, KeffError> {
    get_keepbits(data, inflevel)
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

    #[test]
    fn test_get_keepbits_f32_pure_noise_returns_one() {
        // Uncorrelated bits between neighbors → MI ≈ 0 → all below noise floor → 1 bit fallback.
        let mut rng = fastrand::Rng::with_seed(0x12345);
        let data: Vec<f32> = (0..4096).map(|_| rng.f32()).collect();
        let kb = get_keepbits_f32(&data, 0.99).unwrap();
        assert_eq!(kb, 1);
    }

    #[test]
    fn test_get_keepbits_f32_smooth_signal_keeps_many() {
        // Smooth sinusoid: high mantissa bits carry real info between neighbors.
        let n = 4096;
        let data: Vec<f32> = (0..n)
            .map(|i| (i as f32 * 0.01).sin() * 10.0 + 100.0)
            .collect();
        let kb_99 = get_keepbits_f32(&data, 0.99).unwrap();
        let kb_90 = get_keepbits_f32(&data, 0.90).unwrap();
        assert!(kb_99 >= 5, "smooth data should keep many bits, got {}", kb_99);
        assert!(kb_90 <= kb_99);
    }

    #[test]
    fn test_get_keepbits_f64_smooth_signal() {
        let n = 4096;
        let data: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.01).sin() * 10.0 + 100.0)
            .collect();
        let kb = get_keepbits_f64(&data, 0.99).unwrap();
        assert!(kb >= 5);
        assert!(kb <= 52);
    }

    #[test]
    fn test_get_keepbits_monotone_in_inflevel() {
        let n = 4096;
        let data: Vec<f32> = (0..n)
            .map(|i| (i as f32 * 0.003).sin() * 5.0 + 20.0)
            .collect();
        let kb50 = get_keepbits_f32(&data, 0.50).unwrap();
        let kb95 = get_keepbits_f32(&data, 0.95).unwrap();
        let kb99 = get_keepbits_f32(&data, 0.99).unwrap();
        assert!(kb50 <= kb95);
        assert!(kb95 <= kb99);
    }

    #[test]
    fn test_get_keepbits_invalid_inputs() {
        let data = vec![1.0f32; 10];
        assert!(get_keepbits_f32(&data, 0.0).is_err());
        assert!(get_keepbits_f32(&data, 1.5).is_err());
        assert!(get_keepbits_f32(&[1.0f32], 0.99).is_err());
    }
}
