use crate::functions::role_with_language::role_with_language;
use crate::functions::role_without_language::role_without_language;
use crate::types::language::Language;

pub fn role_from_language_maybe(language_opt: Option<Language>) -> String {
    language_opt
        .map(role_with_language)
        .unwrap_or_else(role_without_language)
}
