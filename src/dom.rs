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


impl Document {

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





////////////////////////
// End of file
////////////////////////
