// src/render.rs

/////////////////////////
// top of file
/////////////////////////

// import modules from other files
use crate::layout::LayoutBox;
use font8x8::{BASIC_FONTS, UnicodeFonts};

use crate::css::{get_font_size, get_text_color};
use crate::net::fetch_image;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const ADDRESS_BAR_HEIGHT: i32 = 72;
const TAB_WIDTH: i32 = 140;

pub struct LinkBox {
    pub text: String,
    pub url: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct TextBox {
    pub text: String,
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

fn get_tab_title(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        if let Ok(parsed) = reqwest::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return host.replace("www.", "").to_string();
            }
        }
    } else if let Some(filename) = std::path::Path::new(url).file_name() {
        if let Some(s) = filename.to_str() {
            return s.to_string();
        }
    }
    url.to_string()
}

pub fn draw_page(
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    tab_urls: &[String],
    favicons: &[Option<image::RgbaImage>],
    active_tab: usize,
    layout_boxes: &[LayoutBox],
    links: &mut Vec<LinkBox>,
    text_boxes: &mut Vec<TextBox>,
    current_url: &str,
    typing_url: &str,
    typing: bool,
    scroll_y: i32,
    selecting: bool,
    selection_start: (i32, i32),
    selection_end: (i32, i32),
) {
    clear(frame, [255, 255, 255, 255]);
    links.clear();

    draw_address_bar(frame, tab_urls, favicons, active_tab, current_url, typing_url, typing);
    draw_document(frame, cache, layout_boxes, links, text_boxes, scroll_y);

    // draw selection highlight rectangle if selecting is active
    if selecting && selection_start.1 >= ADDRESS_BAR_HEIGHT {
        let x1 = selection_start.0.min(selection_end.0);
        let y1 = selection_start.1.min(selection_end.1);
        let x2 = selection_start.0.max(selection_end.0);
        let y2 = selection_start.1.max(selection_end.1);

        let width = x2 - x1;
        let height = y2 - y1;

        blend_rect(frame, x1, y1, width, height, [100, 149, 237, 80]); // Cornflower blue with alpha 80
    }
}

fn draw_address_bar(
    frame: &mut [u8],
    tab_urls: &[String],
    favicons: &[Option<image::RgbaImage>],
    active_tab: usize,
    current_url: &str,
    typing_url: &str,
    typing: bool,
) {
    // adddress/Tab Bar Background
    fill_rect(
        frame,
        0,
        0,
        WIDTH as i32,
        ADDRESS_BAR_HEIGHT,
        [32, 35, 42, 255],
    );

    // draw Tabs
    let mut tab_x = 10;
    for (i, url) in tab_urls.iter().enumerate() {
        let is_active = i == active_tab;
        let bg_color = if is_active {
            [66, 66, 72, 255]
        } else {
            [45, 45, 50, 255]
        };

        fill_rect(frame, tab_x, 2, TAB_WIDTH, 24, bg_color);

        let title = get_tab_title(url);
        let truncated_title = if title.len() > 12 {
            format!("{}..", &title[..10])
        } else {
            title
        };

        // draw favicon if present, shift title right
        let title_x = if let Some(ref img) = favicons.get(i).and_then(|opt| opt.as_ref()) {
            draw_favicon(frame, img, tab_x + 4, 4);
            tab_x + 4 + 16 + 4 // icon width + gap
        } else {
            tab_x + 10
        };

        draw_text_line_raw(
            frame,
            &truncated_title,
            title_x,
            10,
            1,
            [255, 255, 255, 255],
        );

        draw_text_line_raw(
            frame,
            "x",
            tab_x + TAB_WIDTH - 18,
            10,
            1,
            [160, 160, 160, 255],
        );

        tab_x += TAB_WIDTH + 8;
    }

    // draw '+' new tab button
    fill_rect(frame, tab_x, 2, 24, 24, [45, 45, 50, 255]);
    draw_text_line_raw(frame, "+", tab_x + 8, 10, 1, [255, 255, 255, 255]);

    // 3. address bar input field white if active/typing, off-white if not
    let input_bg = if typing {
        [255, 255, 255, 255]
    } else {
        [240, 240, 245, 255]
    };
    fill_rect(frame, 8, 36, WIDTH as i32 - 16, 26, input_bg);

    // separator line after reload button 'R'
    fill_rect(frame, 78, 36, 1, 26, [200, 200, 205, 255]);

    let label = if typing {
        format!("/{typing_url}_")
    } else {
        current_url.to_string()
    };

    draw_text_line_raw(frame, "<", 18, 45, 1, [20, 22, 26, 255]);
    draw_text_line_raw(frame, ">", 38, 45, 1, [20, 22, 26, 255]);
    draw_text_line_raw(frame, "R", 58, 45, 1, [20, 22, 26, 255]);
    draw_text_line_raw(frame, &label, 90, 45, 1, [20, 22, 26, 255]);
}

fn draw_document(
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    layout_boxes: &[LayoutBox],
    links: &mut Vec<LinkBox>,
    text_boxes: &mut Vec<TextBox>,
    scroll_y: i32,
) {
    // browser pipeline:
    // dom tree = structure
    // layout tree = positions and sizes
    // renderer = pixels
    render_layout_tree(layout_boxes, frame, cache, links, text_boxes, scroll_y);
}

// Final document renderer:
// dom tree = structure
// layout tree = positions and sizes
// renderer = pixels only
pub fn render_layout_tree(
    layout_boxes: &[LayoutBox],
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    links: &mut Vec<LinkBox>,
    text_boxes: &mut Vec<TextBox>,
    scroll_y: i32,
) {
    for layout_box in layout_boxes {
        render_layout_box(layout_box, frame, cache, links, text_boxes, scroll_y);
    }
}

pub fn render_layout_box(
    layout_box: &LayoutBox,
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    links: &mut Vec<LinkBox>,
    text_boxes: &mut Vec<TextBox>,
    scroll_y: i32,
) {
    // skip elements entirely off-screen
    let y_pos = layout_box.y + scroll_y;
    if y_pos + layout_box.height < ADDRESS_BAR_HEIGHT || y_pos > HEIGHT as i32 {
        return;
    }

    match layout_box.tag.as_str() {
        "h1" | "h2" | "h3" => {
            let scale = match layout_box.tag.as_str() {
                "h1" => 3,
                "h2" | "h3" => 2,
                _ => 1,
            };

            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                scale,
                [0, 0, 0, 255],
                false,
                text_boxes,
            );
        }

        "p" => {
            let color = get_text_color(&layout_box.style);
            let font_size = get_font_size(&layout_box.style);
            let scale = match font_size {
                0..=16 => 1,
                17..=32 => 2,
                _ => 3,
            };

            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                scale,
                color,
                false,
                text_boxes,
            );
        }

        "message" => {
            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                1,
                [0, 0, 0, 255],
                false,
                text_boxes,
            );
        }

        "div" => {
            render_layout_children(layout_box, frame, cache, links, text_boxes, scroll_y);
        }

        "img" => {
            render_image_layout_box(layout_box, frame, cache, scroll_y);
        }

        "table" | "tbody" | "thead" => {
            render_layout_children(layout_box, frame, cache, links, text_boxes, scroll_y);
        }

        "tr" => {
            render_layout_children(layout_box, frame, cache, links, text_boxes, scroll_y);
        }

        "td" | "th" => {
            // draw border for table cells
            draw_rect_outline(
                frame,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                layout_box.height,
                [0, 0, 0, 255],
            );

            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                1,
                [0, 0, 0, 255],
                false,
                text_boxes,
            );
        }

        "ul" => {
            render_layout_children(layout_box, frame, cache, links, text_boxes, scroll_y);
        }

        "li" => {
            draw_wrapped_text_in_box(
                frame,
                &format!("* {}", layout_box.text),
                layout_box.x + 20,
                layout_box.y + scroll_y,
                layout_box.width - 20,
                1,
                [0, 0, 0, 255],
                false,
                text_boxes,
            );
        }

        "a" => {
            let href = layout_attribute(layout_box, "href");

            if !layout_box.text.is_empty() && !href.is_empty() {
                draw_wrapped_text_in_box(
                    frame,
                    &layout_box.text,
                    layout_box.x,
                    layout_box.y + scroll_y,
                    layout_box.width,
                    1,
                    [95, 170, 255, 255],
                    false,
                    text_boxes,
                );

                links.push(LinkBox {
                    text: layout_box.text.clone(),
                    url: href,
                    x: layout_box.x,
                    y: layout_box.y + scroll_y,
                    width: layout_box.width,
                    height: layout_box.height.max(10),
                });
            }
        }

        "b" | "strong" => {
            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                2,
                [0, 0, 0, 255],
                false,
                text_boxes,
            );
        }

        "i" | "em" => {
            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                1,
                [0, 0, 0, 255],
                true,
                text_boxes,
            );
        }

        "span" => {
            if layout_box.children.is_empty() {
                draw_wrapped_text_in_box(
                    frame,
                    &layout_box.text,
                    layout_box.x,
                    layout_box.y + scroll_y,
                    layout_box.width,
                    1,
                    [0, 0, 0, 255],
                    false,
                    text_boxes,
                );
            } else {
                render_layout_children(layout_box, frame, cache, links, text_boxes, scroll_y);
            }
        }

        "#text" => {
            draw_wrapped_text_in_box(
                frame,
                &layout_box.text,
                layout_box.x,
                layout_box.y + scroll_y,
                layout_box.width,
                1,
                [0, 0, 0, 255],
                false,
                text_boxes,
            );
        }

        "hr" => {
            draw_horizontal_line(frame, layout_box.y + scroll_y);
        }

        _ => {
            render_layout_children(layout_box, frame, cache, links, text_boxes, scroll_y);
        }
    }
}

fn render_layout_children(
    layout_box: &LayoutBox,
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    links: &mut Vec<LinkBox>,
    text_boxes: &mut Vec<TextBox>,
    scroll_y: i32,
) {
    for child in &layout_box.children {
        render_layout_box(child, frame, cache, links, text_boxes, scroll_y);
    }
}

fn render_image_layout_box(
    layout_box: &LayoutBox,
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    scroll_y: i32,
) {
    let src = layout_attribute(layout_box, "src");
    let alt = layout_attribute(layout_box, "alt");
    let y = layout_box.y + scroll_y;
    let is_network = src.starts_with("http://") || src.starts_with("https://");

    if is_network {
        if draw_network_image_in_box(
            frame,
            cache,
            &src,
            layout_box.x,
            y,
            layout_box.width,
            layout_box.height,
        )
        .is_some()
        {
            return;
        }
    } else if !src.is_empty()
        && draw_local_image_in_box(
            frame,
            cache,
            &src,
            layout_box.x,
            y,
            layout_box.width,
            layout_box.height,
        )
        .is_some()
    {
        return;
    }

    fill_rect(
        frame,
        layout_box.x,
        y,
        layout_box.width,
        layout_box.height,
        [45, 50, 60, 255],
    );
    draw_text_line_raw(
        frame,
        "[ IMAGE ]",
        layout_box.x + 10,
        y + 10,
        1,
        [255, 255, 255, 255],
    );
    draw_text_line_raw(
        frame,
        &format!("src: {}", src),
        layout_box.x + 10,
        y + 30,
        1,
        [180, 180, 180, 255],
    );
    if !alt.is_empty() {
        draw_text_line_raw(
            frame,
            &alt,
            layout_box.x + 10,
            y + 50,
            1,
            [220, 220, 220, 255],
        );
    }
}

fn layout_attribute(layout_box: &LayoutBox, name: &str) -> String {
    layout_box
        .attributes
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.clone())
        .unwrap_or_default()
}

fn draw_wrapped_text_in_box(
    frame: &mut [u8],
    text: &str,
    start_x: i32,
    start_y: i32,
    width: i32,
    scale: i32,
    color: [u8; 4],
    italic: bool,
    text_boxes: &mut Vec<TextBox>,
) -> i32 {
    let max_x = start_x + width;
    draw_wrapped_text_with_max_x(
        frame, text, start_x, start_y, max_x, scale, color, italic, text_boxes,
    )
}

fn draw_wrapped_text_with_max_x(
    frame: &mut [u8],
    text: &str,
    start_x: i32,
    start_y: i32,
    max_x: i32,
    scale: i32,
    color: [u8; 4],
    italic: bool,
    text_boxes: &mut Vec<TextBox>,
) -> i32 {
    let char_width = 8 * scale;
    let line_height = 12 * scale;
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

            text_boxes.push(TextBox {
                text: word.to_string(),
                x,
                y,
                width: word_width,
                height: line_height,
            });

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

fn blend_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < ADDRESS_BAR_HEIGHT || x >= WIDTH as i32 || y >= HEIGHT as i32 {
        return;
    }

    let idx = (y as usize * WIDTH as usize + x as usize) * 4;
    let alpha = color[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;

    frame[idx] = (color[0] as f32 * alpha + frame[idx] as f32 * inv_alpha) as u8;
    frame[idx + 1] = (color[1] as f32 * alpha + frame[idx + 1] as f32 * inv_alpha) as u8;
    frame[idx + 2] = (color[2] as f32 * alpha + frame[idx + 2] as f32 * inv_alpha) as u8;
}

fn blend_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: [u8; 4]) {
    for py in y..y + height {
        for px in x..x + width {
            blend_pixel(frame, px, py, color);
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

fn draw_local_image_in_box(
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    path: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Option<i32> {
    let cache_key = format!("local:{}:{}x{}", path, width, height);

    if let Some(rgba) = cache.get(&cache_key) {
        draw_cached_image(frame, rgba, x, y);
        return Some(rgba.height() as i32);
    }

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

    let target_width = width.max(1) as u32;
    let target_height = height.max(1) as u32;
    let scaled_img = img.thumbnail(target_width, target_height);
    let rgba = scaled_img.to_rgba8();

    cache.insert(cache_key.clone(), rgba);
    let rgba = cache.get(&cache_key)?;
    draw_cached_image(frame, rgba, x, y);

    Some(rgba.height() as i32)
}

fn draw_network_image_in_box(
    frame: &mut [u8],
    cache: &mut std::collections::HashMap<String, image::RgbaImage>,
    url: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Option<i32> {
    // try cache first

    if let Some(rgba) = cache.get(url) {
        //println!("Cache HIT: {}", url);
        draw_cached_image(frame, rgba, x, y);

        return Some(rgba.height() as i32);
    }

    // try fetch

    let bytes = fetch_image(url).ok()?;

    let img = image::load_from_memory(&bytes).ok()?;

    let target_width = width.max(1) as u32;
    let target_height = height.max(1) as u32;
    let scaled_img = img.thumbnail(target_width, target_height);

    let rgba = scaled_img.to_rgba8();

    println!("Cache MISS: {}", url);

    cache.insert(url.to_string(), rgba.clone());

    draw_cached_image(frame, &rgba, x, y);

    Some(rgba.height() as i32)
}

fn draw_cached_image(frame: &mut [u8], rgba: &image::RgbaImage, x: i32, y: i32) {
    for py in 0..rgba.height() {
        for px in 0..rgba.width() {
            let pixel = rgba.get_pixel(px, py);
            set_pixel(frame, x + px as i32, y + py as i32, pixel.0);
        }
    }
}

// horizontal line render function
fn draw_horizontal_line(frame: &mut [u8], y: i32) {
    let start_x = 20;

    let end_x = WIDTH as i32 - 20;

    for x in start_x..end_x {
        set_pixel(frame, x, y, [200, 200, 200, 255]);
    }
}

fn draw_rect_outline(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: [u8; 4]) {
    for px in x..x + width {
        set_pixel(frame, px, y, color);
        set_pixel(frame, px, y + height - 1, color);
    }

    for py in y..y + height {
        set_pixel(frame, x, py, color);
        set_pixel(frame, x + width - 1, py, color);
    }
}

fn draw_favicon(frame: &mut [u8], img: &image::RgbaImage, x: i32, y: i32) {
    for py in 0..img.height() {
        for px in 0..img.width() {
            let pixel = img.get_pixel(px, py);
            set_raw_pixel(frame, x + px as i32, y + py as i32, pixel.0);
        }
    }
}

/////////////////////////
// End of file
/////////////////////////
