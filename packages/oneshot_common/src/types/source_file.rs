use std::io;
use std::path::{Path, PathBuf};

use derive_getters::{Dissolve, Getters};
use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;
use fs_err::read_to_string;

/// `content` is not guaranteed to come from `path_buf` because it is possible to call `SourceFile::new` with any arguments (this is intentional)
#[derive(new, Getters, Dissolve, From, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SourceFile {
    #[new(into)]
    #[serde(rename = "path")]
    path_buf: PathBuf,
    #[new(into)]
    content: String,
}

impl SourceFile {
    pub fn from_path_buf(path_buf: impl Into<PathBuf>) -> io::Result<Self> {
        let path_buf = path_buf.into();
        let content = read_to_string(path_buf.as_path())?;
        Ok(Self {
            path_buf,
            content,
        })
    }

    pub fn from_path_buf_if_exists(path_buf: impl Into<PathBuf>) -> Option<io::Result<Self>> {
        let path_buf = path_buf.into();
        if path_buf.exists() { Some(Self::from_path_buf(path_buf)) } else { None }
    }

    pub fn from_path_bufs<P: Into<PathBuf>>(path_bufs: impl IntoIterator<Item = P>) -> io::Result<Vec<Self>> {
        path_bufs.into_iter().map(Self::from_path_buf).collect()
    }

    pub fn to_xml(&self) -> Result<String, quick_xml::DeError> {
        quick_xml::se::to_string(self)
    }

    pub fn from_path_buf_to_xml_if_exists(path_buf: impl Into<PathBuf>) -> Result<Option<String>, FromPathBufToXmlIfExists> {
        let source_file_opt = SourceFile::from_path_buf_if_exists(path_buf).transpose()?;
        let content_xml_opt = source_file_opt
            .as_ref()
            .map(SourceFile::to_xml)
            .transpose()?;
        Ok(content_xml_opt)
    }

    pub fn from_path_bufs_to_xml<P: Into<PathBuf>>(path_bufs: impl IntoIterator<Item = P>) -> Result<Vec<String>, FromPathBufToXmlIfExists> {
        let source_files = Self::from_path_bufs(path_bufs)?;
        source_files
            .into_iter()
            .map(|file| file.to_xml().map_err(FromPathBufToXmlIfExists::from))
            .collect()
    }

    pub fn to_xml_many<'a>(files: impl IntoIterator<Item = &'a Self>) -> Result<Vec<String>, quick_xml::DeError> {
        files.into_iter().map(Self::to_xml).collect()
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

#[derive(Error, Display, From, Debug)]
pub enum FromPathBufToXmlIfExists {
    TheIoError(io::Error),
    TheXmlError(quick_xml::DeError),
}
