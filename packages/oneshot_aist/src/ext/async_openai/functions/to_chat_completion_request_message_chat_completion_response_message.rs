use async_openai::types::{ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageAudio, ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestMessage, ChatCompletionResponseMessage, Role};

pub fn to_chat_completion_request_message_chat_completion_response_message(message: ChatCompletionResponseMessage) -> ChatCompletionRequestMessage {
    match message.role {
        Role::Assistant => {
            #[allow(deprecated)]
            let ChatCompletionResponseMessage {
                content,
                refusal,
                tool_calls,
                role: _role,
                function_call,
                audio,
            } = message;
            #[allow(deprecated)]
            ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
                content: content.map(ChatCompletionRequestAssistantMessageContent::Text),
                refusal,
                name: None,
                audio: audio.map(|t| ChatCompletionRequestAssistantMessageAudio {
                    id: t.id,
                }),
                tool_calls,
                function_call,
            })
        }
        _ => {
            let message_string = serde_json::ser::to_string(&message).expect("`message: ChatCompletionResponseMessage` should serialize successfully");
            panic!("Unexpected message.role for `message: ChatCompletionResponseMessage` = {message_string}")
        }
    }
}
