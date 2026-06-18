// src/css.rs



// style struct that used for storing css properties
pub struct Style {
    pub color: Option<[u8; 4]>,       // css
    pub background: Option<[u8; 4]>,  // css
}

pub fn get_text_color(style: &Option<String>) -> [u8; 4] {
    if let Some(style_text) = style {

        // basic colors added
        if style_text.contains("color:red") {
            return [255, 0, 0, 255];
        }
        if style_text.contains("color:black") {
            return [0, 0, 0, 255];
        }
        if style_text.contains("color:blue") {
            return [0, 0, 255, 255];
        }
        if style_text.contains("color:green") {
            return [0, 255, 0, 255];
        }
        if style_text.contains("color:yellow") {
            return [255, 255, 0, 255];
        }
        if style_text.contains("color:purple") {
            return [255, 0, 255, 255];
        }
        if style_text.contains("color:orange") {
            return [255, 165, 0, 255];
        }
        if style_text.contains("color:pink") {
            return [255, 192, 203, 255];
        }
        if style_text.contains("color:brown") {
            return [165, 42, 42, 255];
        }
        if style_text.contains("color:gray") {
            return [128, 128, 128, 255];
        }
        if style_text.contains("color:cyan") {
            return [0, 255, 255, 255];
        }
        if style_text.contains("color:magenta") {
            return [255, 0, 255, 255];
        }
        if style_text.contains("color:white") {
            return [255, 255, 255, 255];
        }
        if style_text.contains("color:silver") {
            return [192, 192, 192, 255];
        }
    }

    [0, 0, 0, 255]
}