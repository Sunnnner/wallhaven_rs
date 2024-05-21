use std::io::Write;
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
        let download_path = Config::load()?.download_path;
        let download_path = std::path::Path::new(&download_path).join(&self.file_name);
        let client = context.http_client();
        let mut response = client.get(&self.url).send().await?;
        let mut file = std::fs::File::create(download_path)?;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk)?;
        }
        Ok(())
    }
}
