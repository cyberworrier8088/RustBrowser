use crate::{
    dom::{Document, parse_html},
    net::fetch_page,
    render::{LinkBox, draw_page},
};
use pixels::Pixels;
use std::fs;
use winit::{event::MouseScrollDelta, window::Window};

pub struct Tab {
    pub document: Document,
    pub current_url: String,
    pub history: Vec<String>,
    pub history_index: usize,
    pub scroll_y: i32,
}

pub struct App {
    pub window: Window,
    pub pixels: Pixels,
    pub links: Vec<LinkBox>,
    pub typing_url: String,
    pub typing: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

impl App {
    pub fn new_tab(&mut self, url: &str) {
        self.tabs.push(Tab {
            document: Document::default(),
            current_url: url.to_string(),
            history: vec![url.to_string()],
            history_index: 0,
            scroll_y: 0,
        });

        self.active_tab = self.tabs.len() - 1;

        self.load_current_url();
    }
    fn current_tab(&self) -> &Tab {
        &self.tabs[self.active_tab]
    }

    fn current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_tab]
    }

    pub fn new(window: Window, pixels: Pixels, initial_url: &str) -> Self {
        let mut app = Self {
            window,
            pixels,
            links: Vec::new(),
            typing_url: String::new(),
            typing: false,
            mouse_x: 0,
            mouse_y: 0,
            tabs: vec![Tab {
                document: Document::default(),
                current_url: initial_url.to_string(),
                history: vec![initial_url.to_string()],
                history_index: 0,
                scroll_y: 0,
            }],
            active_tab: 0,
        };

        app.load_current_url();
        app
    }

    pub fn draw(&mut self) {
        let active = self.active_tab;
        let document = &self.tabs[active].document;
        let current_url = &self.tabs[active].current_url;
        let scroll_y = self.tabs[active].scroll_y;
        let frame = self.pixels.frame_mut();
        draw_page(
            frame,
            document,
            &mut self.links,
            current_url,
            &self.typing_url,
            self.typing,
            scroll_y,
        );
        self.pixels.render().unwrap();
    }

    pub fn scroll(&mut self, delta: MouseScrollDelta) {
        let amount = match delta {
            MouseScrollDelta::LineDelta(_, y) => (y as i32) * 20,
            MouseScrollDelta::PixelDelta(position) => position.y as i32,
        };

        self.current_tab_mut().scroll_y += amount;
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
            let url = normalize_url(&next_url);
            self.visit(url);
        }

        self.cancel_typing();
    }

    pub fn visit(&mut self, url: String) {
        let tab = self.current_tab_mut();

        if tab.history_index + 1 < tab.history.len() {
            tab.history.truncate(tab.history_index + 1);
        }

        tab.history.push(url.clone());
        tab.history_index += 1;
        tab.current_url = url;

        self.load_current_url();
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
                resolve_url(&self.current_tab().current_url, &link.url)
            })
        {
            self.visit(url);
        }
    }

    pub fn go_back(&mut self) {
        let tab = self.current_tab_mut();

        if tab.history_index > 0 {
            tab.history_index -= 1;

            tab.current_url = tab.history[tab.history_index].clone();

            self.load_current_url();
        }
    }

    pub fn go_forward(&mut self) {
        let tab = self.current_tab_mut();

        if tab.history_index + 1 < tab.history.len() {
            tab.history_index += 1;

            tab.current_url = tab.history[tab.history_index].clone();

            self.load_current_url();
        }
    }

    fn load_current_url(&mut self) {
        let current_url = self.current_tab().current_url.clone();

        println!("Downloading {}...", current_url);

        let result = if current_url.ends_with(".html") {
            fs::read_to_string(&current_url).map_err(|error| error.to_string())
        } else {
            fetch_page(&current_url)
        };

        match result {
            Ok(html) => {
                println!("Downloaded {} bytes. Parsing HTML...", html.len());
                let tab = self.current_tab_mut();
                tab.document = parse_html(&html);
                tab.scroll_y = 0;
            }
            Err(error) => {
                self.current_tab_mut().document =
                    Document::from_message(format!("Could not load {}\n\n{}", current_url, error));
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
