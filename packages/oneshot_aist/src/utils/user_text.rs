use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent};

pub fn user_text(text: String) -> ChatCompletionRequestMessage {
    let message = ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::Text(text),
        name: None,
    };
    ChatCompletionRequestMessage::User(message)
}
