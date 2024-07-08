#![allow(dead_code)]

use fierce_nerds_node::node::Node;
use fierce_nerds_node::s;

pub fn all() -> Vec<Node<String>> {
    vec![
        s!("Rendered filenames must be relative to the current workspace"),
        s!("User must be able to fully control the output (there must be no println! in the code)"),
        s!("Code from LLM must be syntax-highlighted"),
    ]
}
