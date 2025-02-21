use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent};

pub fn user_text(text: impl Into<String>) -> ChatCompletionRequestMessage {
    let message = ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::Text(text.into()),
        name: None,
    };
    ChatCompletionRequestMessage::User(message)
}
