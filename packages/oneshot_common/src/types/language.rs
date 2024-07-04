use strum::Display;

use Language::*;

/// Use `#[strum(to_string = "LanguageName")]` to specify a proper language name If the proper language name is different from enum variant name (for example: since `Cpp` variant proper name is `"C++"`, use `#[strum(to_string = "C++")]`)
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
    #[strum(to_string = "C++")]
    Cpp,
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
