use async_openai::types::ChatCompletionRequestMessage;

pub trait InputLike: Into<ChatCompletionRequestMessage> {}
