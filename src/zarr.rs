use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct ZarrError {
    message: String,
}

impl std::error::Error for ZarrError {}

impl std::fmt::Display for ZarrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ZarrError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

pub struct ZarrReader {
    path: PathBuf,
}

impl ZarrReader {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

pub struct ZarrWriter {
    path: PathBuf,
}

impl ZarrWriter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

pub fn get_zstd_codec(level: i32) -> Result<(), ZarrError> {
    Ok(())
}

pub fn get_blosc_codec(_cname: &str, _clevel: i32, _shuffle: bool) -> Result<(), ZarrError> {
    Ok(())
}

pub fn calculate_compression_ratio(original_size: u64, compressed_size: u64) -> f64 {
    if compressed_size == 0 {
        0.0
    } else {
        original_size as f64 / compressed_size as f64
    }
}
