use std::fs::{File, self};
use std::path::Path;
use std::io::Write;
use futures_util::StreamExt;

async fn download_file_internal(url: &str, path: &str) -> Result<(), String> {
    let res = reqwest::get(url)
        .await
        .or(Err(format!("Failed to get content at {url}")))?;

    let mut file = File::create(&path)
        .or(Err(format!("Failed to create file at {path}")))?;

    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing file")))?;
    }

    Ok(())
}

pub async fn download_file(url: String, path: String) -> Result<(), String> {
    match download_file_internal(url.as_str(), path.as_str()).await {
        Ok(_) => Ok(()),
        Err(e) => {
            if Path::new(&path).exists() {
                fs::remove_file(&path)
                    .or(Err(format!("Failed file cleanup after error: {e}")))?;
            }
            Err(e)
        }
    }
}
