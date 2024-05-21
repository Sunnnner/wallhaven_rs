
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct WallhavenResponse {
    pub name: String,
    pub url: String,
    pub full_url: String,
    pub width: i32,
    pub height: i32,
}

impl WallhavenResponse {
    pub fn new(name: String, url: String, full_url: String, width: i32, height: i32) -> Self {
        Self {
            name,
            url,
            full_url,
            width,
            height,
        }
    }
}

pub type WallhavenResult = Vec<WallhavenResponse>;
