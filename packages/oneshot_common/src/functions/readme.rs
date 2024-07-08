use std::path::{Path, PathBuf};

use oneshot_utils::constants::README_FILE_NAME;

#[inline(always)]
pub fn readme_path_buf(package_root: &Path) -> PathBuf {
    package_root.join(README_FILE_NAME)
}

pub fn readme_path_buf_maybe(package_root_opt: Option<&Path>) -> Option<PathBuf> {
    package_root_opt.map(readme_path_buf)
}
