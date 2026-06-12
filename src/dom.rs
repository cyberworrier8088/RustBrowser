use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

#[derive(Default)]
pub struct Document {
    pub elements: Vec<Element>,
}

pub enum Element {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph(String),
    Link { text: String, url: String },
    ListIteam(String),
}

impl Document {
    pub fn from_message(message: String) -> Self {
        Self {
            elements: vec![Element::Paragraph(message)],
        }
    }
}

pub fn parse_html(html: &str) -> Document {
    let dom = parse_document(RcDom::default(), Default::default()).one(html);
    let mut elements = Vec::new();

    collect_elements(&dom.document, &mut elements);

    Document { elements }
}

fn collect_elements(handle: &Handle, elements: &mut Vec<Element>) {
    if let NodeData::Element { name, attrs, .. } = &handle.data {
        let tag = name.local.as_ref();
        let text = collect_text(handle);
        let text = clean_text(&text);

        if text.is_empty() {
            return;
        }

        match tag {
            "h1" => elements.push(Element::Heading {
                level: 1,
                text,
            }),
            "h2" => elements.push(Element::Heading {
                level: 2,
                text,
            }),
            "h3" => elements.push(Element::Heading {
                level: 3,
                text,
            }),
            "h4" => elements.push(Element::Heading {
                level: 4,
                text,
            }),
            "h5" => elements.push(Element::Heading {
                level: 5,
                text,
            }),
            "h6" => elements.push(Element::Heading {
                level: 6,
                text,
            }),
            "p" => elements.push(Element::Paragraph(text)),
            "li" => elements.push(Element::ListIteam(text)),
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
            _ => {}
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
