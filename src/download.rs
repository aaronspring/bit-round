use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct DownloadError {
    message: String,
}

impl std::error::Error for DownloadError {}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl DownloadError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub struct DownloadOptions {
    pub max_retries: u32,
    pub timeout_ms: u64,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout_ms: 300_000,
        }
    }
}

pub fn download_file(
    url: &str,
    output_path: PathBuf,
    options: Option<DownloadOptions>,
) -> Result<PathBuf, DownloadError> {
    let opts = options.unwrap_or_default();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(opts.timeout_ms))
        .build()
        .map_err(|e| DownloadError::new(e.to_string()))?;

    let mut last_error = None;
    for _ in 0..opts.max_retries {
        match client.get(url).send() {
            Ok(response) => {
                if !response.status().is_success() {
                    last_error = Some(DownloadError::new(format!(
                        "HTTP error: {}",
                        response.status()
                    )));
                    continue;
                }
                let bytes = response
                    .bytes()
                    .map_err(|e| DownloadError::new(e.to_string()))?;
                std::fs::write(&output_path, &bytes)
                    .map_err(|e| DownloadError::new(e.to_string()))?;
                return Ok(output_path);
            }
            Err(e) => {
                last_error = Some(DownloadError::new(e.to_string()));
            }
        }
    }

    Err(last_error.unwrap_or_else(|| DownloadError::new("Download failed")))
}
