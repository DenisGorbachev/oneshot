use crate::user_text;
use async_openai::types::ChatCompletionRequestMessage;
use subtype::subtype_string;

subtype_string!(
    pub struct InputString(String);
);

impl InputString {}

impl From<InputString> for ChatCompletionRequestMessage {
    fn from(value: InputString) -> Self {
        user_text(value.0)
    }
}
