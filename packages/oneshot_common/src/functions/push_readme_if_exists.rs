use std::io;
use std::path::Path;

use fs_err::read_to_string;

use oneshot_utils::functions::file_maybe_if_exists::source_file_maybe_if_exists;

use crate::functions::readme::readme_path_buf;

pub fn push_readme_if_exists(strings: &mut Vec<String>, package_root: &Path) -> io::Result<()> {
    let readme = readme_path_buf(package_root);
    if readme.exists() {
        strings.push(read_to_string(readme)?)
    }
    Ok(())
}

pub fn push_readme_maybe_if_exists(parts: &mut Vec<String>, package_root_opt: Option<impl AsRef<Path>>) -> io::Result<()> {
    if let Some(package_root) = package_root_opt {
        let readme = readme_path_buf(package_root.as_ref());
        if readme.exists() {
            parts.push(read_to_string(readme)?)
        }
    }
    Ok(())
}

pub fn readme_maybe_if_exists<T: AsRef<Path>>(package_root_opt: Option<T>) -> io::Result<Option<String>> {
    source_file_maybe_if_exists(package_root_opt, readme_path_buf)
}
