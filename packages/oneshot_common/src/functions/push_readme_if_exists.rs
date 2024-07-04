use std::io;
use std::path::Path;

use fs_err::read_to_string;

use crate::functions::readme::readme;

pub fn push_readme_if_exists(package_root: &Path, strings: &mut Vec<String>) -> io::Result<()> {
    let readme = readme(package_root);
    if readme.exists() {
        strings.push(read_to_string(readme)?)
    }
    Ok(())
}
