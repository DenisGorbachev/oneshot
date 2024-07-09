use std::path::{Path, PathBuf};

use anyhow::Result;
use syn::{UseGroup, UsePath, UseTree};

/// Converts use trees into file paths relative to the package root.
///
/// # Arguments
///
/// * `package_root` - the root path of the package
/// * `use_trees` - the use trees that were extracted from the file specified by the path
///
/// The paths that are prefixed by `crate::` must be
pub fn into_file_paths_v1(package_root: &Path, use_trees: Vec<UseTree>) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    for use_tree in use_trees {
        match use_tree {
            UseTree::Path(UsePath {
                ident,
                tree,
                ..
            }) => {
                let mut current_path = PathBuf::from(ident.to_string());
                process_use_tree(*tree, &mut current_path, &mut file_paths)?;
            }
            UseTree::Group(UseGroup {
                items,
                ..
            }) => {
                for item in items {
                    let mut current_path = PathBuf::new();
                    process_use_tree(item, &mut current_path, &mut file_paths)?;
                }
            }
            _ => continue, // Skip other types of UseTree
        }
    }

    // Convert relative paths to absolute paths based on package root
    let absolute_paths: Vec<PathBuf> = file_paths
        .into_iter()
        .map(|path| package_root.join(path).with_extension("rs"))
        .collect();

    Ok(absolute_paths)
}

pub fn process_use_tree(tree: UseTree, current_path: &mut PathBuf, file_paths: &mut Vec<PathBuf>) -> Result<()> {
    match tree {
        UseTree::Path(UsePath {
            ident,
            tree,
            ..
        }) => {
            current_path.push(ident.to_string());
            process_use_tree(*tree, current_path, file_paths)?;
            current_path.pop();
        }
        UseTree::Name(_) | UseTree::Glob(_) => {
            file_paths.push(current_path.clone());
        }
        UseTree::Group(UseGroup {
            items,
            ..
        }) => {
            for item in items {
                process_use_tree(item, current_path, file_paths)?;
            }
        }
        UseTree::Rename(_) => {
            // skip UseTree::Rename
        }
    }
    Ok(())
}
