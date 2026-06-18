use std::fs;

pub fn download_file(url: &str) -> Result<(), String> {
    println!("[DOWNLOAD]: Downloading {}", url);

    fs::create_dir_all("downloads").map_err(|e| e.to_string())?;

    let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;

    let bytes = response.bytes().map_err(|e| e.to_string())?;

    let filename = url.split("/").last().unwrap_or("download.bin");

    let path = format!("downloads/{}", filename);

    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;

    println!("Download folder ready");
    println!("Downloaded {} bytes", bytes.len());
    println!("Saved to {}", path);

    Ok(())
}
