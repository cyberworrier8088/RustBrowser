// src/net.rs


/////////////////////////
// top of file
/////////////////////////


// using libraries
use reqwest::blocking;

// func for fetch html page from url
pub fn fetch_page(url: &str) -> Result<String, String> {
    blocking::get(url)
        .map_err(|error| error.to_string())?
        .text()
        .map_err(|error| error.to_string())
}


// func for fetch image from url for display in the page
pub fn fetch_image(url: &str) -> Result<Vec<u8>, String> {
    reqwest::blocking::get(url)
        .map_err(|error| error.to_string())?
        .bytes()
        .map(|bytes| bytes.to_vec())
        .map_err(|error| error.to_string())
}

/////////////////////////
// End of file
/////////////////////////