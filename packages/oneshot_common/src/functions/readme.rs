use std::path::{Path, PathBuf};

use crate::constants::README;

#[inline(always)]
pub fn readme(package_root: &Path) -> PathBuf {
    package_root.join(README)
}
