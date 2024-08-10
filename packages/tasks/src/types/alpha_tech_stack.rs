use AlphaTechStack::*;

use crate::notes;
use crate::types::deno_alpha_release::DenoAlphaRelease;
use crate::types::unit::Any;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlphaTechStack {
    Generic(Any),
    Next(Any),
    Deno(DenoAlphaRelease),
    StaticReact(Any),
    StaticHtml(Any),
}

impl AlphaTechStack {
    pub fn executes_code_on_server(&self) -> bool {
        match self {
            Generic(_) => true,
            Next(_) => true,
            Deno(_) => true,
            StaticReact(_) => false,
            StaticHtml(_) => false,
        }
    }
}

impl Default for AlphaTechStack {
    fn default() -> Self {
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
