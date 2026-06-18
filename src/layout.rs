// src/layout.rs

// :)
// A browser uses a few separate trees:
//
// DOM Tree:
// The parsed HTML document. It knows tags, text, attributes, and children.
//
// Layout Tree:
// The measured version of the DOM. It knows where every box should live on
// the page, but it does not draw pixels.
//
// Rendering Pipeline:
// HTML -> DOM Tree -> Layout Tree -> Renderer -> Pixels/Winit Window
//
// browsers separate layout from rendering because layout answers "where and
// how big?" while rendering answers "what pixels should be painted there?".
// Keeping those jobs apart makes CSS backgrounds, margins, padding, borders,
// images, tables, buttons, and forms much easier to grow later.

// :)
use crate::dom::{Document, Node};

const CONTENT_LEFT: i32 = 10;
const CONTENT_TOP: i32 = 84;
const CONTENT_WIDTH: i32 = 780;
const DEFAULT_TEXT_HEIGHT: i32 = 16;
const DEFAULT_IMAGE_HEIGHT: i32 = 200;
const TABLE_CELL_WIDTH: i32 = 160;
const TABLE_ROW_HEIGHT: i32 = 24;

#[derive(Debug, Clone)]
pub struct LayoutBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub tag: String,
    pub text: String,
    pub style: Option<String>,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<LayoutBox>,
}

pub fn layout_document(document: &Document) -> Vec<LayoutBox> {
    let mut y = CONTENT_TOP;
    let mut boxes = Vec::new();

    if let Some(root) = &document.root {
        if let Some(layout_box) = layout_node(root, &mut y) {
            boxes.push(layout_box);
        }
    }

    boxes
}

// builds one layout box for one DOM node.
// this function calculates positions only.
// it never paints pixels :)
pub fn layout_node(node: &Node, y: &mut i32) -> Option<LayoutBox> {
    if node.tag == "#text" && node.text.trim().is_empty() {
        return None;
    }

    // disable some tags
    if node.tag == "script"
        || node.tag == "style"
        || node.tag == "!doctype"
        || node.tag == "noscript"
    {
        return None;
    }
    let start_y = *y;
    let mut layout_box = LayoutBox {
        x: CONTENT_LEFT,
        y: start_y,
        width: CONTENT_WIDTH,
        height: default_height(node),
        tag: node.tag.clone(),
        text: collect_layout_text(node),
        style: node.style.clone(),
        attributes: node.attributes.clone(),
        children: Vec::new(),
    };

    match node.tag.as_str() {
        "h1" | "h2" | "h3" | "p" | "message" | "a" | "li" | "ul" | "b" | "strong" | "i" | "em"
        | "span" | "#text" => {
            *y += layout_box.height;
            add_default_spacing(node, y);
        }

        "img" => {
            layout_box.height = image_height(node);
            *y += layout_box.height + 12;
        }

        "div" => {
            *y += 10;
            layout_box.y = start_y;
            layout_box.children = layout_children(node, y);
            *y += 10;
            layout_box.height = *y - start_y;
        }

        "table" => {
            layout_box.children = layout_children(node, y);
            layout_box.height = *y - start_y;
        }

        "tr" => {
            layout_box.height = TABLE_ROW_HEIGHT;
            layout_box.children = layout_table_cells(node, start_y);
            *y += TABLE_ROW_HEIGHT;
        }

        "td" | "th" => {
            *y += TABLE_ROW_HEIGHT;
        }

        "hr" => {
            layout_box.height = 20;
            *y += layout_box.height;
        }
        "html" | "body" => {
            layout_box.children = layout_children(node, y);
            layout_box.height = *y - start_y;
        }

        _ => {
            layout_box.children = layout_children(node, y);
            layout_box.height = *y - start_y;
        }
    }

    Some(layout_box)
}

fn layout_children(node: &Node, y: &mut i32) -> Vec<LayoutBox> {
    let mut children = Vec::new();

    for child in &node.children {
        if let Some(child_box) = layout_node(child, y) {
            children.push(child_box);
        }
    }

    children
}

fn layout_table_cells(node: &Node, row_y: i32) -> Vec<LayoutBox> {
    let mut cell_x = CONTENT_LEFT;
    let mut cells = Vec::new();

    for child in &node.children {
        if child.tag == "td" || child.tag == "th" {
            cells.push(LayoutBox {
                x: cell_x,
                y: row_y,
                width: TABLE_CELL_WIDTH,
                height: TABLE_ROW_HEIGHT,
                tag: child.tag.clone(),
                text: collect_layout_text(child),
                style: child.style.clone(),
                attributes: child.attributes.clone(),
                children: Vec::new(),
            });
            cell_x += TABLE_CELL_WIDTH;
        }
    }

    cells
}

fn default_height(node: &Node) -> i32 {
    match node.tag.as_str() {
        "h1" => 40,
        "h2" => 32,
        "h3" => 28,
        "h4" => 24,
        "h5" => 20,
        "h6" => 19,
        "p" => 24,
        "message" => 24,
        "a" | "li" | "ul" | "b" | "strong" | "i" | "em" | "span" | "#text" => DEFAULT_TEXT_HEIGHT,
        "td" | "th" | "tr" => TABLE_ROW_HEIGHT,
        _ => 0,
    }
}

fn add_default_spacing(node: &Node, y: &mut i32) {
    match node.tag.as_str() {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "message" | "a" | "b" | "strong" | "i"
        | "em" | "span" => {
            *y += 12;
        }
        "li" | "ul" => {
            *y += 8;
        }
        "#text" => {
            *y += 4;
        }
        _ => {}
    }
}

fn image_height(node: &Node) -> i32 {
    node.attributes
        .iter()
        .find(|(name, _)| name == "height")
        .and_then(|(_, value)| value.parse::<i32>().ok())
        .unwrap_or(DEFAULT_IMAGE_HEIGHT)
}

fn collect_layout_text(node: &Node) -> String {
    let mut result = String::new();
    collect_layout_text_recursive(node, &mut result);
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn collect_layout_text_recursive(node: &Node, output: &mut String) {
    if node.tag == "#text" {
        output.push_str(&node.text);
        output.push(' ');
    }
    if node.tag == "br" {
        output.push('\n');
    }
    for child in &node.children {
        collect_layout_text_recursive(child, output);
    }
}


/////////////////////////
// End of file
/////////////////////////