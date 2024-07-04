use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

use derive_getters::{Dissolve, Getters};
use derive_new::new;

/// `content` is not guaranteed to come from `path_buf` because it is possible to call `SourceFile::new` with any arguments (this is intentional)
#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceFile {
    #[new(into)]
    path_buf: PathBuf,
    #[new(into)]
    content: String,
}

impl SourceFile {
    pub fn from_path_buf(path_buf: impl Into<PathBuf>) -> io::Result<Self> {
        let path_buf = path_buf.into();
        let content = read_to_string(path_buf.as_path())?;
        Ok(Self { path_buf, content })
    }

    #[cfg(all(feature = "serde", feature = "serde-xml-rs"))]
    pub fn serialize_to_xml(&self) -> Result<String, serde_xml_rs::Error> {
        serde_xml_rs::to_string(self)
    }
}

// #[derive(Error, Display, From, Debug)]
// pub enum SourceFileFromPathBufError {
//     TheIoError(io::Error),
//     TheGetPathBufRelativeToPackageRootError(GetPathBufRelativeToPackageRootError),
// }

impl<'a> TryFrom<&'a Path> for SourceFile {
    type Error = io::Error;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        Self::from_path_buf(path)
    }
}

impl TryFrom<PathBuf> for SourceFile {
    type Error = io::Error;

    fn try_from(path_buf: PathBuf) -> Result<Self, Self::Error> {
        Self::from_path_buf(path_buf)
    }
}
