use crate::types::language::Language;

pub fn role(language: Language) -> String {
    format!("You are an expert {language} developer. You adhere to {language} best practices and guidelines. You write clear, readable, maintainable code.")
}
