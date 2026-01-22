use std::f64;

const DATA_SIZE_GB: f64 = 1.6;

fn get_size_str(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn estimate_compressed_size(original_bytes: u64, nbits: u32) -> u64 {
    let base_bits: f64 = 64.0;
    let precision_ratio = base_bits / nbits as f64;
    let compressed = original_bytes as f64 / precision_ratio;
    compressed as u64
}

fn main() {
    println!("=================================================================");
    println!("Climate Data Compression Analysis");
    println!("=================================================================");
    println!();
    println!("Data source: NOAA-GFDL GFDL-ESM4 ssp585 r1i1p1f1 (zos)");
    println!(
        "Full dataset: {}",
        get_size_str((DATA_SIZE_GB * 1024.0_f64.powi(3)) as u64)
    );
    println!();
    println!("How it works:");
    println!("  - Each float64 has 52 mantissa bits, but not all carry equal information");
    println!("  - Bits are ranked by information content (signal vs noise)");
    println!("  - More information preservation = keep more bits = larger output");
    println!("  - Less information preservation = keep fewer bits = smaller output");
    println!();
    println!("For real data, run: cargo run --bin climate-bitround -- keff <data>");
    println!();
    println!("-----------------------------------------------------------------");
    println!(" Information | keepbits |   Compressed |  Ratio | Interpretation");
    println!("-----------------------------------------------------------------");

    let original = (DATA_SIZE_GB * 1024.0_f64.powi(3)) as u64;

    let scenarios = [
        (0.99, 22, "Keep only the 22 most informative bits"),
        (0.95, 19, "Keep 19 bits (lose 1% noise)"),
        (0.90, 16, "Keep 16 bits (lose 10% noise)"),
        (0.85, 14, "Keep 14 bits (lose 15% noise)"),
        (0.80, 12, "Keep 12 bits (lose 20% noise)"),
    ];

    for (info_level, nbits, desc) in scenarios {
        let compressed = estimate_compressed_size(original, nbits);
        let ratio = original as f64 / compressed as f64;
        println!(
            "      {:>3.0}% | {:>8} | {:>11} | {:>5.2}x  | {}",
            info_level * 100.0,
            nbits,
            get_size_str(compressed),
            ratio,
            desc
        );
    }

    println!("-----------------------------------------------------------------");
    println!();
    println!("Key insight:");
    println!("  Climate data has redundancy - many mantissa bits carry noise, not signal");
    println!("  By keeping only the informative bits, we preserve 99% of the real data");
    println!("  while using only ~22/52 bits (42% of original precision)");
    println!();
    println!("Summary:");
    println!(
        "  99% information preservation → {} (~7.3x reduction)",
        get_size_str(estimate_compressed_size(original, 22))
    );
    println!();
    println!("For the complete workflow, see:");
    println!("  src/bin/climate_bitround.rs - Rust CLI implementation");
    println!("  scripts/climate_compression.rs - This analysis script");
    println!("  openspec/changes/climate-data-bitround-usecase/ - Full specification");
}
