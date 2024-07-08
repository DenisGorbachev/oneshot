use std::path::PathBuf;

use derive_getters::{Dissolve, Getters};
use derive_new::new;

#[derive(new, Getters, Dissolve, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileItem {
    path_buf: PathBuf,
    items: Vec<syn::Item>,
}

impl FileItem {}