use std::path::PathBuf;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub download_path: PathBuf,
}

// Path: src-tauri/src/config/config.rs

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PathConfig {
    pub download_path: PathBuf,
    pub config_path: PathBuf
}