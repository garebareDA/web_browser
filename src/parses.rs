#[derive(Debug)]
struct Attribute {
    name: String,
    contents: String,
}

#[derive(Debug)]
pub struct Nodes {
    tag_name: String,
    text: String,
    attributes: Vec<Attribute>,
    child: Vec<Nodes>,
}

pub struct Html {
    pub html: String,
    pub tag: Vec<String>,
}

pub fn parse_node(mut html: &mut Html) -> Nodes {
    let mut nodes: Nodes = Nodes {
        tag_name: "".to_string(),
        text: "".to_string(),
        attributes: Vec::new(),
        child: Vec::new(),
    };

    if html.html.len() < 1 {
        return nodes;
    }

    if html.html.chars().nth(0).unwrap() == '<' {
        if html.html.chars().nth(1).unwrap() == '/' {
            loop {
                remove_close_tag(&mut html);
                if html.html.len() == 0 || html.html.chars().nth(1).unwrap() != '/' {
                    break;
                }
            }

            return nodes;
        }

        let (tag_name, tag_text, node, attr) = parse_element(&mut html);

        nodes.tag_name = tag_name;
        nodes.text = tag_text;

        if !attr.is_empty() {
            nodes.attributes = attr;
        }

        if node.tag_name != "" {
            nodes.child.push(node);
        }

        if !nodes.child.is_empty()
            && html.tag.len() > 0
            && nodes.child[0].tag_name == html.tag[html.tag.len() - 1]
        {
            let node = parse_node(&mut html);
            if node.tag_name != "" {
                nodes.child.push(node);
            }
        }

        loop {
            if html.html.len() == 0 {
                break;
            }

            if nodes.tag_name == "html" {
                let node = parse_node(&mut html);
                nodes.child.push(node);
            } else {
                break;
            }
        }
    }

    return nodes;
}

fn parse_element(mut html: &mut Html) -> (String, String, Nodes, Vec<Attribute>) {
    let mut tag_name = "".to_string();
    let mut text = "".to_string();
    let mut nodes: Nodes = Nodes {
        tag_name: "".to_string(),
        text: "".to_string(),
        attributes: Vec::new(),
        child: Vec::new(),
    };
    html.html.remove(0);

    loop {
        let contents = html.html.chars().nth(0).unwrap();
        if contents == ' ' || contents == '>' {
            break;
        }
        tag_name = format!("{}{}", tag_name, contents);
        html.html.remove(0);
    }

    let attrs = parse_attribute(&mut html);

    html.html.remove(0);

    if html.html.chars().nth(0).unwrap() != '<' {
        text = parse_text(&mut html);
    }

    if tag_judgment(&tag_name) {
        html.tag.push(tag_name.clone());
        nodes = parse_node(&mut html);
    }

    return (tag_name, text, nodes, attrs);
}

fn parse_text(html: &mut Html) -> String {
    let mut tag_text = "".to_string();

    loop {
        if html.html.chars().nth(0).unwrap() == '<' {
            break;
        }

        let contents = html.html.chars().nth(0).unwrap();
        tag_text = format!("{}{}", tag_text, contents);
        html.html.remove(0);
    }

    return tag_text;
}

fn remove_close_tag(html: &mut Html) {
    html.html.remove(0);
    html.html.remove(0);

    loop {
        if html.html.len() == 0 || html.html.chars().nth(0).unwrap() == '<' {
            break;
        }
        html.html.remove(0);
    }

    html.tag.remove(html.tag.len() - 1);
}

fn parse_attribute(html: &mut Html) -> Vec<Attribute> {
    let mut attr_vec: Vec<Attribute> = Vec::new();

    loop {
        let mut attr = Attribute {
            name: "".to_string(),
            contents: "".to_string(),
        };

        if html.html.len() < 1 {
            break;
        }

        if html.html.chars().nth(0).unwrap() == '>' {
            break;
        }

        if html.html.chars().nth(0).unwrap() == ' ' {
            html.html.remove(0);
        }

        if html.html.chars().nth(0).unwrap() != '=' {
            let mut name = "".to_string();

            loop {
                let next_char = html.html.chars().nth(0).unwrap();
                if next_char == '>' || next_char == '=' {
                    break;
                }

                name = format!("{}{}", name, next_char);
                html.html.remove(0);
            }

            attr.name = name;
        }

        if html.html.chars().nth(0).unwrap() == '=' {
            let mut contents = "".to_string();
            html.html.remove(0);

            loop {
                let next_char = html.html.chars().nth(0).unwrap();
                if next_char == ' ' || next_char == '>' {
                    break;
                }

                contents = format!("{}{}", contents, next_char);
                html.html.remove(0);
            }

            attr.contents = contents;
        }

        println!("{:?}", attr);
        attr_vec.push(attr);
    }

    return attr_vec;
}

fn tag_judgment(tag: &str) -> bool {
    match tag{
        "area" => false,
        "base" => false,
        "br" => false,
        "col" => false,
        "embed" => false,
        "hr" => false,
        "img" => false,
        "input" => false,
        "link" => false,
        "meta" => false,
        "param" => false,
        "source" => false,
        "track" => false,
        "wbr" => false,
        _ => true,
    }
}