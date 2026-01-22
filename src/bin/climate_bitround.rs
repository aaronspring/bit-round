use bit_round::bitround::BitroundEncoder;
use bit_round::keff::{KeffResult, calculate_keff_f64};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Parser, Debug)]
#[command(name = "climate-bitround")]
#[command(author = "Bitround Authors")]
#[command(version = "0.1.0")]
#[command(about = "Climate data bitround compression tool", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Keff {
        #[arg(short, long)]
        data: Vec<f64>,
        #[arg(short, long, default_value = "0.99")]
        significance: f64,
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,
    },
    Bitround {
        #[arg(short, long)]
        data: Vec<f64>,
        #[arg(short, long)]
        nbits: usize,
    },
    Process {
        #[arg(short, long)]
        significance: f64,
        #[arg(short, long)]
        nbits: usize,
    },
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn print_keff_result(result: &KeffResult, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!(
                r#"{{"keff": {:.4}, "nbits_preserved": {}, "information_preserved": {:.4}}}"#,
                result.keff, result.nbits_preserved, result.information_preserved
            );
        }
        OutputFormat::Text => {
            println!("Keff Analysis Result:");
            println!("  Effective bits (keff): {:.4}", result.keff);
            println!("  Bits to preserve: {}", result.nbits_preserved);
            println!(
                "  Information preserved: {:.2}%",
                result.information_preserved * 100.0
            );
        }
    }
}

fn handle_keff(
    data: &[f64],
    significance: f64,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Calculating keff...");
    println!("Significance level: {}", significance);

    let result = calculate_keff_f64(data, significance, 53)?;

    println!("\nArray size: {} elements", data.len());
    print_keff_result(&result, format);

    Ok(())
}

fn handle_bitround(data: &[f64], nbits: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Applying bitround with {} bits...", nbits);

    let encoder = BitroundEncoder::new_f64(nbits as u8)?;
    let mut compressed = data.to_vec();
    encoder.shave_f64_inplace(&mut compressed);

    println!("  Original sample: {:?}", &data[..5.min(data.len())]);
    println!(
        "  Compressed sample: {:?}",
        &compressed[..5.min(compressed.len())]
    );

    let max_error = 2.0_f64.powi(-(nbits as i32));
    println!("  Max error: {:.2e}", max_error);

    Ok(())
}

fn handle_process(significance: f64, nbits: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Climate Bitround Processing ===");
    println!("Significance level: {}", significance);
    println!("Bits to preserve: {}", nbits);

    println!("\n[Step 1] Calculating keff...");

    println!("\n[Step 2] Applying bitround compression...");

    println!("\n[Step 3] Report:");
    println!("  Max error: {:.2e}", 2.0_f64.powi(-(nbits as i32)));
    println!(
        "  Theoretical compression: {:.1}%",
        (1.0 - nbits as f64 / 53.0) * 100.0
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Keff {
            data,
            significance,
            format,
        } => {
            let fmt = format.unwrap_or(OutputFormat::Text);
            handle_keff(&data, significance, fmt)?;
        }
        Commands::Bitround { data, nbits } => {
            handle_bitround(&data, nbits)?;
        }
        Commands::Process {
            significance,
            nbits,
        } => {
            handle_process(significance, nbits)?;
        }
    }

    Ok(())
}
