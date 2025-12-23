use std::io::Write;
use http_cache_reqwest::CacheMode;
use tauri::State;
use crate::api::http::Context;
use crate::config::interface::Config;
use crate::utils::error::WallResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Download {
    pub url: String,
    pub file_name: String,
}


impl Download {
    pub fn new(url: String, file_name: String) -> Self {
        Self {
            url,
            file_name
        }
    }

    pub async fn save(&self, context: State<'_, Context>) -> WallResult<()> {
        // 加载配置并确保路径存在
        let config = Config::load()?;
        let download_path = std::path::Path::new(&config.download_path);
        
        // 确保下载目录存在
        if !download_path.exists() {
            std::fs::create_dir_all(download_path)?;
        }
        
        let file_path = download_path.join(&self.file_name);
        
        // 检查文件是否已存在
        if file_path.exists() {
            println!("文件已存在，将被覆盖: {:?}", file_path);
        }
        
        // 发送网络请求
        let client = context.http_client();
        let response = client.get(&self.url)
            .with_extension(CacheMode::NoStore)
            .send()
            .await?;
        
        // 检查响应状态
        if !response.status().is_success() {
            return Err(crate::utils::error::Error::new(
                &format!("下载失败: HTTP {}", response.status())
            ));
        }
        
        // 创建文件
        let mut file = std::fs::File::create(&file_path)
            .map_err(|e| crate::utils::error::Error::new(
                &format!("创建文件失败: {}", e)
            ))?;
        
        // 写入文件内容
        let bytes = response.bytes().await?;
        file.write_all(&bytes)
            .map_err(|e| crate::utils::error::Error::new(
                &format!("写入文件失败: {}", e)
            ))?;
        
        println!("下载成功: {:?}", file_path);
        Ok(())
    }
}
