use web_browser::parses;

fn main() {
    let html = "<html><body>hello world<p>aaaaa<p>dddddd</p><p>ffff<div>eeeeeee</div></p></p><p>bbbbbbbb<p>hhhhhh</p></p><p>cccc<p>ggggggg</p></p></body></html>".to_string();
    let mut html =parses::Html{
        html:html,
        tag:Vec::new(),
    };

    let node = parses::parse_node(&mut html);

    println!("{:?}", node);
}