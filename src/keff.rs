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

    let valid_max_nbits = max_nbits.clamp(1, 24);
    let bits_data: Vec<u32> = data.iter().map(|&x| x.to_bits()).collect();
    let bit_significance = calculate_bit_significance_u32(&bits_data, 24);

    let (keff, nbits_preserved, info_preserved) =
        find_optimal_nbits(&bit_significance, significance_level, valid_max_nbits);

    Ok(KeffResult::new(
        keff,
        nbits_preserved,
        info_preserved,
        bit_significance,
    ))
}

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

    let valid_max_nbits = max_nbits.clamp(1, 53);
    let bits_data: Vec<u64> = data.iter().map(|&x| x.to_bits()).collect();
    let bit_significance = calculate_bit_significance_u64(&bits_data, 53);

    let (keff, nbits_preserved, info_preserved) =
        find_optimal_nbits(&bit_significance, significance_level, valid_max_nbits);

    Ok(KeffResult::new(
        keff,
        nbits_preserved,
        info_preserved,
        bit_significance,
    ))
}

fn calculate_bit_significance_u32(bits: &[u32], total_bits: usize) -> Vec<f64> {
    let n = bits.len() as f64;
    let mut significance: Vec<f64> = vec![0.0; total_bits];

    for bit_idx in 0..total_bits {
        let mask = 1u32 << bit_idx;
        let mut entropy_sum = 0.0f64;

        let ones_count: usize = bits.iter().filter(|&&b| (b & mask) != 0).count();
        let p_ones = ones_count as f64 / n;
        let p_zeros = 1.0 - p_ones;

        if p_ones > 0.0 {
            entropy_sum -= p_ones * p_ones.log2();
        }
        if p_zeros > 0.0 {
            entropy_sum -= p_zeros * p_zeros.log2();
        }

        significance[bit_idx] = if entropy_sum.is_nan() {
            0.0
        } else {
            entropy_sum
        };
    }

    significance
}

fn calculate_bit_significance_u64(bits: &[u64], total_bits: usize) -> Vec<f64> {
    let n = bits.len() as f64;
    let mut significance: Vec<f64> = vec![0.0; total_bits];

    for bit_idx in 0..total_bits {
        let mask = 1u64 << bit_idx;
        let mut entropy_sum = 0.0f64;

        let ones_count: usize = bits.iter().filter(|&&b| (b & mask) != 0).count();
        let p_ones = ones_count as f64 / n;
        let p_zeros = 1.0 - p_ones;

        if p_ones > 0.0 {
            entropy_sum -= p_ones * p_ones.log2();
        }
        if p_zeros > 0.0 {
            entropy_sum -= p_zeros * p_zeros.log2();
        }

        significance[bit_idx] = if entropy_sum.is_nan() {
            0.0
        } else {
            entropy_sum
        };
    }

    significance
}

fn find_optimal_nbits(
    significance: &[f64],
    target_significance: f64,
    max_nbits: usize,
) -> (f64, usize, f64) {
    let total_info: f64 = significance.iter().sum();
    let mut accumulated_info = 0.0f64;
    let mut nbits = 0;

    for (i, &sig) in significance.iter().enumerate().take(max_nbits) {
        accumulated_info += sig;
        nbits = i + 1;
        let info_fraction = accumulated_info / total_info;
        if info_fraction >= target_significance {
            break;
        }
    }

    let info_fraction = if total_info > 0.0 {
        accumulated_info / total_info
    } else {
        0.0
    };

    let keff = accumulated_info;
    (keff, nbits, info_fraction)
}

pub fn apply_bitrounding_f32(data: &[f32], nbits: usize) -> Vec<f32> {
    let nbits = nbits.clamp(1, 24);
    let scale = 2.0_f32.powi(nbits as i32 - 24);
    data.iter()
        .map(|&x| {
            let scaled = x * scale;
            scaled.round() / scale
        })
        .collect()
}

pub fn apply_bitrounding_f64(data: &[f64], nbits: usize) -> Vec<f64> {
    let nbits = nbits.clamp(1, 53);
    let scale = 2.0_f64.powi(nbits as i32 - 53);
    data.iter()
        .map(|&x| {
            let scaled = x * scale;
            scaled.round() / scale
        })
        .collect()
}

pub fn calculate_max_error_f32(nbits: usize) -> f32 {
    if nbits >= 24 {
        0.0
    } else {
        2.0_f32.powi(-(nbits as i32))
    }
}

pub fn calculate_max_error_f64(nbits: usize) -> f64 {
    if nbits >= 53 {
        0.0
    } else {
        2.0_f64.powi(-(nbits as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keff_f32_constant() {
        let data = vec![1.0f32; 1000];
        let result = calculate_keff_f32(&data, 0.99, 24).unwrap();
        assert!(result.keff > 0.0);
        assert!(result.nbits_preserved <= 24);
    }

    #[test]
    fn test_keff_f64_constant() {
        let data = vec![1.0f64; 1000];
        let result = calculate_keff_f64(&data, 0.99, 53).unwrap();
        assert!(result.keff > 0.0);
        assert!(result.nbits_preserved <= 53);
    }

    #[test]
    fn test_keff_empty_data() {
        let data: Vec<f32> = vec![];
        let result = calculate_keff_f32(&data, 0.99, 24);
        assert!(result.is_err());
    }

    #[test]
    fn test_bitrounding_f32() {
        let data = vec![1.234567f32; 10];
        let rounded = apply_bitrounding_f32(&data, 10);
        assert!(rounded.iter().all(|&x| x == rounded[0]));
    }

    #[test]
    fn test_bitrounding_f64() {
        let data = vec![1.23456789012345f64; 10];
        let rounded = apply_bitrounding_f64(&data, 20);
        assert!(rounded.iter().all(|&x| x == rounded[0]));
    }
}
