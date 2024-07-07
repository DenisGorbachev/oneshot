use std::path::{Path, PathBuf};

use crate::constants::README;

#[inline(always)]
pub fn readme_path_buf(package_root: &Path) -> PathBuf {
    package_root.join(README)
}

pub fn readme_path_buf_maybe(package_root_opt: Option<&Path>) -> Option<PathBuf> {
    package_root_opt.map(readme_path_buf)
}
