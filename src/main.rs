struct Nodes {
    tag_name: String,
    text: String,
    tree: Vec<Nodes>,
}

fn main() {
    let html = "<html><body>hello world</body></html>".to_string();
    let node = parse_node(html);
}

fn parse_node(html: String) -> Nodes {
    let NodeVec:Vec<Nodes> = Vec::new();

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
        //TODO 配列にPush
        } else {
            break;
        }
    }

    return nodes;
}

fn parse_element(element: &str) {
    let mut element = element.to_string();
    let mut tag_name = "".to_string();
    let chars = "abcdefghijklmnopqrstuvwxyz";
    element.remove(0);

    let mut element = element.to_string();
    while chars.contains(element.chars().nth(0).unwrap()) {
        let contents = element.chars().nth(0).unwrap();
        tag_name = format!("{}{}", tag_name, contents);
        element.remove(0);
    }
    element.remove(0);

    let nodes = parse_node(element.clone());
}
