use std::io;
use std::path::{Path, PathBuf, StripPrefixError};

use derive_more::{Error, From};
use fmt_derive::Display;

use crate::functions::find_package_root::{GetPackageRootError, get_package_root};

pub fn get_path_buf_relative_to_package_root(input: impl AsRef<Path>) -> Result<PathBuf, GetPathBufRelativeToPackageRootError> {
    let absolute_input = input.as_ref().canonicalize()?;
    let package_root = get_package_root(&absolute_input)?;
    let relative_path = absolute_input.strip_prefix(package_root)?;
    Ok(relative_path.to_path_buf())
}

#[derive(Error, Display, From, Debug)]
pub enum GetPathBufRelativeToPackageRootError {
    TheIoError(io::Error),
    TheStripPrefixError(StripPrefixError),
    TheGetPackageRootError(GetPackageRootError),
}
