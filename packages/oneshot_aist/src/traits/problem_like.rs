use async_openai::types::ChatCompletionRequestMessage;

pub trait ProblemLike: Into<ChatCompletionRequestMessage> {}
