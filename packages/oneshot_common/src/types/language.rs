use std::path::Path;

use strum::{Display, EnumIter};

use Language::*;
use oneshot_utils::traits::maybe_from::MaybeFrom;

/// Use `#[strum(to_string = "LanguageName")]` to specify a proper language name If the proper language name is different from enum variant name (for example: since `Cpp` variant proper name is `"C++"`, use `#[strum(to_string = "C++")]`)
#[derive(Display, EnumIter, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
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
    /// TODO: Implement a more robust language detection
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Rust),
            "py" => Some(Python),
            "ts" => Some(TypeScript),
            "go" => Some(Go),
            "java" => Some(Java),
            "rb" => Some(Ruby),
            "cpp" => Some(Cpp),
            _ => None,
        }
    }

    pub fn to_extension(self) -> String {
        match self {
            Rust => "rs",
            Python => "py",
            TypeScript => "ts",
            Go => "go",
            Java => "java",
            Ruby => "rb",
            Cpp => "cpp",
        }
        .to_owned()
    }

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

impl<'a> MaybeFrom<&'a Path> for Language {
    fn maybe_from(path: &'a Path) -> Option<Self> {
        let extension = path.extension()?;
        let extension_str = extension.to_str()?;
        Self::from_extension(extension_str)
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::types::language::Language;

    #[test]
    fn must_roundtrip_from_extension_with_to_extension() {
        for lang in Language::iter() {
            assert_eq!(Some(lang), Language::from_extension(&Language::to_extension(lang)))
        }
    }
}
