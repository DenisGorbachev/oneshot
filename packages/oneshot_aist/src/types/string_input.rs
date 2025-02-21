use crate::user_text;
use async_openai::types::ChatCompletionRequestMessage;
use subtype::subtype_string;

subtype_string!(
    pub struct StringInput(String);
);

impl StringInput {}

impl From<StringInput> for ChatCompletionRequestMessage {
    fn from(value: StringInput) -> Self {
        user_text(value.0)
    }
}
