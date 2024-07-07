use indoc::formatdoc;

use crate::types::language::Language;

pub fn push_intro_maybe(parts: &mut Vec<String>, language: Option<Language>) {
    if let Some(language) = language {
        push_intro(parts, language);
    }
}

pub fn push_intro(parts: &mut Vec<String>, language: Language) {
    parts.push(intro(language))
}

pub fn intro(language: Language) -> String {
    intro_raw(&language.to_string(), language.package())
}

pub fn intro_maybe(language_opt: Option<Language>) -> Option<String> {
    language_opt.map(intro)
}

pub fn intro_raw(language: &str, package: &str) -> String {
    formatdoc! {"
        Implement a {language} {package} according to the specification provided in the <readme> tags.
    "}
}
