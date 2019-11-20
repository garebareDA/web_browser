#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub contents: String,
}

impl Attribute {
    pub fn new() -> Attribute {
        return Attribute { name:"".to_string(), contents:"".to_string() };
    }
}

pub struct Html {
    pub html: String,
    pub tag: Vec<String>,
}

#[derive(Debug)]
pub struct Nodes {
    pub tag_name: String,
    pub text: String,
    pub attributes: Vec<Attribute>,
    pub child: Vec<Nodes>,
}

impl Nodes {
    pub fn new() -> Nodes {
        return Nodes {
            tag_name: "".to_string(),
            text: "".to_string(),
            attributes: Vec::new(),
            child: Vec::new(),
        };
    }
}
