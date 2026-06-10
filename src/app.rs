use crate::{
    dom::{Document, parse_html},
    net::fetch_page,
    render::{LinkBox, draw_page},
};
use pixels::Pixels;
use winit::{event::MouseScrollDelta, window::Window};

pub struct App {
    pub window: Window,
    pub pixels: Pixels,
    pub document: Document,
    pub links: Vec<LinkBox>,
    pub current_url: String,
    pub typing_url: String,
    pub typing: bool,
    pub scroll_y: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl App {
    pub fn new(window: Window, pixels: Pixels, initial_url: &str) -> Self {
        let mut app = Self {
            window,
            pixels,
            document: Document::default(),
            links: Vec::new(),
            current_url: initial_url.to_string(),
            typing_url: String::new(),
            typing: false,
            scroll_y: 0,
            mouse_x: 0,
            mouse_y: 0,
        };

        app.load_current_url();
        app
    }

    pub fn draw(&mut self) {
        let frame = self.pixels.frame_mut();

        draw_page(
            frame,
            &self.document,
            &mut self.links,
            &self.current_url,
            &self.typing_url,
            self.typing,
            self.scroll_y,
        );

        self.pixels.render().unwrap();
    }

    pub fn scroll(&mut self, delta: MouseScrollDelta) {
        if let MouseScrollDelta::LineDelta(_, y) = delta {
            self.scroll_y += (y as i32) * 20;
        }
    }

    pub fn start_typing(&mut self) {
        self.typing = true;
        self.typing_url.clear();
    }

    pub fn cancel_typing(&mut self) {
        self.typing = false;
        self.typing_url.clear();
    }

    pub fn submit_url(&mut self) {
        let next_url = self.typing_url.trim().to_string();

        if !next_url.is_empty() {
            self.current_url = normalize_url(&next_url);
            self.load_current_url();
        }

        self.cancel_typing();
    }

    pub fn click_link(&mut self) {
        if self.typing {
            return;
        }

        if let Some(url) = self
            .links
            .iter()
            .find(|link| link.contains(self.mouse_x, self.mouse_y))
            .map(|link| {
                println!("Opening link: {} -> {}", link.text, link.url);
                resolve_url(&self.current_url, &link.url)
            })
        {
            self.current_url = url;
            self.load_current_url();
        }
    }

    fn load_current_url(&mut self) {
        println!("Downloading {}...", self.current_url);

        match fetch_page(&self.current_url) {
            Ok(html) => {
                println!("Downloaded {} bytes. Parsing HTML...", html.len());
                self.document = parse_html(&html);
                self.scroll_y = 0;
            }
            Err(error) => {
                self.document = Document::from_message(format!(
                    "Could not load {}\n\n{}",
                    self.current_url, error
                ));
            }
        }
    }
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{url}")
    }
}

fn resolve_url(base_url: &str, url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }

    match reqwest::Url::parse(base_url).and_then(|base| base.join(url)) {
        Ok(url) => url.to_string(),
        Err(_) => normalize_url(url),
    }
}
