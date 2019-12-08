use crate::html_parser::structs::Nodes;

fn html_judg(mut nodes: &Nodes) {
    if !nodes.child.is_empty() {
        node_serch(&nodes.child);
    }
}

pub fn node_serch(child: &Vec<Nodes>){
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();

        if tag_name == "head" ||tag_name == "html"{
            continue;
        }

        if !child[index].child.is_empty() {
            node_serch(&child[index].child);
        }

    }
}
