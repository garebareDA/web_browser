use super::structs::Html;

pub fn remove_close_tag(html: &mut Html) {
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

pub fn tag_judgment(tag: &str) -> bool {
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