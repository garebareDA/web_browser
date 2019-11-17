#[derive(Debug)]
struct Nodes {
    tag_name: String,
    text: String,
    tree: Vec<Nodes>,
}

struct Html {
    html: String,
    tag: Vec<String>,
    innner: String,
}

fn main() {
    let html = "<html><body>hello world<p>aaaaa<p>dddddd</p><p>ffff<div>eeeeeee</div></p></p><p>bbbbbbbb</p><p>cccc</p></body></html>".to_string();
    let mut html = Html{
        html:html,
        tag:Vec::new(),
        innner:"".to_string(),
    };

    let node = parse_node(&mut html);

    println!("{:?}", node);
}

fn parse_node(mut html: &mut Html) -> Nodes {

    let mut nodes: Nodes = Nodes {
        tag_name: "".to_string(),
        text: "".to_string(),
        tree: Vec::new(),
    };

    if html.html.chars().nth(0).unwrap() == '<' {
        if html.html.chars().nth(1).unwrap() == '/' {
            return nodes;
        }

        let (tag_name, tag_text, node) = parse_element(&mut html);

        nodes.tag_name = tag_name;
        nodes.text = tag_text;

        if node.tag_name != "" {
            nodes.tree.push(node);
        }
    }

    return nodes;
}

fn parse_element(mut element: &mut Html) -> (String, String, Nodes) {
    let mut tag_name = "".to_string();
    let mut text = "".to_string();
    let chars = "abcdefghijklmnopqrstuvwxyz";
    element.html.remove(0);

    while chars.contains(element.html.chars().nth(0).unwrap()) {
        let contents = element.html.chars().nth(0).unwrap();
        tag_name = format!("{}{}", tag_name, contents);
        element.html.remove(0);
    }
    element.html.remove(0);

    if element.html.chars().nth(0).unwrap() != '<' {
        text = parse_text(&mut element);
    }

    let mut nodes = parse_node(&mut element);

    println!("{}", element.html);

    return (tag_name, text, nodes);
}

fn parse_text(text: &mut Html) -> String {
    let mut tag_text = "".to_string();

    loop {
        if text.html.chars().nth(0).unwrap() == '<' {
            break;
        }

        let contents = text.html.chars().nth(0).unwrap();
        tag_text = format!("{}{}", tag_text, contents);
        text.html.remove(0);
    }

    return tag_text;
}