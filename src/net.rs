use reqwest::blocking;

pub fn fetch_page(url: &str) -> Result<String, String> {
    blocking::get(url)
        .map_err(|error| error.to_string())?
        .text()
        .map_err(|error| error.to_string())
}

pub fn fetch_image(url: &str) -> Result<Vec<u8>, String> {
    reqwest::blocking::get(url)
        .map_err(|error| error.to_string())?
        .bytes()
        .map(|bytes| bytes.to_vec())
        .map_err(|error| error.to_string())
}

