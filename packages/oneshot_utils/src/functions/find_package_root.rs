use std::path::Path;

use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

use crate::constants::CARGO_TOML_FILE_NAME;
use crate::functions::find_parent_containing_filename::find_parent_containing_filename;

pub fn find_package_root(start: &Path) -> Option<&Path> {
    find_parent_containing_filename(start, CARGO_TOML_FILE_NAME)
}

pub fn get_package_root(start: &Path) -> Result<&Path, PackageRootNotFoundError> {
    find_package_root(start).ok_or(PackageRootNotFoundError)
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct PackageRootNotFoundError;
