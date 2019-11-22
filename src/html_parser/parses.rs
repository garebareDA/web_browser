use super::structs::{Attribute, Html, Nodes, Style};
use super::tags::{remove_close_tag, tag_judgment};

pub fn parse_node(mut html: &mut Html) -> Nodes {
    let mut nodes: Nodes = Nodes::new();

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

        nodes.tag_name = tag_name.trim().to_string();
        nodes.text = tag_text.trim().to_string();

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
    let mut tag_name = String::new();
    let mut text = String::new();
    let mut nodes = Nodes::new();
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
    let mut tag_text = String::new();

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

fn parse_attribute(html: &mut Html) -> Vec<Attribute> {
    let mut attr_vec: Vec<Attribute> = Vec::new();

    loop {
        let mut attr = Attribute::new();

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
            let mut name = String::new();

            loop {
                let next_char = html.html.chars().nth(0).unwrap();
                if next_char == '>' || next_char == '=' {
                    break;
                }

                name = format!("{}{}", name, next_char);
                html.html.remove(0);
            }

            if name == "style" {
                attr.style = perse_style(html);
            }

            attr.name = name.trim().to_string();
        }

        if html.html.chars().nth(0).unwrap() == '=' {
            let mut contents = String::new();
            html.html.remove(0);
            html.html.remove(0);

            loop {
                let next_char = html.html.chars().nth(0).unwrap();
                if next_char == ' ' || next_char == '>' {
                    break;
                } else if next_char == '"' {
                    html.html.remove(0);
                    break;
                }

                contents = format!("{}{}", contents, next_char);
                html.html.remove(0);
            }

            attr.contents = contents.trim().to_string();
        }

        println!("{:?}", attr);
        attr_vec.push(attr);
    }

    return attr_vec;
}

fn perse_style(html: &mut Html) -> Vec<Style> {
    let mut style_vec: Vec<Style> = Vec::new();

    if html.html.chars().nth(0).unwrap() == '=' {
        html.html.remove(0);
    } else {
        return style_vec;
    }
    html.html.remove(0);

    loop {
        let mut style = Style::new();
        let mut name = String::new();
        let mut contents = String::new();

        if html.html.chars().nth(0).unwrap() == '>' {
            break;
        }

        while html.html.chars().nth(0).unwrap() != ':' {
            let contents = html.html.chars().nth(0).unwrap();
            name = format!("{}{}", name, contents);
            html.html.remove(0);
        }

        if html.html.chars().nth(0).unwrap() == ':' {
            html.html.remove(0);
            loop {
                let chars = html.html.chars().nth(0).unwrap();
                if chars == ';' {
                    html.html.remove(0);
                    break;
                } else if chars == '>' {
                    break;
                }
                contents = format!("{}{}", contents, chars);
                html.html.remove(0);
            }
        }

        contents.pop();
        style.name = name.trim().to_string();
        style.contents = contents.trim().to_string();
        style_vec.push(style);
    }

    return style_vec;
}
