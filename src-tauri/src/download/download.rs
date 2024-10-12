use crate::api::http::Context;
use crate::config::interface::{Config, PathConfig};
use crate::utils::error::WallResult;
use http_cache_reqwest::CacheMode;
use std::io::Write;
use tauri::State;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Download {
    pub url: String,
    pub file_name: String,
}

impl Download {
    pub fn new(url: String, file_name: String) -> Self {
        Self { url, file_name }
    }

    pub async fn save(&self, context: State<'_, Context>, config_path: State<'_, PathConfig>) -> WallResult<()> {
        let download_path = Config::load(config_path)?.download_path;
        let download_path = download_path.join(&self.file_name);
        let client = context.http_client();
        let mut response = client
            .get(&self.url)
            .with_extension(CacheMode::NoStore)
            .send()
            .await?;
        let mut file = std::fs::File::create(download_path)?;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk)?;
        }
        Ok(())
    }
}
