use crate::constants::YOU_WRITE_CODE;
use crate::types::language::Language;

pub fn role_with_language(language: Language) -> String {
    format!("You are an expert {language} developer. You adhere to {language} best practices and guidelines. {you_write_code}", you_write_code = YOU_WRITE_CODE)
}
