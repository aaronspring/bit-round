use bit_round::bitround::BitroundEncoder;
use bit_round::keff::calculate_keff_f64;
use bit_round::zarr::{format_size, get_directory_size};
use std::path::Path;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Arc;
use zarrs::array::codec::ZstdCodec;
use zarrs::array::data_type::{Float32DataType, Float64DataType};
use zarrs::array::{Array, ArrayBuilder, ArrayBuilderFillValue, ArraySubset, DataType};
use zarrs::array::data_type;
use zarrs::filesystem::FilesystemStore;
use zarrs::storage::ReadableWritableListableStorage;

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
    /// Show info about a Zarr store
    Info {
        /// Path to the Zarr store
        path: PathBuf,
    },
    /// Compress a Zarr store with bitround + zstd
    Compress {
        /// Input Zarr store path
        #[arg(short, long)]
        input: PathBuf,
        /// Output Zarr store path
        #[arg(short, long)]
        output: PathBuf,
        /// Significance level for keff calculation (0.0-1.0)
        #[arg(short, long, default_value = "0.99")]
        significance: f64,
        /// Zstd compression level (1-22)
        #[arg(short, long, default_value = "19")]
        level: i32,
        /// Override number of bits to preserve (skip keff calculation)
        #[arg(short, long)]
        nbits: Option<usize>,
        /// Name of the array to compress (if store contains multiple arrays)
        #[arg(short, long)]
        array: Option<String>,
    },
}

fn is_float32(dtype: &DataType) -> bool {
    dtype.is::<Float32DataType>()
}

fn is_float64(dtype: &DataType) -> bool {
    dtype.is::<Float64DataType>()
}

fn dtype_name(dtype: &DataType) -> &'static str {
    if is_float32(dtype) {
        "float32"
    } else if is_float64(dtype) {
        "float64"
    } else {
        "unknown"
    }
}

fn handle_info(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Zarr store: {}", path.display());

    if !path.exists() {
        return Err("Path does not exist".into());
    }

    let size = get_directory_size(Path::new(path))?;
    println!("Total size: {}", format_size(size));

    let store = Arc::new(FilesystemStore::new(path)?);

    println!("\nArrays:");
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }

            let array_path = format!("/{}", name);
            match Array::open(store.clone(), &array_path) {
                Ok(array) => {
                    let shape = array.shape();
                    let dtype = array.data_type();
                    let sub_size = get_directory_size(entry_path.as_path())?;
                    println!(
                        "  {}: shape={:?}, dtype={}, size={}",
                        name,
                        shape,
                        dtype_name(dtype),
                        format_size(sub_size)
                    );
                }
                Err(_) => {
                    let sub_size = get_directory_size(entry_path.as_path())?;
                    println!("  {} (not a valid array): {}", name, format_size(sub_size));
                }
            }
        }
    }

    if let Ok(root_array) = Array::open(store.clone(), "/") {
        let shape = root_array.shape();
        let dtype = root_array.data_type();
        println!("\nRoot array: shape={:?}, dtype={}", shape, dtype_name(dtype));
    }

    Ok(())
}

fn find_array_path(store: &Arc<FilesystemStore>, path: &PathBuf, array_name: Option<&String>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(name) = array_name {
        return Ok(format!("/{}", name));
    }

    if Array::open(store.clone(), "/").is_ok() {
        return Ok("/".to_string());
    }

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let array_path = format!("/{}", name);
            if Array::open(store.clone(), &array_path).is_ok() {
                return Ok(array_path);
            }
        }
    }

    Err("No valid array found in Zarr store".into())
}

fn read_array_data(array: &Array<FilesystemStore>) -> Result<(Vec<f64>, Vec<u64>, bool), Box<dyn std::error::Error>> {
    let shape = array.shape().to_vec();
    let subset = ArraySubset::new_with_shape(shape.clone());
    let dtype = array.data_type();
    
    let is_f32 = is_float32(dtype);
    let is_f64 = is_float64(dtype);
    
    if !is_f32 && !is_f64 {
        return Err(format!("Unsupported data type: {}. Only float32 and float64 are supported.", dtype_name(dtype)).into());
    }

    let data: Vec<f64> = if is_f32 {
        let values: Vec<f32> = array.retrieve_array_subset(&subset)?;
        values.into_iter().map(|v| v as f64).collect()
    } else {
        array.retrieve_array_subset(&subset)?
    };

    Ok((data, shape, is_f32))
}

fn handle_compress(
    input: &PathBuf,
    output: &PathBuf,
    significance: f64,
    level: i32,
    nbits_override: Option<usize>,
    array_name: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Climate Bitround Compression ===");
    println!("Input:  {}", input.display());
    println!("Output: {}", output.display());
    println!("Significance level: {}", significance);
    println!("Zstd level: {}", level);
    if let Some(n) = nbits_override {
        println!("NBits override: {}", n);
    }
    println!();

    if !input.exists() {
        return Err("Input path does not exist".into());
    }

    let original_size = get_directory_size(input.as_path())?;
    println!("[Step 1] Original size: {}", format_size(original_size));

    println!("\n[Step 2] Opening input Zarr store...");
    let input_store = Arc::new(FilesystemStore::new(input)?);
    let array_path = find_array_path(&input_store, input, array_name)?;
    println!("  Found array at: {}", array_path);

    let input_array = Array::open(input_store.clone(), &array_path)?;
    let shape = input_array.shape().to_vec();
    let dtype = input_array.data_type().clone();
    let dtype_str = dtype_name(&dtype);
    println!("  Shape: {:?}", shape);
    println!("  Data type: {}", dtype_str);

    println!("\n[Step 3] Reading array data...");
    let (data, _, is_f32) = read_array_data(&input_array)?;
    println!("  Read {} elements", data.len());

    let nbits = if let Some(n) = nbits_override {
        n
    } else {
        println!("\n[Step 4] Calculating keff...");
        let keff_result = calculate_keff_f64(&data, significance, 53)?;
        println!("  keff: {:.4}", keff_result.keff);
        println!("  Bits to preserve: {}", keff_result.nbits_preserved);
        println!(
            "  Information preserved: {:.2}%",
            keff_result.information_preserved * 100.0
        );
        keff_result.nbits_preserved
    };

    println!(
        "\n[Step 5] Applying bitround compression ({} bits)...",
        nbits
    );
    let mut compressed_data = data;
    
    if is_f32 {
        let encoder = BitroundEncoder::new_f32(nbits as u8)?;
        let mut f32_data: Vec<f32> = compressed_data.iter().map(|&v| v as f32).collect();
        encoder.shave_f32_inplace(&mut f32_data);
        compressed_data = f32_data.iter().map(|&v| v as f64).collect();
    } else {
        let encoder = BitroundEncoder::new_f64(nbits as u8)?;
        encoder.shave_f64_inplace(&mut compressed_data);
    }
    println!("  Applied bitround");

    println!("\n[Step 6] Writing compressed Zarr to output...");
    
    if output.exists() {
        std::fs::remove_dir_all(output)?;
    }
    std::fs::create_dir_all(output)?;

    let output_store: ReadableWritableListableStorage = Arc::new(FilesystemStore::new(output)?);

    zarrs::group::GroupBuilder::new()
        .build(output_store.clone(), "/")?
        .store_metadata()?;

    let chunk_shape: Vec<u64> = shape.iter().map(|&s| s.min(64)).collect();
    let zstd_codec = ZstdCodec::new(level, false);

    let output_array = if is_f32 {
        let fill_value: ArrayBuilderFillValue = f32::NAN.into();
        ArrayBuilder::new(
            shape.clone(),
            chunk_shape,
            data_type::float32(),
            fill_value,
        )
        .bytes_to_bytes_codecs(vec![Arc::new(zstd_codec)])
        .build(output_store.clone(), "/data")?
    } else {
        let fill_value: ArrayBuilderFillValue = f64::NAN.into();
        ArrayBuilder::new(
            shape.clone(),
            chunk_shape,
            data_type::float64(),
            fill_value,
        )
        .bytes_to_bytes_codecs(vec![Arc::new(zstd_codec)])
        .build(output_store.clone(), "/data")?
    };

    output_array.store_metadata()?;

    let subset = ArraySubset::new_with_shape(shape.clone());
    if is_f32 {
        let f32_data: Vec<f32> = compressed_data.iter().map(|&v| v as f32).collect();
        output_array.store_array_subset(&subset, &f32_data)?;
    } else {
        output_array.store_array_subset(&subset, &compressed_data)?;
    }

    let compressed_size = get_directory_size(output.as_path())?;
    let ratio = original_size as f64 / compressed_size as f64;

    println!("\n=== Results ===");
    println!("  Original size:    {}", format_size(original_size));
    println!("  Compressed size:  {}", format_size(compressed_size));
    println!("  Compression ratio: {:.2}x", ratio);
    println!(
        "  Space saved: {:.1}%",
        (1.0 - compressed_size as f64 / original_size as f64) * 100.0
    );
    println!(
        "  Max relative error ({} bits): {:.2e}",
        nbits,
        2.0_f64.powi(-(nbits as i32))
    );

    Ok(())
}

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Commands::Info { path } => handle_info(&path),
        Commands::Compress {
            input,
            output,
            significance,
            level,
            nbits,
            array,
        } => handle_compress(&input, &output, significance, level, nbits, array.as_ref()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
