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
        attributes:Vec::new(),
        child: Vec::new(),
    };

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

        let (tag_name, tag_text, node) = parse_element(&mut html);

        nodes.tag_name = tag_name;
        nodes.text = tag_text;

        if node.tag_name != "" {
            nodes.child.push(node);
        }

        if !nodes.child.is_empty() && html.tag.len() > 0 && nodes.child[0].tag_name == html.tag[html.tag.len() - 1] {
            let node = parse_node(&mut html);
            if node.tag_name != "" {
                nodes.child.push(node);
            }

        }

        loop{
            if html.html.len() == 0 {
                break;
            }

            if nodes.tag_name == "html" {
                let node = parse_node(&mut html);
                nodes.child.push(node);
            }else{
                break;
            }
        }
    }

    return nodes;
}

fn parse_element(mut html: &mut Html) -> (String, String, Nodes) {
    let mut tag_name = "".to_string();
    let mut text = "".to_string();
    let chars = "abcdefghijklmnopqrstuvwxyz";
    html.html.remove(0);

    while chars.contains(html.html.chars().nth(0).unwrap()) {
        let contents = html.html.chars().nth(0).unwrap();
        tag_name = format!("{}{}", tag_name, contents);
        html.html.remove(0);
    }
    html.html.remove(0);

    if html.html.chars().nth(0).unwrap() != '<' {
        text = parse_text(&mut html);
    }

    html.tag.push(tag_name.clone());
    let nodes = parse_node(&mut html);

    return (tag_name, text, nodes);
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

fn remove_close_tag( html: &mut Html) {
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