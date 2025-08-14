use std::io;
use std::path::Path;

use fs_err::read_to_string;

pub fn read_to_string_if_exists(path: impl AsRef<Path>) -> Option<io::Result<String>> {
    if path.as_ref().exists() { Some(read_to_string(path)) } else { None }
}
