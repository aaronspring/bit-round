use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use bit_round::bitround::BitroundEncoder;

const TESTDATA_DIR: &str = "testdata";

fn load_f32_from_binary(filepath: &Path) -> Vec<f32> {
    let mut file = File::open(filepath).expect(&format!("Failed to open {}", filepath.display()));
    let metadata = file.metadata().expect("Failed to get metadata");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read_exact(&mut buffer).expect("Failed to read file");

    let num_floats = buffer.len() / 4;
    let mut result = Vec::with_capacity(num_floats);
    for chunk in buffer.chunks_exact(4) {
        let bits = u32::from_le_bytes(chunk.try_into().unwrap());
        result.push(f32::from_bits(bits));
    }
    result
}

fn load_f64_from_binary(filepath: &Path) -> Vec<f64> {
    let mut file = File::open(filepath).expect(&format!("Failed to open {}", filepath.display()));
    let metadata = file.metadata().expect("Failed to get metadata");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read_exact(&mut buffer).expect("Failed to read file");

    let num_floats = buffer.len() / 8;
    let mut result = Vec::with_capacity(num_floats);
    for chunk in buffer.chunks_exact(8) {
        let bits = u64::from_le_bytes(chunk.try_into().unwrap());
        result.push(f64::from_bits(bits));
    }
    result
}

fn load_u32_from_binary(filepath: &Path) -> Vec<u32> {
    let mut file = File::open(filepath).expect(&format!("Failed to open {}", filepath.display()));
    let metadata = file.metadata().expect("Failed to get metadata");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read_exact(&mut buffer).expect("Failed to read file");

    let num_u32s = buffer.len() / 4;
    let mut result = Vec::with_capacity(num_u32s);
    for chunk in buffer.chunks_exact(4) {
        result.push(u32::from_le_bytes(chunk.try_into().unwrap()));
    }
    result
}

fn load_u64_from_binary(filepath: &Path) -> Vec<u64> {
    let mut file = File::open(filepath).expect(&format!("Failed to open {}", filepath.display()));
    let metadata = file.metadata().expect("Failed to get metadata");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read_exact(&mut buffer).expect("Failed to read file");

    let num_u64s = buffer.len() / 8;
    let mut result = Vec::with_capacity(num_u64s);
    for chunk in buffer.chunks_exact(8) {
        result.push(u64::from_le_bytes(chunk.try_into().unwrap()));
    }
    result
}

fn save_u32_to_binary(data: &[u32], filepath: &Path) {
    let mut file =
        File::create(filepath).expect(&format!("Failed to create {}", filepath.display()));
    for &value in data {
        file.write_all(&value.to_le_bytes())
            .expect("Failed to write");
    }
}

fn save_u64_to_binary(data: &[u64], filepath: &Path) {
    let mut file =
        File::create(filepath).expect(&format!("Failed to create {}", filepath.display()));
    for &value in data {
        file.write_all(&value.to_le_bytes())
            .expect("Failed to write");
    }
}

#[test]
fn test_f32_encoding_python_zeros_nbits_16() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/zeros_f32.bin");
    let expected_path = testdata_dir.join("python/zeros_f32_nbits16.bin");

    let input = load_f32_from_binary(&input_path);
    let expected = load_u32_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f32(16).unwrap();
    let result = encoder.encode_f32(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#010x}, got {:#010x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f32_encoding_python_zeros_nbits_8() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/zeros_f32.bin");
    let expected_path = testdata_dir.join("python/zeros_f32_nbits8.bin");

    let input = load_f32_from_binary(&input_path);
    let expected = load_u32_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f32(8).unwrap();
    let result = encoder.encode_f32(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#010x}, got {:#010x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f32_encoding_python_zeros_nbits_24() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/zeros_f32.bin");
    let expected_path = testdata_dir.join("python/zeros_f32_nbits24.bin");

    let input = load_f32_from_binary(&input_path);
    let expected = load_u32_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f32(24).unwrap();
    let result = encoder.encode_f32(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#010x}, got {:#010x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f32_encoding_python_constants_nbits_16() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/constants_f32.bin");
    let expected_path = testdata_dir.join("python/constants_f32_nbits16.bin");

    let input = load_f32_from_binary(&input_path);
    let expected = load_u32_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f32(16).unwrap();
    let result = encoder.encode_f32(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#010x}, got {:#010x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f32_encoding_python_random_nbits_16() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/random_f32.bin");
    let expected_path = testdata_dir.join("python/random_f32_nbits16.bin");

    let input = load_f32_from_binary(&input_path);
    let expected = load_u32_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f32(16).unwrap();
    let result = encoder.encode_f32(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#010x}, got {:#010x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f32_encoding_python_edge_nbits_16() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/edge_f32.bin");
    let expected_path = testdata_dir.join("python/edge_f32_nbits16.bin");

    let input = load_f32_from_binary(&input_path);
    let expected = load_u32_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f32(16).unwrap();
    let result = encoder.encode_f32(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#010x}, got {:#010x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f64_encoding_python_zeros_nbits_32() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/zeros_f64.bin");
    let expected_path = testdata_dir.join("python/zeros_f64_nbits32.bin");

    let input = load_f64_from_binary(&input_path);
    let expected = load_u64_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f64(32).unwrap();
    let result = encoder.encode_f64(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#018x}, got {:#018x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f64_encoding_python_random_nbits_32() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/random_f64.bin");
    let expected_path = testdata_dir.join("python/random_f64_nbits32.bin");

    let input = load_f64_from_binary(&input_path);
    let expected = load_u64_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f64(32).unwrap();
    let result = encoder.encode_f64(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#018x}, got {:#018x}",
            i, exp, actual
        );
    }
}

#[test]
fn test_f64_encoding_python_edge_nbits_32() {
    let testdata_dir = Path::new(TESTDATA_DIR);
    let input_path = testdata_dir.join("inputs/edge_f64.bin");
    let expected_path = testdata_dir.join("python/edge_f64_nbits32.bin");

    let input = load_f64_from_binary(&input_path);
    let expected = load_u64_from_binary(&expected_path);

    let encoder = BitroundEncoder::new_f64(32).unwrap();
    let result = encoder.encode_f64(&input).unwrap();

    assert_eq!(result.len(), expected.len());
    for (i, (actual, exp)) in result.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            actual, exp,
            "Mismatch at index {}: expected {:#018x}, got {:#018x}",
            i, exp, actual
        );
    }
}
