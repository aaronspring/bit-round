use bit_round::bitround::BitroundEncoder;
use std::collections::HashMap;
use std::time::Instant;

fn get_machine_specs() -> HashMap<String, String> {
    let mut specs = HashMap::new();
    specs.insert("computer_family".to_string(), "Unknown".to_string());
    specs.insert("cpu_model".to_string(), "Unknown".to_string());
    specs.insert("cpu_cores".to_string(), "Unknown".to_string());
    specs.insert("ram_gb".to_string(), "Unknown".to_string());
    specs.insert(
        "os".to_string(),
        format!("{} {}", std::env::consts::OS, std::env::consts::ARCH),
    );

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("uname -m")
        .output();
    if let Ok(o) = output {
        if let Ok(s) = String::from_utf8(o.stdout) {
            specs.insert("computer_family".to_string(), s.trim().to_string());
        }
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("nproc 2>/dev/null || sysctl -n hw.physicalcpu 2>/dev/null || echo 1")
        .output();
    if let Ok(o) = output {
        if let Ok(s) = String::from_utf8(o.stdout) {
            specs.insert("cpu_cores".to_string(), format!("{} cores", s.trim()));
        }
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("getconf _NPROCESSORS_ONLN 2>/dev/null || echo 1")
        .output();
    if let Ok(o) = output {
        if let Ok(s) = String::from_utf8(o.stdout) {
            specs.insert("cpu_cores".to_string(), format!("{} cores", s.trim()));
        }
    }

    specs
}

fn generate_random_3d_array(edge_size: usize, seed: u64) -> Vec<f32> {
    let mut rng = fastrand::Rng::new();
    rng.seed(seed);
    let n_elements = edge_size * edge_size * edge_size;
    (0..n_elements)
        .map(|_| 273.0 + rng.f32() as f32 * 20.0)
        .collect()
}

fn time_encode_only(data: &[f32], encoder: &BitroundEncoder, n_iterations: usize) -> Vec<f64> {
    let mut times = Vec::with_capacity(n_iterations);
    for _ in 0..n_iterations {
        let start = Instant::now();
        let _ = encoder.encode_f32(data).unwrap();
        let end = Instant::now();
        let t_us = (end - start).as_secs_f64() * 1e6;
        times.push(t_us);
    }
    times
}

fn time_decode_only(encoded: &[u32], encoder: &BitroundEncoder, n_iterations: usize) -> Vec<f64> {
    let mut times = Vec::with_capacity(n_iterations);
    for _ in 0..n_iterations {
        let start = Instant::now();
        let _ = encoder.decode_f32(encoded).unwrap();
        let end = Instant::now();
        let t_us = (end - start).as_secs_f64() * 1e6;
        times.push(t_us);
    }
    times
}

fn calculate_stats(times: &[f64]) -> (f64, f64, f64, f64, f64) {
    let mean: f64 = times.iter().sum::<f64>() / times.len() as f64;
    let variance: f64 = times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64;
    let std = variance.sqrt();
    let min = times
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = times
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let median = {
        let mut sorted: Vec<f64> = times.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() / 2]
    };
    (mean, std, *min, *max, median)
}

fn run_benchmarks(
    nbits: u8,
    n_warmup: usize,
    n_iterations: usize,
) -> HashMap<String, serde_json::Value> {
    let edge_sizes = [1, 10, 100, 1000];
    let mut results = HashMap::new();
    let machine_specs = get_machine_specs();

    eprintln!("Rust bitround benchmark");
    eprintln!("=======================");
    eprintln!(
        "Machine: {}",
        machine_specs
            .get("computer_family")
            .unwrap_or(&"Unknown".to_string())
    );
    eprintln!(
        "CPU: {}",
        machine_specs
            .get("cpu_model")
            .unwrap_or(&"Unknown".to_string())
    );
    eprintln!(
        "Cores: {}",
        machine_specs
            .get("cpu_cores")
            .unwrap_or(&"Unknown".to_string())
    );
    eprintln!(
        "RAM: {}",
        machine_specs
            .get("ram_gb")
            .unwrap_or(&"Unknown".to_string())
    );
    eprintln!(
        "OS: {}",
        machine_specs.get("os").unwrap_or(&"Unknown".to_string())
    );
    eprintln!();
    eprintln!("nbits: {}", nbits);
    eprintln!("warmup iterations: {}", n_warmup);
    eprintln!("measured iterations: {}", n_iterations);
    eprintln!();

    for &edge_size in &edge_sizes {
        let size_str = format!("{}x{}x{}", edge_size, edge_size, edge_size);
        let n_elements = edge_size * edge_size * edge_size;
        let data_mb = n_elements as f64 * 4.0 / (1024.0 * 1024.0);

        eprintln!(
            "Benchmarking {} ({} elements, {:.3} MB)...",
            size_str, n_elements, data_mb
        );

        let data = generate_random_3d_array(edge_size, 42);
        let encoder = BitroundEncoder::new(nbits).unwrap();

        for _ in 0..n_warmup {
            let _ = encoder.encode_f32(&data).unwrap();
        }

        let encode_times = time_encode_only(&data, &encoder, n_iterations);
        let (encode_mean, encode_std, encode_min, encode_max, encode_median) =
            calculate_stats(&encode_times);

        let encoded = encoder.encode_f32(&data).unwrap();
        let decode_times = time_decode_only(&encoded, &encoder, n_iterations);
        let (decode_mean, decode_std, decode_min, decode_max, decode_median) =
            calculate_stats(&decode_times);

        let size_results = serde_json::json!({
            "n_elements": n_elements,
            "encode_us": {
                "mean_us": encode_mean,
                "std_us": encode_std,
                "min_us": encode_min,
                "max_us": encode_max,
                "median_us": encode_median
            },
            "decode_us": {
                "mean_us": decode_mean,
                "std_us": decode_std,
                "min_us": decode_min,
                "max_us": decode_max,
                "median_us": decode_median
            }
        });

        results.insert(size_str, size_results);

        eprintln!("  Encode: {:.2} ± {:.2} us", encode_mean, encode_std);
        eprintln!("  Decode: {:.2} ± {:.2} us", decode_mean, decode_std);
    }

    results
}

fn format_markdown_report(
    results: &HashMap<String, serde_json::Value>,
    machine_specs: &HashMap<String, String>,
) -> String {
    let mut md: Vec<String> = Vec::new();

    md.push("## Rust bitround Benchmark Results".to_string());
    md.push("".to_string());
    md.push("### Machine Specifications".to_string());
    let computer = machine_specs
        .get("computer_family")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    md.push(format!("- Computer: {}", computer));
    let cpu = machine_specs
        .get("cpu_model")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    md.push(format!("- CPU: {}", cpu));
    let cores = machine_specs
        .get("cpu_cores")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    md.push(format!("- Cores: {}", cores));
    let ram = machine_specs
        .get("ram_gb")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    md.push(format!("- RAM: {}", ram));
    let os = machine_specs
        .get("os")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    md.push(format!("- OS: {}", os));
    md.push("".to_string());
    md.push("### Timing Results (microseconds)".to_string());
    md.push("".to_string());
    md.push("| Array Size | Elements | Encode (μs) | Decode (μs) |".to_string());
    md.push("|------------|----------|-------------|-------------|".to_string());

    let mut sizes: Vec<_> = results.keys().collect();
    sizes.sort_by_key(|s| {
        let parts: Vec<usize> = s.split('x').map(|p| p.parse().unwrap()).collect();
        parts[0]
    });

    for size_str in sizes {
        let data = &results[size_str];
        let n_elements = data["n_elements"].as_u64().unwrap();
        let encode_mean = data["encode_us"]["mean_us"].as_f64().unwrap();
        let encode_std = data["encode_us"]["std_us"].as_f64().unwrap();
        let decode_mean = data["decode_us"]["mean_us"].as_f64().unwrap();
        let decode_std = data["decode_us"]["std_us"].as_f64().unwrap();

        md.push(format!(
            "| {} | {} | {:.2} ± {:.2} | {:.2} ± {:.2} |",
            size_str, n_elements, encode_mean, encode_std, decode_mean, decode_std
        ));
    }

    md.push("".to_string());
    md.join("\n")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut nbits = 16;
    let mut n_warmup = 3;
    let mut n_iterations = 10;
    let mut output_json = false;
    let mut output_markdown = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--nbits" if i + 1 < args.len() => {
                nbits = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--warmup" if i + 1 < args.len() => {
                n_warmup = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--iterations" if i + 1 < args.len() => {
                n_iterations = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--json" => {
                output_json = true;
                i += 1;
            }
            "--markdown" => {
                output_markdown = true;
                i += 1;
            }
            _ => i += 1,
        }
    }

    let machine_specs = get_machine_specs();
    let results = run_benchmarks(nbits as u8, n_warmup, n_iterations);

    let results_json = serde_json::json!({
        "rust": results,
        "machine_specs": {
            "computer_family": machine_specs.get("computer_family").cloned().unwrap_or_else(|| "Unknown".to_string()),
            "cpu_model": machine_specs.get("cpu_model").cloned().unwrap_or_else(|| "Unknown".to_string()),
            "cpu_cores": machine_specs.get("cpu_cores").cloned().unwrap_or_else(|| "Unknown".to_string()),
            "ram_gb": machine_specs.get("ram_gb").cloned().unwrap_or_else(|| "Unknown".to_string()),
            "os": machine_specs.get("os").cloned().unwrap_or_else(|| "Unknown".to_string()),
        }
    });

    if output_json {
        println!("{}", serde_json::to_string_pretty(&results_json).unwrap());
    } else if output_markdown {
        eprintln!("{}", format_markdown_report(&results, &machine_specs));
    } else {
        eprintln!("\n{}", format_markdown_report(&results, &machine_specs));
    }
}
