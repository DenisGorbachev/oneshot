use std::io;
use std::path::PathBuf;

use derive_getters::{Dissolve, Getters};
use derive_new::new;

use oneshot_common::types::language::Language;
use oneshot_utils::functions::find_package_root::find_package_root;
use oneshot_utils::traits::maybe_from::MaybeFrom;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileInfo {
    path_buf: PathBuf,
    language: Option<Language>,
    package_root: Option<PathBuf>,
}

impl TryFrom<PathBuf> for FileInfo {
    type Error = io::Error;

    fn try_from(path_buf: PathBuf) -> Result<Self, Self::Error> {
        let path = path_buf.as_path();
        let language = Language::maybe_from(path);
        let package_root = find_package_root(path)?;
        Ok(Self {
            path_buf,
            language,
            package_root,
        })
    }
}
