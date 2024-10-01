use std::env;
use std::error::Error;
use tokio::process::Command;

#[derive(Debug)]
pub enum DownloaderError {
    ProgramNotFound,
    DownloadFailed(String),
    HomeDirNotFound,
}

impl std::fmt::Display for DownloaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloaderError::ProgramNotFound => write!(f, "yt-dlp program not found"),
            DownloaderError::DownloadFailed(msg) => write!(f, "Download failed: {}", msg),
            DownloaderError::HomeDirNotFound => write!(f, "Home directory not found"),
        }
    }
}

impl Error for DownloaderError {}

pub async fn downloader(url: &str, username: &str) -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME").map_err(|_| DownloaderError::HomeDirNotFound)?;
    let download_path = format!("{}/Downloads", home_dir);

    let output = Command::new("yt-dlp")
        .args(&[
            "-x",
            "--audio-format",
            "mp3",
            "-o",
            &format!("{}/%(title)s.%(ext)s", download_path),
            url,
        ])
        .output()
        .await;

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Download successful for user {}!", username);
                Ok(())
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to download: {}", error_message);
                Err(Box::new(DownloaderError::DownloadFailed(
                    error_message.to_string(),
                )))
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Err(Box::new(DownloaderError::ProgramNotFound))
            } else {
                Err(Box::new(e))
            }
        }
    }
}
