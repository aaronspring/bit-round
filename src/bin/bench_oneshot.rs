//! One-shot bitround for a single 3D array. Prints `encode_seconds=<float>`.
//! Mirrors scripts/oneshot_python.py and scripts/oneshot_julia.jl for fair
//! cross-implementation memory + time measurement via /usr/bin/time -l.

use bit_round::bitround::BitroundEncoder;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let edge: usize = args[1].parse().expect("edge size");
    let keepbits: u8 = args[2].parse().expect("keepbits");

    // Same distribution as Python/Julia oneshots; bytes will differ from
    // numpy/Julia RNG but element count and footprint match.
    let mut rng = fastrand::Rng::with_seed(42);
    let n = edge * edge * edge;
    let data: Vec<f32> = (0..n).map(|_| 273.0 + rng.f32() * 20.0).collect();

    let encoder = BitroundEncoder::new(keepbits).unwrap();
    let _ = encoder.encode_f32(&data).unwrap(); // warmup
    let t0 = Instant::now();
    let out = encoder.encode_f32(&data).unwrap();
    let elapsed = t0.elapsed().as_secs_f64();
    println!("encode_seconds={}", elapsed);
    println!("out_bytes={}", out.len() * std::mem::size_of::<u32>());
}
