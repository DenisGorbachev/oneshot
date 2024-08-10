use derive_getters::{Dissolve, Getters};
use derive_new::new;

use crate::types::unit::Any;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DenoAlphaRelease {
    import_fixer: Any,
}

impl DenoAlphaRelease {}
