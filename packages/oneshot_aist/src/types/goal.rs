use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::path::PathBuf;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Goal {
    path_buf: PathBuf,
    prompt: String,
}

impl Goal {}
