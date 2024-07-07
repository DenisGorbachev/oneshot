use crate::constants::YOU_WRITE_CODE;

pub fn role_without_language() -> String {
    format!("You are an expert software developer. You adhere to best practices and guidelines. {you_write_code}", you_write_code = YOU_WRITE_CODE)
}
