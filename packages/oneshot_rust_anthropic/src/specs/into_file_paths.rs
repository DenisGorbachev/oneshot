use std::path::{Path, PathBuf};

use oneshot_rust::functions::get_dependency_map::{DependencyMap, get_dependency_map};
use oneshot_utils::constants::CARGO_TOML_FILE_NAME;
use oneshot_utils::types::skip_last_iterator::SkipLast;

/// # Arguments
///
/// * `package_root` - the root of the package that contains the file from which the `paths` have been extracted
/// * `paths` - the use trees that were extracted from the file specified by the path
///
/// Note that the directories and files used for loading external file modules can be influenced with the `path` attribute on the module
pub fn into_file_paths(package_root: &Path, paths: Vec<syn::Path>) -> anyhow::Result<Vec<PathBuf>> {
    let manifest_path = package_root.join(CARGO_TOML_FILE_NAME);
    let dependency_map = get_dependency_map(manifest_path.as_path())?;
    paths
        .into_iter()
        .filter_map(|path| into_file_path(package_root, &path, &dependency_map).transpose())
        .collect()
}

/// This function returns `Ok(None)` if the `path` is empty
/// TODO: This function assumes that the directory with source files is named `src`, but it can be configured in a different way in Cargo.toml
pub fn into_file_path(package_root: &Path, path: &syn::Path, dependency_map: &DependencyMap) -> anyhow::Result<Option<PathBuf>> {
    // skip the last element (the item ident)
    let mut segments = path.segments.iter().skip_last();

    match segments.next() {
        None => Ok(None),
        Some(first_segment) => {
            let first_segment_ident = first_segment.ident.to_string();

            // Check if the first segment is a known dependency
            if let Some(crate_info) = dependency_map.get(&first_segment_ident) {
                let mut file_path = crate_info.path_buf().join("src");

                // Add remaining segments as directories
                for segment in segments {
                    file_path.push(segment.ident.to_string());
                }

                add_rs_suffix(&mut file_path);

                Ok(Some(file_path))
            } else {
                // If not a known dependency, assume it's a local module
                let mut file_path = package_root.to_path_buf().join("src");

                // Add all segments as directories
                for segment in segments {
                    file_path.push(segment.ident.to_string());
                }

                add_rs_suffix(&mut file_path);

                Ok(Some(file_path))
            }
        }
    }
}

fn add_rs_suffix(crate_src: &mut PathBuf) {
    // Replace last segment with .rs extension
    if let Some(file_name) = crate_src.file_name() {
        let mut file_name = file_name.to_os_string();
        file_name.push(".rs");
        crate_src.set_file_name(file_name);
    }
}

pub mod v1;
