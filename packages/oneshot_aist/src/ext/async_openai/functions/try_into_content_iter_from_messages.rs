use crate::{try_into_content_iter, ConfigLike, TryIntoContentError};
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs};
use async_openai::Client;

pub async fn try_into_content_iter_from_messages<Config>(messages: Vec<ChatCompletionRequestMessage>, client: &Client<Config>, args: &CreateChatCompletionRequestArgs) -> Result<impl Iterator<Item = Result<String, TryIntoContentError>>, OpenAIError>
where
    Config: ConfigLike,
{
    let request = args.clone().messages(messages).build()?;
    try_into_content_iter(client, request).await
}
