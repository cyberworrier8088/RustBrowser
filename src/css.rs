// src/css.rs

// style struct that used for storing css properties
pub struct Style {
    pub color: Option<[u8; 4]>,      // css
    pub background: Option<[u8; 4]>, // css
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

pub fn get_font_size(style: &Option<String>) -> u32 {
    if let Some(style_text) = style {
        let style_text = style_text.replace(" ", "");

        // basic font sizes added
        if style_text.contains("font-size:10px") {
            return 10;
        }
        if style_text.contains("font-size:12px") {
            return 12;
        }
        if style_text.contains("font-size:14px") {
            return 14;
        }
        if style_text.contains("font-size:16px") {
            return 16;
        }
        if style_text.contains("font-size:18px") {
            return 18;
        }
        if style_text.contains("font-size:20px") {
            return 20;
        }
        if style_text.contains("font-size:22px") {
            return 22;
        }
        if style_text.contains("font-size:24px") {
            return 24;
        }
        if style_text.contains("font-size:26px") {
            return 26;
        }
        if style_text.contains("font-size:28px") {
            return 28;
        }
        if style_text.contains("font-size:30px") {
            return 30;
        }
        if style_text.contains("font-size:32px") {
            return 32;
        }
        if style_text.contains("font-size:34px") {
            return 34;
        }
        if style_text.contains("font-size:36px") {
            return 36;
        }
    }

    16
}



