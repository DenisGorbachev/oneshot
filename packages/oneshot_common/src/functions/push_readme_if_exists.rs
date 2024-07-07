use std::io;
use std::path::Path;

use fs_err::read_to_string;

use crate::functions::readme::readme_path_buf;

pub fn push_readme_if_exists(strings: &mut Vec<String>, package_root: &Path) -> io::Result<()> {
    let readme = readme_path_buf(package_root);
    if readme.exists() {
        strings.push(read_to_string(readme)?)
    }
    Ok(())
}

pub fn push_readme_maybe_if_exists(
    parts: &mut Vec<String>,
    package_root_opt: Option<&Path>,
) -> io::Result<()> {
    if let Some(package_root) = package_root_opt {
        let readme = readme_path_buf(package_root);
        if readme.exists() {
            parts.push(read_to_string(readme)?)
        }
    }
    Ok(())
}

pub fn readme_maybe_if_exists(package_root_opt: Option<&Path>) -> io::Result<Option<String>> {
    package_root_opt
        .map(readme_path_buf)
        .and_then(|readme| {
            if readme.exists() {
                Some(read_to_string(readme))
            } else {
                None
            }
        })
        .transpose()
}
