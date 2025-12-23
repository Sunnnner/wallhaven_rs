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
        let config_path_dir = config_dir().ok_or(Error::new("获取配置目录失败"))?;
        let config_path = config_path_dir.join("config.toml");
        
        // 判断config_path是否存在
        if !config_path.exists() {
            // 创建默认配置
            let default_config = Self::default();
            if let Err(e) = std::fs::create_dir_all(&default_config.download_path) {
                eprintln!("创建下载目录失败: {}", e);
            }
            let toml_config = toml::to_string(&default_config)?;
            std::fs::write(&config_path, toml_config)?;
            return Ok(default_config);
        }
        
        // 读取配置文件
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| Error::new(&format!("读取配置文件失败: {}", e)))?;
        
        let config: Table = toml::from_str(&config_str)
            .map_err(|e| Error::new(&format!("解析配置文件失败: {}", e)))?;
        
        let download_path = config
            .get("download_path")
            .and_then(|v| v.as_str())
            .ok_or(Error::new("配置文件中缺少 download_path 字段"))?
            .to_string();
        
        Ok(Self::new(download_path))
    }
}

impl Default for Config {
    fn default() -> Self {
        let download_path = picture_dir()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| {
                // 如果获取图片目录失败，使用用户主目录下的 Pictures
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{}/Pictures", home)
            });
        
        Self {
            download_path,
        }
    }
}