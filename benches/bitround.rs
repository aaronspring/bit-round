//! Criterion microbench mirroring bench_python.py / bench_julia.jl 3D sizes.
//! For cross-language timing comparison, use `cargo run --release --bin bench`.

use bit_round::bitround::BitroundEncoder;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

const EDGES: [usize; 3] = [10, 100, 1000];
const KEEPBITS: u8 = 16;
const SEED: u64 = 42;

fn temperature_3d(edge: usize, seed: u64) -> Vec<f32> {
    let mut rng = fastrand::Rng::with_seed(seed);
    let n = edge * edge * edge;
    (0..n).map(|_| 273.0 + rng.f32() * 20.0).collect()
}

fn bench_encode(c: &mut Criterion) {
    let encoder = BitroundEncoder::new(KEEPBITS).unwrap();
    let mut group = c.benchmark_group("encode_f32");
    for &edge in &EDGES {
        let data = temperature_3d(edge, SEED);
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(edge), &data, |b, d| {
            b.iter(|| encoder.encode_f32(black_box(d)).unwrap());
        });
    }
    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let encoder = BitroundEncoder::new(KEEPBITS).unwrap();
    let mut group = c.benchmark_group("decode_f32");
    for &edge in &EDGES {
        let data = temperature_3d(edge, SEED);
        let encoded = encoder.encode_f32(&data).unwrap();
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(edge), &encoded, |b, e| {
            b.iter(|| encoder.decode_f32(black_box(e)).unwrap());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
