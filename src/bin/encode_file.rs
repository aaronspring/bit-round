//! Read a raw little-endian f32 array from a file, bitround with given keepbits,
//! and write the resulting u32 encoded array (little-endian) to an output file.
//!
//! Used by scripts/verify_equivalence.py to check Rust ≡ numcodecs ≡ BitInformation.jl
//! on byte-identical inputs.

use bit_round::bitround::BitroundEncoder;
use std::fs;

fn parse_args() -> (String, String, u8) {
    let args: Vec<String> = std::env::args().collect();
    let mut input = None;
    let mut output = None;
    let mut keepbits: u8 = 16;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = Some(args[i + 1].clone());
                i += 2;
            }
            "--output" => {
                output = Some(args[i + 1].clone());
                i += 2;
            }
            "--keepbits" => {
                keepbits = args[i + 1].parse().expect("--keepbits must be u8");
                i += 2;
            }
            _ => {
                eprintln!("unknown arg: {}", args[i]);
                std::process::exit(2);
            }
        }
    }
    (
        input.expect("--input required"),
        output.expect("--output required"),
        keepbits,
    )
}

fn main() {
    let (input_path, output_path, keepbits) = parse_args();

    let bytes = fs::read(&input_path).expect("read input");
    assert!(
        bytes.len() % 4 == 0,
        "input size must be multiple of 4 bytes (f32)"
    );
    let data: Vec<f32> = bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();

    let encoder = BitroundEncoder::new(keepbits).unwrap();
    let encoded: Vec<u32> = encoder.encode_f32(&data).expect("encode");

    let mut out_bytes = Vec::with_capacity(encoded.len() * 4);
    for u in &encoded {
        out_bytes.extend_from_slice(&u.to_le_bytes());
    }
    fs::write(&output_path, &out_bytes).expect("write output");
}
