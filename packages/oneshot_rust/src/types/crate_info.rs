use std::path::PathBuf;

use derive_getters::{Dissolve, Getters};
use derive_new::new;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CrateInfo {
    name: String,
    path_buf: PathBuf,
}

impl CrateInfo {}
