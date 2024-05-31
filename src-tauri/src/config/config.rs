use tauri::api::path::{config_dir, picture_dir};
use crate::config::interface::Config;
use crate::utils::error::{WallResult, Error};
use toml::Table;


impl Config {
    pub fn new(download_path: String) -> Self {
        Self {
            download_path,
        }
    }

    pub fn save(&self) -> WallResult<()> {
        let config_path = config_dir().ok_or(Error::new("config path error"))?;
        let config_path = config_path.join("config.toml");
        std::fs::create_dir_all(&self.download_path)?;
        let config = Self::new(self.download_path.clone());
        let toml_config = toml::to_string(&config)?;
        std::fs::write(config_path, toml_config)?;
        Ok(())
    }

    // 从本机配置文件读取配置
    pub fn load() -> WallResult<Self> {
        let config_path_dir = config_dir().ok_or(Error::new("config path error"))?;
        let config_path = config_path_dir.join("config.toml");
        // 判断config_path是否存在
        if !config_path.exists() {
            std::fs::create_dir_all(Self::default().download_path)?;
            let config = Self::new(Self::default().download_path.to_string());
            let toml_config = toml::to_string(&config)?;
            std::fs::write(config_path, toml_config)?;
            return Ok(Self::default());
        }
        let config = std::fs::read_to_string(&config_path)?;
        let config: Table = toml::from_str(&config)?;
        Ok(Self::new(config["download_path"].as_str().unwrap().to_string()))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            download_path: picture_dir().unwrap().to_str().unwrap().to_string(),
        }
    }
}