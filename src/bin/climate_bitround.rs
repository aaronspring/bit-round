use bit_round::bitround::BitroundEncoder;
use bit_round::keff::{KeffResult, calculate_keff_f64};
use bit_round::zarr::{format_size, get_directory_size};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CodecType {
    Zstd,
    Gzip,
    None,
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
    Compress {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, default_value = "0.99")]
        significance: f64,
        #[arg(short, long, value_enum, default_value = "zstd")]
        codec: CodecType,
        #[arg(short, long, default_value = "19")]
        level: i32,
    },
    Info {
        #[arg(short, long)]
        path: PathBuf,
    },
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

fn handle_compress(
    input: &PathBuf,
    output: &PathBuf,
    significance: f64,
    codec: CodecType,
    level: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Climate Bitround Compression ===");
    println!("Input (original):  {}", input.display());
    println!("Output (compressed): {}", output.display());
    println!("Significance level: {}", significance);
    println!("Codec: {:?} (level {})", codec, level);
    println!();

    if !input.exists() {
        return Err("Input path does not exist".into());
    }

    let original_size = get_directory_size(input)?;
    println!("[Step 1] Original size: {}", format_size(original_size));

    println!("\n[Step 2] Reading data from input Zarr...");
    let data = read_zarr_data(input)?;
    println!("  Read {} elements", data.len());

    println!("\n[Step 3] Calculating keff...");
    let keff_result = calculate_keff_f64(&data, significance, 53)?;
    let nbits = keff_result.nbits_preserved;
    println!("  keff: {:.4}", keff_result.keff);
    println!("  Bits to preserve: {}", nbits);
    println!(
        "  Information preserved: {:.2}%",
        keff_result.information_preserved * 100.0
    );

    println!("\n[Step 4] Applying bitround compression...");
    let encoder = BitroundEncoder::new_f64(nbits as u8)?;
    let mut compressed = data;
    encoder.shave_f64_inplace(&mut compressed);
    println!("  Applied bitround with {} bits", nbits);

    println!("\n[Step 5] Writing compressed Zarr to output...");
    write_zarr_data(output, &compressed, nbits, codec, level)?;

    let compressed_size = get_directory_size(output)?;
    let ratio = original_size as f64 / compressed_size as f64;

    println!("\n=== Results ===");
    println!("  Original size:  {}", format_size(original_size));
    println!("  Compressed size: {}", format_size(compressed_size));
    println!("  Compression ratio: {:.2}x", ratio);
    println!(
        "  Space saved: {:.1}%",
        (1.0 - compressed_size as f64 / original_size as f64) * 100.0
    );
    println!(
        "  Max error ({} bits): {:.2e}",
        nbits,
        2.0_f64.powi(-(nbits as i32))
    );

    Ok(())
}

fn read_zarr_data(path: &PathBuf) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let name = entry.file_name();
            if name.to_string_lossy().starts_with('.') {
                continue;
            }

            let zarr_path = entry.path();
            for chunk in std::fs::read_dir(&zarr_path)? {
                let chunk = chunk?;
                if chunk.path().is_file()
                    && chunk
                        .path()
                        .extension()
                        .map(|e| e == "bin")
                        .unwrap_or(false)
                {
                    let content = std::fs::read(&chunk.path())?;
                    let chunk_data: Vec<f64> = content
                        .chunks_exact(8)
                        .map(|chunk| {
                            let mut arr = [0u8; 8];
                            arr.copy_from_slice(chunk);
                            f64::from_le_bytes(arr)
                        })
                        .collect();
                    data.extend(chunk_data);
                }
            }
        }
    }

    if data.is_empty() {
        return Err("No data found in Zarr directory".into());
    }

    Ok(data)
}

fn write_zarr_data(
    path: &PathBuf,
    data: &[f64],
    nbits: usize,
    codec: CodecType,
    level: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    std::fs::create_dir_all(path)?;

    let codec_name = match codec {
        CodecType::Zstd => "zstd",
        CodecType::Gzip => "gzip",
        CodecType::None => "none",
    };

    let metadata = format!(
        r#"{{
  "zarr_format": 3,
  "node_type": "array",
  "shape": [{}],
  "data_type": "float64",
  "chunk_grid": {{
    "type": "regular",
    "chunk_shape": [{}]
  }},
  "codecs": [{{
    "type": "{}",
    "level": {}
  }}],
  "fill_value": 0.0
}}"#,
        data.len(),
        1000,
        codec_name,
        level
    );

    std::fs::write(path.join(".zarray"), metadata)?;

    let chunk_size = 1000;
    for (i, chunk) in data.chunks(chunk_size).enumerate() {
        let chunk_path = path.join(format!("{}", i));
        std::fs::create_dir_all(&chunk_path)?;

        let compressed: Vec<u8> = chunk
            .iter()
            .flat_map(|&v| {
                let encoder = BitroundEncoder::new_f64(nbits as u8).unwrap();
                let masked = (v.to_bits() & encoder.get_keep_mask_f64()) as f64;
                masked.to_le_bytes().to_vec()
            })
            .collect();

        std::fs::write(chunk_path.join("0.bin"), &compressed)?;
    }

    Ok(())
}

fn handle_info(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Zarr directory: {}", path.display());

    if !path.exists() {
        return Err("Path does not exist".into());
    }

    let size = get_directory_size(path)?;
    println!("Size on disk: {}", format_size(size));

    println!("\nSubdirectories (potential arrays):");
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with('.') {
                let sub_size = get_directory_size(&entry.path())?;
                println!("  {}: {}", name, format_size(sub_size));
            }
        }
    }

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
        Commands::Compress {
            input,
            output,
            significance,
            codec,
            level,
        } => {
            handle_compress(&input, &output, significance, codec, level)?;
        }
        Commands::Info { path } => {
            handle_info(&path)?;
        }
    }

    Ok(())
}
