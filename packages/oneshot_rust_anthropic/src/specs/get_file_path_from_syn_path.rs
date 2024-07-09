use std::path::PathBuf;

/// A Rust file may contain multiple items.
/// Each item may refer to multiple other items by their paths.
/// Paths may be located in visibility markers, attributes, macros, and use items ([see reference](https://doc.rust-lang.org/reference/paths.html))
/// Some paths are shortened by the `use` statements, which are specified at the top of the file.
/// It is not enough to parse the `use` statements - we need to recursively traverse all items to extract the paths.
/// Some paths may refer to items in external crates. In this case, it is necessary to resolve the
///
/// The argument [`path`](syn::Path) must be a global, fully resolved path (so that there is no need to check the use statements).
pub fn get_file_path_buf_from_syn_path(path: syn::Path) -> PathBuf {
    todo!()
}
