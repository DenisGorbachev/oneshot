use std::path::{Path, PathBuf};

use syn::UseTree;

/// # Arguments
///
/// * `file_path` - the path of the file that contains the [`use_trees`](UseTree)
/// * `use_trees` - the use trees that were extracted from the file specified by the path
pub fn into_file_paths(file_path: &Path, use_trees: Vec<UseTree>) -> anyhow::Result<Vec<PathBuf>> {
    todo!()
}

pub mod v1;
