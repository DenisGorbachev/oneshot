use std::path::{Path, PathBuf};

use derive_more::{Deref, DerefMut, Into};

use oneshot_utils::functions::get_path_buf_relative_to_package_root::{get_path_buf_relative_to_package_root, GetPathBufRelativeToPackageRootError};

/// A `PathBuf` that is relative to the current package
#[derive(Deref, DerefMut, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct PackagePathBuf {
    inner: PathBuf,
}

impl PackagePathBuf {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, GetPathBufRelativeToPackageRootError> {
        let inner = get_path_buf_relative_to_package_root(path)?;
        Ok(Self {
            inner,
        })
    }
}

pub mod impl_into_source_file;
