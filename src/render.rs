use crate::dom::{Document, Element};
use font8x8::{BASIC_FONTS, UnicodeFonts};

use crate::net::fetch_image;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

const ADDRESS_BAR_HEIGHT: i32 = 44;
const CONTENT_LEFT: i32 = 10;
const CONTENT_TOP: i32 = 56;

pub struct LinkBox {
    pub text: String,
    pub url: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl LinkBox {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

pub fn draw_page(
    frame: &mut [u8],
    document: &Document,
    links: &mut Vec<LinkBox>,
    current_url: &str,
    typing_url: &str,
    typing: bool,
    scroll_y: i32,
) {
    clear(frame, [12, 13, 16, 255]);
    links.clear();

    draw_address_bar(frame, current_url, typing_url, typing);
    draw_document(frame, document, links, scroll_y);
}

fn draw_address_bar(frame: &mut [u8], current_url: &str, typing_url: &str, typing: bool) {
    fill_rect(
        frame,
        0,
        0,
        WIDTH as i32,
        ADDRESS_BAR_HEIGHT,
        [32, 35, 42, 255],
    );
    fill_rect(frame, 8, 7, WIDTH as i32 - 16, 20, [245, 245, 245, 255]);

    let label = if typing {
        format!("/{typing_url}_")
    } else {
        current_url.to_string()
    };

    draw_text_line_raw(frame, "<", 12, 13, 1, [20, 22, 26, 255]);
    draw_text_line_raw(frame, ">", 32, 13, 1, [20, 22, 26, 255]);
    draw_text_line_raw(frame, "R", 52, 13, 1, [20, 22, 26, 255]);
    draw_text_line_raw(frame, &label, 90, 13, 1, [20, 22, 26, 255]);
}

fn draw_document(frame: &mut [u8], document: &Document, links: &mut Vec<LinkBox>, scroll_y: i32) {
    let mut y = CONTENT_TOP + scroll_y;

    for element in &document.elements {
        match element {
            Element::Heading { level, text } => {
                let scale = match level {
                    1 => 3,
                    2 => 2,
                    3 => 2,
                    4 => 1,
                    5 => 1,
                    _ => 1,
                };
                y = draw_wrapped_text(
                    frame,
                    text,
                    CONTENT_LEFT,
                    y,
                    scale,
                    [255, 255, 255, 255],
                    false,
                );
                y += 12;
            }
            Element::Paragraph(text) => {
                y = draw_wrapped_text(frame, text, CONTENT_LEFT, y, 1, [225, 225, 225, 255], false);
                y += 12;
            }
            Element::Link { text, url } => {
                let start_y = y;
                y = draw_wrapped_text(frame, text, CONTENT_LEFT, y, 1, [95, 170, 255, 255], false);
                let height = (y - start_y).max(10);

                links.push(LinkBox {
                    text: text.clone(),
                    url: url.clone(),
                    x: CONTENT_LEFT,
                    y: start_y,
                    width: (text.len() as i32 * 8).min(WIDTH as i32 - CONTENT_LEFT * 2),
                    height,
                });

                y += 12;
            }
            Element::ListIteam(text) => {
                y = draw_wrapped_text(
                    frame,
                    &format!("* {}", text),
                    CONTENT_LEFT + 20,
                    y,
                    1,
                    [97, 225, 225, 255],
                    false,
                );
                y += 8;
            }
            Element::Bold(text) => {
                y = draw_wrapped_text(frame, text, CONTENT_LEFT, y, 2, [255, 255, 255, 255], false);
                y += 12;
            }
            Element::Italic(text) => {
                // Option 1: Use a slightly smaller scale and a distinct color
                y = draw_wrapped_text(frame, text, CONTENT_LEFT, y, 1, [200, 200, 200, 255], true);
                y += 12;
            }
            Element::Image { src, alt } => {
                let is_network =
                    src.starts_with("http://")
                    || src.starts_with("https://");

                if is_network {
                    if let Some(height) =
                        draw_network_image(frame, src, CONTENT_LEFT, y)
                    {
                        y += height + 12;
                        continue;
                    }
                } else {
                    if let Some(height) =
                        draw_local_image(frame, src, CONTENT_LEFT, y)
                    {
                        y += height + 12;
                        continue;
                    }
                }

                let box_width = 220;
                let box_height = 80;
                fill_rect(
                    frame,
                    CONTENT_LEFT,
                    y,
                    box_width,
                    box_height,
                    [45, 50, 60, 255],
                );
                draw_text_line_raw(
                    frame,
                    "[ IMAGE ]",
                    CONTENT_LEFT + 10,
                    y + 10,
                    1,
                    [255, 255, 255, 255],
                );
                draw_text_line_raw(
                    frame,
                    &format!("src: {}", src),
                    CONTENT_LEFT + 10,
                    y + 30,
                    1,
                    [180, 180, 180, 255],
                );
                if !alt.is_empty() {
                    draw_text_line_raw(
                        frame,
                        alt,
                        CONTENT_LEFT + 10,
                        y + 50,
                        1,
                        [220, 220, 220, 255],
                    );
                }
                y += box_height + 12;
            }
        }
    }
}

fn draw_wrapped_text(
    frame: &mut [u8],
    text: &str,
    start_x: i32,
    start_y: i32,
    scale: i32,
    color: [u8; 4],
    italic: bool, // new
) -> i32 {
    let char_width = 8 * scale;
    let line_height = 12 * scale;
    let max_x = WIDTH as i32 - 10;
    let mut x = start_x;
    let mut y = start_y;
    let shear = if italic { 0.35 } else { 0.0 };

    for line in text.split('\n') {
        for word in line.split_whitespace() {
            let word_width = word.chars().count() as i32 * char_width;

            if x > start_x && x + word_width > max_x {
                x = start_x;
                y += line_height;
            }

            for ch in word.chars() {
                if italic {
                    draw_char_italic(frame, ch, x, y, scale, color, shear);
                } else {
                    draw_char(frame, ch, x, y, scale, color);
                }
                x += char_width;
            }

            x += char_width;
        }

        x = start_x;
        y += line_height;
    }

    y
}

fn draw_text_line_raw(frame: &mut [u8], text: &str, x: i32, y: i32, scale: i32, color: [u8; 4]) {
    let mut cursor_x = x;

    for ch in text.chars() {
        if cursor_x > WIDTH as i32 - 12 {
            break;
        }

        draw_char_raw(frame, ch, cursor_x, y, scale, color);
        cursor_x += 8 * scale;
    }
}

fn draw_char(frame: &mut [u8], ch: char, x: i32, y: i32, scale: i32, color: [u8; 4]) {
    if let Some(bitmap) = BASIC_FONTS.get(ch) {
        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if (bits >> col) & 1 != 0 {
                    draw_scaled_pixel(frame, x + col * scale, y + row as i32 * scale, scale, color);
                }
            }
        }
    }
}

fn draw_char_raw(frame: &mut [u8], ch: char, x: i32, y: i32, scale: i32, color: [u8; 4]) {
    if let Some(bitmap) = BASIC_FONTS.get(ch) {
        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if (bits >> col) & 1 != 0 {
                    draw_scaled_pixel_raw(
                        frame,
                        x + col * scale,
                        y + row as i32 * scale,
                        scale,
                        color,
                    );
                }
            }
        }
    }
}

fn draw_scaled_pixel(frame: &mut [u8], x: i32, y: i32, scale: i32, color: [u8; 4]) {
    for offset_y in 0..scale {
        for offset_x in 0..scale {
            set_pixel(frame, x + offset_x, y + offset_y, color);
        }
    }
}

fn draw_scaled_pixel_raw(frame: &mut [u8], x: i32, y: i32, scale: i32, color: [u8; 4]) {
    for offset_y in 0..scale {
        for offset_x in 0..scale {
            set_raw_pixel(frame, x + offset_x, y + offset_y, color);
        }
    }
}

fn set_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < ADDRESS_BAR_HEIGHT || x >= WIDTH as i32 || y >= HEIGHT as i32 {
        return;
    }

    let idx = (y as usize * WIDTH as usize + x as usize) * 4;

    frame[idx] = color[0];
    frame[idx + 1] = color[1];
    frame[idx + 2] = color[2];
    frame[idx + 3] = color[3];
}

fn fill_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: [u8; 4]) {
    for py in y..y + height {
        for px in x..x + width {
            set_raw_pixel(frame, px, py, color);
        }
    }
}

fn set_raw_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
        return;
    }

    let idx = (y as usize * WIDTH as usize + x as usize) * 4;

    frame[idx] = color[0];
    frame[idx + 1] = color[1];
    frame[idx + 2] = color[2];
    frame[idx + 3] = color[3];
}

fn clear(frame: &mut [u8], color: [u8; 4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&color);
    }
}

fn draw_char_italic(
    frame: &mut [u8],
    ch: char,
    x: i32,
    y: i32,
    scale: i32,
    color: [u8; 4],
    shear: f32,
) {
    if let Some(bitmap) = BASIC_FONTS.get(ch) {
        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if (bits >> col) & 1 != 0 {          
                    let shear_offset = ((7 - row) as f32 * scale as f32 * shear) as i32;
                    let new_x = x + col * scale + shear_offset;
                    let new_y = y + row as i32 * scale;
                    draw_scaled_pixel(frame, new_x, new_y, scale, color);
                }
            }
        }
    }
}




fn draw_local_image(
    frame: &mut [u8],
    path: &str,
    x: i32,
    y: i32,
) -> Option<i32> {
    let img = match image::ImageReader::open(path) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(e) => {
                    println!("Error decoding image {}: {:?}", path, e);
                    return None;
                }
            },
            Err(e) => {
                println!("Error guessing format for {}: {:?}", path, e);
                return None;
            }
        },
        Err(e) => {
            println!("Error opening file {}: {:?}", path, e);
            return None;
        }
    };

    let scaled_img = img.thumbnail(300, 200);
    let rgba = scaled_img.to_rgba8();

    let width = rgba.width();
    let height = rgba.height();

    for py in 0..height {
        for px in 0..width {
            let pixel = rgba.get_pixel(px, py);

            set_pixel(
                frame,
                x + px as i32,
                y + py as i32,
                pixel.0,
            );
        }
    }

    Some(height as i32)
}

fn draw_network_image(
    frame: &mut [u8],
    url: &str,
    x: i32,
    y: i32,
) -> Option<i32> {
    let bytes = fetch_image(url).ok()?;

    let img = image::load_from_memory(&bytes).ok()?;

    let scaled_img = img.thumbnail(300, 200);

    let rgba = scaled_img.to_rgba8();

    let width = rgba.width();
    let height = rgba.height();

    for py in 0..height {
        for px in 0..width {
            let pixel = rgba.get_pixel(px, py);

            set_pixel(
                frame,
                x + px as i32,
                y + py as i32,
                pixel.0,
            );
        }
    }

    Some(height as i32)
}