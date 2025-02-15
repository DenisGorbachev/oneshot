use crate::{user_text, Outcome, ValidateV1};
use async_openai::config::Config;
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionMessageToolCall, ChatCompletionResponseMessage, ChatCompletionResponseMessageAudio, CreateChatCompletionRequestArgs, FinishReason};
use async_openai::Client;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use syn::File;
use syn_more::SynFrom;
use FinishReason::*;

pub fn execute_v1(path: &Path, _command: Command) -> Outcome {
    let _file = File::syn_from(path)?;
    todo!()
}

/// Parallel execution is implemented separately
/// Use `args.n()` to receive multiple completions in a single response
///
/// TODO: Implement forking for each ChatChoice
pub async fn execute_v2<C: Config, E: Error, Validator: ValidateV1<String, Error = E>, Test: FnMut(&str) -> Vec<E>>(input: String, validator: &Validator, client: &Client<C>, args: &CreateChatCompletionRequestArgs, mut gas: u32) -> Result<String, ExecuteV2Error<Vec<E>>> {
    use ExecuteV2Error::*;
    let errors = validator.validate_v1(&input);
    if errors.is_empty() {
        return Ok(input);
    }
    if gas == 0 {
        return Err(OutOfGas);
    }
    gas -= 1;
    let mut messages = vec![user_text(input)];
    messages.extend(errors.into_iter().map(|e| user_text(e.to_string())));
    let request = args.clone().messages(messages).build().map_err(Client)?;
    let response = client.chat().create(request).await.map_err(Client)?;
    for choice in response.choices {
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
                if let Some(_content) = content {
                    let _gas = gas;
                    todo!()
                }
            }
            Some(Length) => return Err(LengthFinishReason),
            Some(ToolCalls) => return Err(ToolCallsFinishReason),
            Some(ContentFilter) => return Err(ContentFilterFinishReason),
            Some(FunctionCall) => unreachable!("The `finish_reason = FunctionCall` is deprecated in the latest version of the API"),
        }
    }
    todo!()
}

pub enum ExecuteV2Error<E> {
    Custom(E),
    Client(OpenAIError),
    /// The model returned a message with a `refusal` field
    Refusal(String),
    UnsupportedToolCalls(Vec<ChatCompletionMessageToolCall>),
    UnsupportedAudio(ChatCompletionResponseMessageAudio),
    LengthFinishReason,
    /// Tool calls are not supported yet; please open a PR if you need them.
    ToolCallsFinishReason,
    ContentFilterFinishReason,
    /// The function has reached the maximum count of steps
    OutOfGas,
}
