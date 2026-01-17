use std::fs::File;
use std::io::Write;

fn main() {
    std::fs::create_dir_all("testdata/inputs").unwrap();

    const ARRAY_SIZE: usize = 1000;

    let zeros_f32: Vec<f32> = vec![0.0; ARRAY_SIZE];
    let mut file = File::create("testdata/inputs/zeros_f32.bin").unwrap();
    for &val in &zeros_f32 {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/zeros_f32.bin");

    let constants_f32: Vec<f32> = vec![1.5; ARRAY_SIZE];
    let mut file = File::create("testdata/inputs/constants_f32.bin").unwrap();
    for &val in &constants_f32 {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/constants_f32.bin");

    let mut rng = oorandom::Rand32::new(42);
    let random_f32: Vec<f32> = (0..ARRAY_SIZE).map(|_| rng.rand_float()).collect();
    let mut file = File::create("testdata/inputs/random_f32.bin").unwrap();
    for &val in &random_f32 {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/random_f32.bin");

    let edge_f32: Vec<f32> = vec![
        0.0_f32,
        -0.0,
        1.0,
        -1.0,
        f32::MAX,
        f32::MIN,
        f32::EPSILON,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    let subnormal = f32::from_bits(1);
    let mut edge_data = edge_f32.to_vec();
    edge_data.extend(std::iter::repeat(subnormal).take(10));
    let rest: Vec<f32> = (0..80).map(|_| rng.rand_float()).collect();
    edge_data.extend(rest);
    let mut file = File::create("testdata/inputs/edge_f32.bin").unwrap();
    for &val in &edge_data {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/edge_f32.bin");

    let zeros_f64: Vec<f64> = vec![0.0; ARRAY_SIZE];
    let mut file = File::create("testdata/inputs/zeros_f64.bin").unwrap();
    for &val in &zeros_f64 {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/zeros_f64.bin");

    let constants_f64: Vec<f64> = vec![1.5; ARRAY_SIZE];
    let mut file = File::create("testdata/inputs/constants_f64.bin").unwrap();
    for &val in &constants_f64 {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/constants_f64.bin");

    let random_f64: Vec<f64> = (0..ARRAY_SIZE).map(|_| rng.rand_float() as f64).collect();
    let mut file = File::create("testdata/inputs/random_f64.bin").unwrap();
    for &val in &random_f64 {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/random_f64.bin");

    let edge_f64: Vec<f64> = vec![
        0.0_f64,
        -0.0,
        1.0,
        -1.0,
        f64::MAX,
        f64::MIN,
        f64::EPSILON,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    let subnormal = f64::from_bits(1);
    let mut edge_data = edge_f64.to_vec();
    edge_data.extend(std::iter::repeat(subnormal).take(10));
    let rest: Vec<f64> = (0..80).map(|_| rng.rand_float() as f64).collect();
    edge_data.extend(rest);
    let mut file = File::create("testdata/inputs/edge_f64.bin").unwrap();
    for &val in &edge_data {
        file.write_all(&val.to_le_bytes()).unwrap();
    }
    println!("Created testdata/inputs/edge_f64.bin");

    println!("\nAll test input files generated successfully!");
}
