use std::path::Path;

use crate::types::language::Language;

pub fn push_implement_file_instruction(
    parts: &mut Vec<String>,
    language_opt: Option<Language>,
    path: &Path,
) {
    parts.push(implement_file_instruction(language_opt, path))
}

pub fn implement_file_instruction(language: Option<Language>, path: &Path) -> String {
    let language_maybe: String = language
        .as_ref()
        .map(Language::to_string)
        .unwrap_or_default();
    let todo = language.map(Language::todo).unwrap_or("TODO");
    format!("A {language_maybe} source file with the path `{path:?}` is provided below. Write the code according to the specification in the file. Implement items marked with `{todo}`.")
}
