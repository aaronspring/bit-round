use bit_round::bitround::BitroundEncoder;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn encode_f32(c: &mut Criterion) {
    let data: Vec<f32> = (0..10000).map(|_| rand::random::<f32>()).collect();
    let encoder = BitroundEncoder::new(16).unwrap();

    c.bench_function("encode_f32_10k", |b| {
        b.iter(|| encoder.encode_f32(black_box(&data)))
    });
}

fn decode_f32(c: &mut Criterion) {
    let data: Vec<f32> = (0..10000).map(|_| rand::random::<f32>()).collect();
    let encoder = BitroundEncoder::new(16).unwrap();
    let encoded = encoder.encode_f32(&data).unwrap();

    c.bench_function("decode_f32_10k", |b| {
        b.iter(|| encoder.decode_f32(black_box(&encoded)))
    });
}

fn encode_f64(c: &mut Criterion) {
    let data: Vec<f64> = (0..10000).map(|_| rand::random::<f64>()).collect();
    let encoder = BitroundEncoder::new(32).unwrap();

    c.bench_function("encode_f64_10k", |b| {
        b.iter(|| encoder.encode_f64(black_box(&data)))
    });
}

fn decode_f64(c: &mut Criterion) {
    let data: Vec<f64> = (0..10000).map(|_| rand::random::<f64>()).collect();
    let encoder = BitroundEncoder::new(32).unwrap();
    let encoded = encoder.encode_f64(&data).unwrap();

    c.bench_function("decode_f64_10k", |b| {
        b.iter(|| encoder.decode_f64(black_box(&encoded)))
    });
}

criterion_group!(benches, encode_f32, decode_f32, encode_f64, decode_f64);
criterion_main!(benches);
