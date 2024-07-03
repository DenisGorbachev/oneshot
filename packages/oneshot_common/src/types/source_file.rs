use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

use derive_getters::{Dissolve, Getters};
use derive_more::{Error, From};
use fmt_derive::Display;

use oneshot_utils::functions::get_path_buf_relative_to_package_root::{
    get_path_buf_relative_to_package_root, GetPathBufRelativeToPackageRootError,
};

#[derive(Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceFile {
    // Relative to package root (needed for serialization)
    path_buf: PathBuf,
    content: String,
}

impl SourceFile {
    pub fn new(path: &Path) -> Result<Self, SourceFileNewError> {
        let path_buf = get_path_buf_relative_to_package_root(path)?;
        let content = read_to_string(path)?;
        Ok(Self { path_buf, content })
    }

    #[cfg(all(feature = "serde", feature = "serde-xml-rs"))]
    pub fn serialize_to_xml(&self) -> Result<String, serde_xml_rs::Error> {
        serde_xml_rs::to_string(self)
    }
}

#[derive(Error, Display, From, Debug)]
pub enum SourceFileNewError {
    TheIoError(io::Error),
    TheGetPathBufRelativeToPackageRootError(GetPathBufRelativeToPackageRootError),
}

impl<'a> TryFrom<&'a Path> for SourceFile {
    type Error = SourceFileNewError;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}
