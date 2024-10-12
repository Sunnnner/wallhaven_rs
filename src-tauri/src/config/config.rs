use std::path::PathBuf;
use crate::config::interface::{Config, PathConfig};
use crate::utils::error::{WallResult};
use tauri::State;
use toml::Table;

impl Config {
    pub fn new(download_path: PathBuf) -> Self {
        Self { download_path }
    }

    pub fn save(&self, config_path: State<'_, PathConfig>) -> WallResult<()> {
        let config_file_path = &config_path.config_path.join("config.toml");
        std::fs::create_dir_all(&config_path.download_path)?;
        let config = Self::new(config_path.download_path.clone());
        let toml_config = toml::to_string(&config)?;
        std::fs::write(config_file_path, toml_config)?;
        Ok(())
    }

    // 从本机配置文件读取配置
    pub fn load(config_path: State<'_, PathConfig>) -> WallResult<Self> {
        let config_file_path = &config_path.config_path.join("config.toml");
        // 判断config_path是否存在
        if !config_file_path.exists() {
            std::fs::create_dir_all(&config_path.download_path)?;
            let config = Self::new(config_path.download_path.clone());
            let toml_config = toml::to_string(&config)?;
            std::fs::write(config_file_path, toml_config)?;
            return Ok(config);
        }
        let config = std::fs::read_to_string(&config_file_path)?;
        let config: Table = toml::from_str(&config)?;
        Ok(Self::new(
            PathBuf::from(config["download_path"].as_str().unwrap()),
        ))
    }
}

