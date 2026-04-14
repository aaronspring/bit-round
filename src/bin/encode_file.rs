//! Read a raw little-endian f32 array from a file, bitround with given keepbits,
//! and write the resulting u32 encoded array (little-endian) to an output file.
//!
//! Used by scripts/verify_equivalence.py to check Rust ≡ numcodecs ≡ BitInformation.jl
//! on byte-identical inputs.

use bit_round::bitround::BitroundEncoder;
use std::fs;
use std::io::{Read, Write};

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

    let mut file = fs::File::open(&input_path).expect("open input");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("read input");
    assert!(
        bytes.len() % 4 == 0,
        "input size must be multiple of 4 bytes (f32)"
    );
    let n = bytes.len() / 4;
    let mut data = Vec::with_capacity(n);
    for i in 0..n {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&bytes[i * 4..(i + 1) * 4]);
        data.push(f32::from_le_bytes(buf));
    }

    let encoder = BitroundEncoder::new(keepbits).unwrap();
    let encoded: Vec<u32> = encoder.encode_f32(&data).expect("encode");

    let mut out = fs::File::create(&output_path).expect("create output");
    for u in &encoded {
        out.write_all(&u.to_le_bytes()).expect("write");
    }
}
