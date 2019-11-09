fn main() {
    let html = "<html><body>hello world</body></html>".to_string();
    let node = parse_node(html);

    println!("{}", node[0][0]);

}

fn parse_node(html: String) -> Vec<Vec<String>> {
   let mut nodes: Vec<Vec<String>> = Vec::new();

   if html.len() == 0 {
        return nodes;
   }

   for index in 0..html.len() {

       if index == html.len() {
           break;
       }

       if html.chars().nth(index).unwrap() == '<'{

           if html.chars().nth(index + 1).unwrap() == '/' {
                break;
           }
           nodes.push(parse_element(&html));
       }else{
            break;
       }
   }

   return nodes;
}

fn parse_element(element: &str) -> Vec<String> {
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
    let parse = vec![tag_name, nodes[0][0].clone()];

    return parse
}