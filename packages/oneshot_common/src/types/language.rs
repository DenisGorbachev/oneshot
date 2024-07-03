use strum::Display;

use Language::*;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    Go,
    Java,
    Ruby,
    // PRs with additional languages are welcome
}

impl Language {
    pub fn package(self) -> &'static str {
        match self {
            Ruby => "gem",
            _ => "package",
        }
    }

    pub fn todo(self) -> &'static str {
        match self {
            Rust => "todo!()",
            _ => "TODO",
        }
    }
}
