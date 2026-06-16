// src/app.rs :)


//////////////////////////
// start of file
/////////////////////////

// import modules from other files
use crate::{
    dom::{Document, Node, parse_html},
    net::fetch_page,
    render::{LinkBox, draw_page},
};

// useing libraries
use pixels::Pixels;
use std::fs;
use winit::{event::MouseScrollDelta, window::Window};

// tab struct
pub struct Tab {
    pub document: Document,
    pub current_url: String,
    pub history: Vec<String>,
    pub history_index: usize,
    pub scroll_y: i32,
    pub favicon: Option<String>,
}

// main browser struct
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

    // for cache images
    pub image_cache: std::collections::HashMap<String, image::RgbaImage>,

    // book marks
    pub bookmarks: Vec<String>,
}

// this is implementation of app struct
impl App {




    // create new tab
    pub fn new_tab(&mut self, url: &str) {
        self.tabs.push(Tab {
            document: Document::default(),
            current_url: url.to_string(),
            history: vec![url.to_string()],
            history_index: 0,
            scroll_y: 0,
            favicon: None,
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
            
            // bookmark
            bookmarks: Vec::new(),
            
            // image cache
            image_cache: std::collections::HashMap::new(),
            
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
                favicon: None,
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
        
        let tab_urls: Vec<String> = self.tabs.iter().map(|t| t.current_url.clone()).collect();
        
        draw_page(
            frame,
            &mut self.image_cache,
            &tab_urls,
            self.active_tab,
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

        let tab = self.current_tab_mut();
        tab.scroll_y += amount;
        if tab.scroll_y > 0 {
            tab.scroll_y = 0;
        }
    }

    pub fn start_typing(&mut self) {
        self.typing = true;
        self.typing_url = self.tabs[self.active_tab].current_url.clone();
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

    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            self.tabs[0] = Tab {
                document: Document::default(),
                current_url: "src/main.html".to_string(),
                history: vec!["src/main.html".to_string()],
                history_index: 0,
                scroll_y: 0,
                favicon: None,
            };
            self.active_tab = 0;
            self.load_current_url();
            return;
        }

        self.tabs.remove(index);

        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        } else if self.active_tab == index && index > 0 {
            self.active_tab = index - 1;
        }

        self.load_current_url();
    }

    pub fn click_link(&mut self) {
        // 1. Check tab bar clicks (y in 2..26)
        if self.mouse_y >= 2 && self.mouse_y <= 26 {
            let mut tab_x = 10;
            let num_tabs = self.tabs.len();
            for i in 0..num_tabs {
                if self.mouse_x >= tab_x && self.mouse_x < tab_x + 140 {
                    if self.mouse_x >= tab_x + 140 - 20 {
                        println!("GUI: Close tab {} clicked", i);
                        self.close_tab(i);
                    } else {
                        println!("GUI: Switch to tab {} clicked", i);
                        self.active_tab = i;
                        self.cancel_typing();
                    }
                    return;
                }
                tab_x += 140 + 8;
            }

            if self.mouse_x >= tab_x && self.mouse_x < tab_x + 24 {
                println!("GUI: New tab clicked");
                self.new_tab("src/main.html");
                return;
            }
        }

        // 2. check address bar input & buttons clicks (y in 36..62) :)
        if self.mouse_y >= 36 && self.mouse_y <= 62 {
            if self.mouse_x >= 8 && self.mouse_x < 28 {
                println!("GUI: Back clicked");
                self.go_back();
                return;
            } else if self.mouse_x >= 28 && self.mouse_x < 48 {
                println!("GUI: Forward clicked");
                self.go_forward();
                return;
            } else if self.mouse_x >= 48 && self.mouse_x < 68 {
                println!("GUI: Reload clicked");
                self.load_current_url();
                return;
            } else if self.mouse_x >= 80 && self.mouse_x < (800 - 8) {
                println!("GUI: Address bar clicked");
                self.start_typing();
                return;
            }
        }

        if self.typing {
            self.cancel_typing();
            return;
        }

        //  check document link clicks
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
                let mut document = parse_html(&html);
                
                // resolve image URLs in the Node tree directly
                if let Some(ref mut root) = document.root {
                    resolve_image_urls_in_tree(root, &current_url);
                }
                // store the final result
                let tab = self.current_tab_mut();
                tab.document = document;
                tab.scroll_y = 0;
            }
            Err(error) => {
                self.current_tab_mut().document =
                    Document::from_message(format!("Could not load {}\n\n{}", current_url, error));
            }
        }
    }

    pub fn add_bookmark(&mut self) {
        let url = self.current_tab().current_url.clone();

        if !self.bookmarks.contains(&url) {
            self.bookmarks.push(url.clone());

            println!("Bookmarked: {}", url);
        }else {
            println!("Already bookmarked")
        }
    }
}

fn normalize_url(input: &str) -> String {
    let input = input.trim();

    if input.starts_with("http://") || input.starts_with("https://") {
        return input.to_string();
    }

    // looks like a domain
    if input.contains('.') && !input.contains(' ') {
        return format!("https://{}", input);
    }

    // otherwise, use google search
    let query = input.replace(' ', "+");

    format!("https://www.google.com/search?q={}", query)

}

// func for user URL in the page correcting to go
pub fn resolve_url(base_url: &str, url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }

    match reqwest::Url::parse(base_url).and_then(|base| base.join(url)) {
        Ok(url) => url.to_string(),
        Err(_) => normalize_url(url),
    }
}

// walk the Node tree and resolve relative image src URLs
fn resolve_image_urls_in_tree(node: &mut Node, base_url: &str) {
    if node.tag == "img" {
        if let Some((_, src)) = node.attributes.iter_mut().find(|(k, _)| k == "src") {
            let resolved = resolve_url(base_url, src);
            println!(">> RESOLVED IMAGE SRC: {}", resolved);
            *src = resolved;
        }
    }
    for child in &mut node.children {
        resolve_image_urls_in_tree(child, base_url);
    }
}



////////////////////////
// End of file
///////////////////////