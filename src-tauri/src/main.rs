// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use wallhaven_rs::api::http::Context;
use wallhaven_rs::config::interface::Config;
use wallhaven_rs::download::download::Download;
use wallhaven_rs::top::top::TopTag;
use wallhaven_rs::utils::error::{Error, WallResult};
use wallhaven_rs::top::interface::{WallhavenResult};


pub type Result<T> = std::result::Result<T, Error>;

#[tauri::command(rename_all = "snake_case")]
async fn get_top_wallpapers(page: i64, context: State<'_, Context>) -> WallResult<WallhavenResult> {
    let top = TopTag::new(
        111,
        110,
        "6M".to_string(), 
        "toplist".to_string(), 
        "desc".to_string(), 
        0,
        page
    );
    top.get_top_page(context).await
}

fn _construct_new_url(src: &str) -> Result<String> {
    let b_src = src.replace("th.wallhaven.cc/small", "w.wallhaven.cc/full");
    let parts: Vec<&str> = b_src.rsplitn(3, '/').collect();
    let file_name = format!("wallhaven-{}", parts[0]);
    let tag = parts[1];
    Ok(format!("https://w.wallhaven.cc/full/{}/{}", tag, file_name))
}

#[tauri::command(rename_all = "snake_case")]
async fn save_config(path: String) -> WallResult<()> {
    let config = Config::new(path);
    config.save()
}

#[tauri::command(rename_all = "snake_case")]
async fn load_config() -> WallResult<Config> {
    Config::load()
}

#[tauri::command(rename_all = "snake_case")]
async fn download_wallpaper(url: String, file_name: String, context: State<'_, Context>) -> WallResult<()> {
    let download = Download::new(url, file_name);
    download.save(context).await
}

fn main() {
    let context = Context::default();
    tauri::Builder::default()
        .manage(context)
        .invoke_handler(tauri::generate_handler![
            get_top_wallpapers,
            save_config,
            load_config,
            download_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
