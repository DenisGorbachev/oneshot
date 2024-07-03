use std::fs::read_to_string;
use std::io;
use std::path::Path;

use crate::constants::README;

pub fn read_readme(package_root: &Path) -> io::Result<String> {
    read_to_string(package_root.join(README).as_path())
}
