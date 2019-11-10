#[derive(Debug)]
struct Nodes {
    tag_name: String,
    text: String,
    tree: Vec<Nodes>,
}

fn main() {
    let html = "<html><body>hello world<p>aaaaa</p><p>bbbbbbbb</p></body></html>";
    let node = parse_node(html);

    println!("{:?}", node);
}

fn parse_node(html: &str) -> Nodes {
    let NodeVec: Vec<Nodes> = Vec::new();
    let html = html.to_string();

    let mut nodes: Nodes = Nodes {
        tag_name: "".to_string(),
        text: "".to_string(),
        tree: NodeVec,
    };

    if html.len() == 0 {
        return nodes;
    }

    for index in 0..html.len() {
        if index == html.len() {
            break;
        }

        if html.chars().nth(index).unwrap() == '<' {
            if html.chars().nth(index + 1).unwrap() == '/' {
                break;
            }

            let (tag_name, tag_text, node) = parse_element(&mut html.to_string());
            nodes.tag_name = tag_name;
            nodes.text = tag_text;

            if node.tag_name != "" {
                nodes.tree.push(node);
            }

        }else {
            break;
        }
    }

    return nodes;
}

fn parse_element(mut element: &mut String) -> (String, String, Nodes) {
    let mut tag_name = "".to_string();
    let mut text = "".to_string();
    let chars = "abcdefghijklmnopqrstuvwxyz";
    element.remove(0);

    while chars.contains(element.chars().nth(0).unwrap()) {
        let contents = element.chars().nth(0).unwrap();
        tag_name = format!("{}{}", tag_name, contents);
        element.remove(0);
    }
    element.remove(0);

    if element.chars().nth(0).unwrap() != '<'{
        text = parse_text(&mut element);
    }

    let mut nodes = parse_node(&element);

    element.remove(0);
    element.remove(0);

    for index in 0..tag_name.len() {
        element.remove(0);
    }

    element.remove(0);

    println!("{}", element);

    return (tag_name, text, nodes);
}

fn parse_text(text: &mut String) -> String {
    let mut tag_text = "".to_string();

    loop {
        if text.chars().nth(0).unwrap() == '<' {
            break;
        }

        let contents = text.chars().nth(0).unwrap();
        tag_text = format!("{}{}", tag_text, contents);
        text.remove(0);
    }

    return tag_text;
}
