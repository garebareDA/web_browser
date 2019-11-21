#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub contents: String,
    pub style: Vec<Style>,
}

impl Attribute {
    pub fn new() -> Attribute {
        return Attribute { name:String::new(), contents:String::new(), style: Vec::new()};
    }
}

#[derive(Debug)]
pub struct Style {
    pub name: String,
    pub contents: String,
}

impl Style {
    pub fn new() -> Style {
        return Style { name:String::new(), contents:String::new()};
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
            tag_name: String::new(),
            text: String::new(),
            attributes: Vec::new(),
            child: Vec::new(),
        };
    }
}
