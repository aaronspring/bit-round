use bit_round::bitround::BitroundEncoder;
use std::time::Instant;

fn main() {
    let size = 1000;
    let nbits = 11;
    let n_warmup = 3;
    let n_iterations = 10;

    println!("Rust bitround benchmark");
    println!("=======================");
    println!(
        "Array size: {}x{} = {} Float32 elements",
        size,
        size,
        size * size
    );
    println!("nbits: {}", nbits);
    println!();

    // Generate random data
    let mut rng = fastrand::Rng::new();
    rng.seed(42);
    let mut data: Vec<f32> = (0..size * size).map(|_| rng.f32()).collect();

    let encoder = BitroundEncoder::new(nbits as u8).unwrap();

    // Warmup
    println!("Warming up...");
    for _ in 0..n_warmup {
        let _ = encoder.encode_f32(&data).unwrap();
    }

    // Benchmark
    println!("Running benchmark ({} iterations)...", n_iterations);
    let mut times = Vec::new();
    for i in 0..n_iterations {
        let start = Instant::now();
        let _ = encoder.encode_f32(&data).unwrap();
        let end = Instant::now();
        let t = end - start;
        let t_ms = t.as_secs_f64() * 1000.0;
        times.push(t_ms);
        println!("  Iteration {}: {:.4} ms", i + 1, t_ms);
    }

    // Calculate statistics
    let mean_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
    let std_time =
        (times.iter().map(|t| (t - mean_time).powi(2)).sum::<f64>() / times.len() as f64).sqrt();
    let min_time = times
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_time = times
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Calculate throughput
    let data_mb = (size * size * 4) as f64 / (1024.0 * 1024.0); // 4 bytes per float32
    let throughput_mb_s = data_mb / (mean_time / 1000.0);

    println!("\nResults:");
    println!(
        "  Mean:   {:.4} ms ({:.2} MB/s)",
        mean_time, throughput_mb_s
    );
    println!("  Std:    {:.4} ms", std_time);
    println!("  Min:    {:.4} ms", min_time);
    println!("  Max:    {:.4} ms", max_time);
    println!();

    // Return mean time for comparison
    println!("Mean time (ms): {}", mean_time);
}
