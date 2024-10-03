#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;

mod download;

use crate::download::{downloader, DownloaderError};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Aboba!", name)
}

#[tauri::command]
async fn download(url: String) -> Result<(), String> {
    match downloader(&url).await {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(downloader_error) = e.downcast_ref::<DownloaderError>() {
                match downloader_error {
                    DownloaderError::ProgramNotFound => Err("yt-dlp is not installed or not found in PATH. Please install yt-dlp and try again.".to_string()),
                    DownloaderError::DownloadFailed(msg) => Err(format!("Download failed: {}", msg)),
                    DownloaderError::HomeDirNotFound => Err("Unable to determine the home directory.".to_string()),
                    DownloaderError::InvalidTrackloadedPath => Err("Invalid trackloaded path.".to_string()),
                }
            } else {
                Err(format!("An unexpected error occurred: {}", e))
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
