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

    pub fn list_arrays(&self) -> Result<Vec<String>, ZarrError> {
        let mut arrays = Vec::new();
        if self.path.exists() {
            for entry in std::fs::read_dir(&self.path).map_err(|e| ZarrError::new(e.to_string()))? {
                let entry = entry.map_err(|e| ZarrError::new(e.to_string()))?;
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.starts_with('.') {
                        arrays.push(name);
                    }
                }
            }
        }
        Ok(arrays)
    }
}

pub struct ZarrArray {
    data: Vec<f64>,
    shape: Vec<usize>,
}

impl ZarrArray {
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn read_f32(&self) -> Result<Vec<f32>, ZarrError> {
        Ok(self.data.iter().map(|&x| x as f32).collect())
    }

    pub fn read_f64(&self) -> Result<Vec<f64>, ZarrError> {
        Ok(self.data.clone())
    }
}

pub struct ZarrWriter {
    path: PathBuf,
}

impl ZarrWriter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn create_array_f32(
        &self,
        array_name: &str,
        _shape: &[usize],
        _chunk_shape: Option<&[usize]>,
        _codec: Option<()>,
    ) -> Result<(), ZarrError> {
        let array_path = self.path.join(array_name);
        if !array_path.exists() {
            std::fs::create_dir_all(&array_path).map_err(|e| ZarrError::new(e.to_string()))?;
        }
        Ok(())
    }

    pub fn create_array_f64(
        &self,
        array_name: &str,
        _shape: &[usize],
        _chunk_shape: Option<&[usize]>,
        _codec: Option<()>,
    ) -> Result<(), ZarrError> {
        let array_path = self.path.join(array_name);
        if !array_path.exists() {
            std::fs::create_dir_all(&array_path).map_err(|e| ZarrError::new(e.to_string()))?;
        }
        Ok(())
    }

    pub fn write_array_f32(&self, _array_name: &str, _data: &[f32]) -> Result<(), ZarrError> {
        Ok(())
    }

    pub fn write_array_f64(&self, _array_name: &str, _data: &[f64]) -> Result<(), ZarrError> {
        Ok(())
    }
}

pub fn get_directory_size(path: &PathBuf) -> Result<u64, std::io::Error> {
    let mut total = 0u64;
    if path.exists() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                total += get_directory_size(&entry.path())?;
            } else {
                total += metadata.len();
            }
        }
    }
    Ok(total)
}

pub fn format_size(bytes: u64) -> String {
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
