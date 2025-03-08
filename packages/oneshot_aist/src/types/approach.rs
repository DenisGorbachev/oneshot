use crate::Strategy;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

/// The name of the approach is equal to the name of the `figment` profile
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Approach {
    branch: String,
    config: Strategy,
}

impl Approach {}
