use derive_more::From;

use crate::notes;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImplementOneshot {
    ImplementGenericAppOneshot,
    ImplementDenoOneshot,
}

impl ImplementOneshot {
    pub fn choose() -> ImplementOneshot {
        notes!(
            "The final goal is to build a user-facing app" {}
            "It's better to generate an app for a specific tech stack" {
                "#rationale" {
                    "We can achieve a lower error rate by applying stack-specific fixes"
                }
            }
        );
        todo!()
    }
}
