use async_openai::types::FinishReason::{ContentFilter, FunctionCall, Length, Stop, ToolCalls};
use async_openai::types::{ChatChoice, ChatCompletionMessageToolCall, ChatCompletionResponseMessage, ChatCompletionResponseMessageAudio};

pub fn try_into_content(choice: ChatChoice) -> Result<String, TryIntoContentError> {
    use TryIntoContentError::*;
    match choice.finish_reason {
        None => unreachable!("The /v1/chat/completions endpoint always returns choices with `finish_reason = Some(...)` if called with `stream = false`"),
        Some(Stop) => {
            #[allow(deprecated)]
            let ChatCompletionResponseMessage {
                content,
                refusal,
                tool_calls,
                role: _,
                function_call: _,
                audio,
            } = choice.message;
            if let Some(refusal) = refusal {
                return Err(Refusal(refusal));
            }
            if let Some(tool_calls) = tool_calls {
                return Err(UnsupportedToolCalls(tool_calls));
            }
            if let Some(audio) = audio {
                return Err(UnsupportedAudio(audio));
            }
            if let Some(content) = content {
                return Ok(content);
            }
            Err(ContentIsEmpty)
        }
        Some(Length) => Err(LengthFinishReason),
        Some(ToolCalls) => Err(ToolCallsFinishReason),
        Some(ContentFilter) => Err(ContentFilterFinishReason),
        Some(FunctionCall) => unreachable!("The `finish_reason = FunctionCall` is deprecated in the latest version of the API"),
    }
}

pub enum TryIntoContentError {
    /// The model returned a message with a `refusal` field
    ContentIsEmpty,
    Refusal(String),
    UnsupportedToolCalls(Vec<ChatCompletionMessageToolCall>),
    UnsupportedAudio(ChatCompletionResponseMessageAudio),
    LengthFinishReason,
    ToolCallsFinishReason,
    ContentFilterFinishReason,
}
