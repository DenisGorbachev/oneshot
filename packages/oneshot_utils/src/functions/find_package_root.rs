use std::io;
use std::path::{Path, PathBuf};

use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;

use crate::constants::CARGO_TOML_FILE_NAME;
use crate::functions::find_dir_canonical::find_dir_with_file_canonical;
use crate::functions::find_parent_containing_filename::find_parent_containing_filename;

pub fn find_package_root(anchor: &Path) -> io::Result<Option<PathBuf>> {
    find_dir_with_file_canonical(anchor, CARGO_TOML_FILE_NAME)
}

pub fn find_workspace_root(start: &Path) -> io::Result<Option<PathBuf>> {
    let package_root_opt = find_package_root(start)?;
    let package_root_path_opt = package_root_opt.as_deref();
    Ok(find_workspace_root_from_canonical_package_root_opt(package_root_path_opt))
}

pub fn find_workspace_root_from_canonical_package_root_opt(package_root_path_opt: Option<&Path>) -> Option<PathBuf> {
    package_root_path_opt
        .and_then(|package_root| find_parent_containing_filename(package_root, CARGO_TOML_FILE_NAME))
        .map(Path::to_path_buf)
}

pub fn get_package_root(start: &Path) -> Result<PathBuf, GetPackageRootError> {
    let package_root_opt = find_package_root(start)?;
    package_root_opt.ok_or(PackageRootNotFoundError.into())
}

#[derive(Error, Display, From, Debug)]
pub enum GetPackageRootError {
    TheIoError(io::Error),
    ThePackageRootNotFoundError(PackageRootNotFoundError),
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct PackageRootNotFoundError;
