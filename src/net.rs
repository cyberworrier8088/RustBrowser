use reqwest::blocking;

pub fn fetch_page(url: &str) -> Result<String, String> {
    blocking::get(url)
        .map_err(|error| error.to_string())?
        .text()
        .map_err(|error| error.to_string())
}
