// src/dom.rs :)


// dom: Document Object Model :)
// introduction of dom: document object model is a programming interface for web documents.



/////////////////////////
// top of file
/////////////////////////




// using libraries
use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

// derive use for make default values
#[derive(Default)]
pub struct Document {
    pub root: Option<Node>,
}


#[derive(Clone, Debug)]
pub struct Node {
    pub tag: String,
    pub text: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Node>,
}

pub enum Element {
    Heading { level: u8, text: String },
    Paragraph(String),
    Link { text: String, url: String },
    ListIteam(String),

    Bold(String),
    Italic(String),

    Image {
        src: String,
        alt: String,
    },

    HorizontalRule,

    TableRow(Vec<String>),
}

impl Document {
    pub fn elements(&self) -> Vec<Element> {
        let mut elements = Vec::new();

        if let Some(root) = &self.root {
            collect_from_node(root, &mut elements);
        }
        elements
    }

    pub fn from_message(message: String) -> Self {
        Self {
            root: Some(Node {
                tag: "message".to_string(),
                text: message,
                attributes: Vec::new(),
                children: Vec::new(),
            })
        }
    }
}

// collect text from a Node tree mirrors the old collect_text/append_text for RcDom
fn collect_node_text(node: &Node) -> String {
    let mut text = String::new();
    append_node_text(node, &mut text);
    text
}

fn append_node_text(node: &Node, text: &mut String) {
    if node.tag == "br" {
        text.push('\n');
    }
    if node.tag == "#text" {
        text.push_str(&node.text);
        text.push(' ');
    }
    for child in &node.children {
        append_node_text(child, text);
    }
}

// compatibility layer: convert Node tree ->>> Vec<Element> for the existing renderer
fn collect_from_node(node: &Node, elements: &mut Vec<Element>) {
    let tag = node.tag.as_str();
    let text = collect_node_text(node);
    let text = clean_text(&text);

    if !text.is_empty() || tag == "img" || tag == "hr" || tag == "message" {
        match tag {
            "h1" | "heading1" => elements.push(Element::Heading { level: 1, text: text.clone() }),
            "h2" | "heading2" => elements.push(Element::Heading { level: 2, text: text.clone() }),
            "h3" => elements.push(Element::Heading { level: 3, text: text.clone() }),
            "h4" => elements.push(Element::Heading { level: 4, text: text.clone() }),
            "h5" => elements.push(Element::Heading { level: 5, text: text.clone() }),
            "h6" => elements.push(Element::Heading { level: 6, text: text.clone() }),
            "p" | "message" => elements.push(Element::Paragraph(text.clone())),
            "li" => elements.push(Element::ListIteam(text.clone())),
            "b" | "strong" => elements.push(Element::Bold(text.clone())),
            "i" | "em" => elements.push(Element::Italic(text.clone())),
            "img" => {
                let src = node.attributes.iter()
                    .find(|(k, _)| k == "src")
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default();
                let alt = node.attributes.iter()
                    .find(|(k, _)| k == "alt")
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default();
                println!("Found image: {}", src);
                elements.push(Element::Image { src, alt });
            }
            "a" => {
                if let Some(url) = node.attributes.iter()
                    .find(|(k, _)| k == "href")
                    .map(|(_, v)| v.clone())
                {
                    elements.push(Element::Link { text: text.clone(), url });
                }
            }
            "hr" => elements.push(Element::HorizontalRule),
            "tr" => {
                let mut cells = Vec::new();
                for child in &node.children {
                    if child.tag == "td" || child.tag == "th" {
                        let cell_text = clean_text(&collect_node_text(child));
                        cells.push(cell_text);
                    }
                }
                if !cells.is_empty() {
                    elements.push(Element::TableRow(cells));
                }
            }
            _ => {}
        }
    }

    // For tags that were already handled as leaf elements, don't recurse
    // (otherwise we'd get duplicate text from children)
    match tag {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "heading1" | "heading2"
        | "p" | "message" | "li" | "b" | "strong" | "i" | "em"
        | "img" | "a" | "hr" | "tr" => {}
        _ => {
            for child in &node.children {
                collect_from_node(child, elements);
            }
        }
    }
}

pub fn parse_html(html: &str) -> Document {
    let dom = parse_document(
        RcDom::default(),
        Default::default(),
    ).one(html);

    Document {
        root: Some(build_node(&dom.document)),
    }
}

fn build_node(handle: &Handle) -> Node {
    let mut node = Node {
        tag: String::new(),
        text: String::new(),
        attributes: Vec::new(),
        children: Vec::new(),
    };

    match &handle.data {
        NodeData::Element { name, attrs, .. } => {
            node.tag = name.local.to_string();
            for attr in attrs.borrow().iter() {
                node.attributes.push((
                    attr.name.local.to_string(),
                    attr.value.to_string(),
                ));
            }
        }

        NodeData::Text { contents } => {
            node.tag = "#text".to_string();
            node.text = contents.borrow().to_string();
        }

        _ => {
            node.tag = "#document".to_string();
        }
    }

    for child in handle.children.borrow().iter() {
        node.children.push(build_node(child));
    }

    node
}


fn collect_elements(handle: &Handle, elements: &mut Vec<Element>) {
    if let NodeData::Element { name, attrs, .. } = &handle.data {
        let tag = name.local.as_ref();
        let text = collect_text(handle);
        let text = clean_text(&text);

        if !text.is_empty() || tag == "img" || tag == "hr" {
            match tag {
                "h1" | "heading1" => elements.push(Element::Heading { level: 1, text }),
                "h2" | "heading2" => elements.push(Element::Heading { level: 2, text }),
                "h3" => elements.push(Element::Heading { level: 3, text }),
                "h4" => elements.push(Element::Heading { level: 4, text }),
                "h5" => elements.push(Element::Heading { level: 5, text }),
                "h6" => elements.push(Element::Heading { level: 6, text }),
                "p" => elements.push(Element::Paragraph(text)),
                "li" => elements.push(Element::ListIteam(text)),
                "b" | "strong" => elements.push(Element::Bold(text)),
                "i" | "em" => elements.push(Element::Italic(text)),
                "img" => {
                    let src = attrs.borrow().iter().find(|attr| attr.name.local.as_ref() == "src").map(|attr| attr.value.to_string()).unwrap_or_default();
                    let alt = attrs.borrow().iter().find(|attr| attr.name.local.as_ref() == "alt").map(|attr| attr.value.to_string()).unwrap_or_default();
                    println!("Found image: {}", src);
                    elements.push(Element::Image { src, alt })
                }
                "a" => {
                    if let Some(url) = attrs
                        .borrow()
                        .iter()
                        .find(|attr| attr.name.local.as_ref() == "href")
                        .map(|attr| attr.value.to_string())
                    {
                        elements.push(Element::Link { text, url });
                    }
                }
                "hr" => elements.push(Element::HorizontalRule),
                "tr" => {
                    let mut cells = Vec::new();

                    for child in handle.children.borrow().iter() {
                        if let NodeData::Element { name, .. } = &child.data {
                            let child_tag = name.local.as_ref();

                            if child_tag == "td" || child_tag == "th" {
                                let text = clean_text(&collect_text(child));

                                cells.push(text);
                            }
                        }
                    }

                    if !cells.is_empty() {
                        elements.push(Element::TableRow(cells));
                    }
                }
                _ => {}
            }
        }
    }

    for child in handle.children.borrow().iter() {
        collect_elements(child, elements);
    }
}

fn collect_text(handle: &Handle) -> String {
    let mut text = String::new();

    append_text(handle, &mut text);
    text
}

fn append_text(handle: &Handle, text: &mut String) {
    if let NodeData::Element { name, .. } = &handle.data {
        if name.local.as_ref() == "br" {
            text.push('\n');
        }
    }

    if let NodeData::Text { contents } = &handle.data {
        text.push_str(&contents.borrow());
        text.push(' ');
    }

    for child in handle.children.borrow().iter() {
        append_text(child, text);
    }
}

fn clean_text(text: &str) -> String {
    text.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n")
}




////////////////////////
// End of file
////////////////////////
