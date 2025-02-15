use crate::{try_into_content, TryIntoContentError};
use async_openai::config::Config;
use async_openai::error::OpenAIError;
use async_openai::types::CreateChatCompletionRequest;
use async_openai::Client;

/// Set `request.n` to a value > 1 to receive multiple completions in a single response
pub async fn try_into_content_iter<C: Config>(client: &Client<C>, request: CreateChatCompletionRequest) -> Result<impl Iterator<Item = Result<String, TryIntoContentError>>, OpenAIError> {
    let response = client.chat().create(request).await?;
    let iter = response.choices.into_iter().map(try_into_content);
    Ok(iter)
}
