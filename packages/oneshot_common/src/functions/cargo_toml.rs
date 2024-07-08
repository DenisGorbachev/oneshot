use std::io;
use std::path::{Path, PathBuf};

use oneshot_utils::constants::CARGO_TOML_FILE_NAME;
use oneshot_utils::functions::file_maybe_if_exists::source_file_maybe_if_exists;

#[inline(always)]
pub fn cargo_toml_path_buf(package_root: &Path) -> PathBuf {
    package_root.join(CARGO_TOML_FILE_NAME)
}

pub fn cargo_toml_maybe_if_exists(package_root_opt: Option<&Path>) -> io::Result<Option<String>> {
    source_file_maybe_if_exists(package_root_opt, cargo_toml_path_buf)
}
