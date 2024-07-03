use crate::types::language::Language;

pub fn implement_every_function(language: Language) -> String {
    implement_every_function_raw(language.todo())
}

pub fn implement_every_function_raw(todo: &str) -> String {
    format!("Implement every function marked with {todo}.")
}
