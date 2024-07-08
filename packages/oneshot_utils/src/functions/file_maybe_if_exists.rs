use std::io;
use std::path::{Path, PathBuf};

use crate::functions::read_to_string_if_exists::read_to_string_if_exists;

pub fn source_file_maybe_if_exists<T: AsRef<Path>>(parent_opt: Option<T>, to_path_buf: impl FnOnce(&Path) -> PathBuf) -> io::Result<Option<String>> {
    parent_opt
        .map(|parent| to_path_buf(parent.as_ref()))
        .and_then(read_to_string_if_exists)
        .transpose()
}
