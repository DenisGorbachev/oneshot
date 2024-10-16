#![allow(dead_code)]

pub fn get_get_items_referenced_by_file() {
    // rust-analyzer has implemented go-to-definition, so it must maintain this map between a path (or a symbol) and a location in the filesystem
    // Identifier resolution is very complex; I think it is necessary to use the rust-analyzer
    // I need to check out the RA API
    todo!()
}

pub fn unsorted() -> Vec<&'static str> {
    vec![
        "Ensure there are no uses of std::fs (replace it with fs_err)",
        "Rendered filenames must be relative to the current workspace",
        "User must be able to fully control the output (there must be no println! in the code)",
        "Code from LLM must be syntax-highlighted",
    ]
}
