use indoc::formatdoc;

use crate::types::language::Language;

pub fn intro(language: Language) -> String {
    intro_raw()
}

pub fn intro_raw(language: &str, package: &str) -> String {
    formatdoc! {"
        Implement a {language} {package} according to the specification provided in the <readme> tags.
    "}
}
