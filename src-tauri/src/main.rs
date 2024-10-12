// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use tauri::{Manager, State};
use wallhaven_rs::api::http::Context;
use wallhaven_rs::config::interface::{Config, PathConfig};
use wallhaven_rs::download::download::Download;
use wallhaven_rs::tags::interface::TagsResult;
use wallhaven_rs::tags::tags::Tags;
use wallhaven_rs::top::interface::WallhavenResult;
use wallhaven_rs::top::top::TopTag;
use wallhaven_rs::utils::error::{Error, WallResult};

pub type Result<T> = std::result::Result<T, Error>;

#[tauri::command(rename_all = "snake_case")]
async fn get_top_wallpapers(
    params: TopTag,
    context: State<'_, Context>,
) -> WallResult<WallhavenResult> {
    let top = TopTag::new(params);
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
async fn save_config(path: String, config_path: State<'_, PathConfig>) -> WallResult<()> {
    let config = Config::new(PathBuf::from(path));
    config.save(config_path)
}

#[tauri::command(rename_all = "snake_case")]
async fn load_config(config_path: State<'_, PathConfig>) -> WallResult<Config> {
    Config::load(config_path)
}

#[tauri::command(rename_all = "snake_case")]
async fn download_wallpaper(
    url: String,
    file_name: String,
    context: State<'_, Context>,
    config_path: State<'_, PathConfig>
) -> WallResult<()> {
    let download = Download::new(url, file_name);
    download.save(context, config_path).await
}

#[tauri::command(rename_all = "snake_case")]
async fn get_tags(params: Tags, context: State<'_, Context>) -> WallResult<TagsResult> {
    let tags = Tags::new(params);
    tags.get_content(context).await
}

fn main() {
    let context = Context::default();
    tauri::Builder::default()
        .setup(move |app| {
            let config_path = app.path().config_dir().expect("failed to get config dir");
            let default_path = app.path().picture_dir().expect("failed to get picture dir");
            let path_config = PathConfig {
                    download_path: default_path,
                    config_path,
                };
            
            app.manage(context);
            app.manage(path_config);
            Ok(())
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            get_top_wallpapers,
            save_config,
            load_config,
            download_wallpaper,
            get_tags
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
