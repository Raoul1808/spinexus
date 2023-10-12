use std::fs::{File, self};
use std::path::Path;
use std::io;
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

async fn decompress_zip(zip: &str, destination: String) -> io::Result<()> {
    let file = File::open(zip)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let dest = Path::new(destination.as_str());

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = match file.enclosed_name() {
            Some(path) => dest.join(path.to_owned()),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut out_file = File::create(&out_path)?;
            io::copy(&mut file, &mut out_file)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
            }
        }
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

pub async fn download_and_extract_zip(url: String, cache: String, destination: String, filename: String) -> Result<(), String> {
    let cached_zip = Path::new(cache.as_str()).join(filename.as_str()).to_str().unwrap().to_string();
    download_file(url, cached_zip.clone()).await?;
    decompress_zip(cached_zip.as_str(), destination).await.or(Err(format!("Could not extract zip")))?;
    fs::remove_file(&cached_zip).or(Err(format!("Failed file cleanup")))?;
    Ok(())
}
