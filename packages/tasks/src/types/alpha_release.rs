use derive_getters::{Dissolve, Getters};
use derive_new::new;

use crate::types::alpha_tech_stack::AlphaTechStack;
use crate::types::unit::Unknown;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AlphaRelease {
    stack: AlphaTechStack,
    llm_payment_mechanism: Unknown,
    hosting_payment_mechanism: Unknown,
}

impl AlphaRelease {}
