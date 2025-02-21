use crate::ConfigLike;
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse};
use async_openai::Client;

pub async fn send_with_messages<Config>(messages: Vec<ChatCompletionRequestMessage>, client: &Client<Config>, args: &CreateChatCompletionRequestArgs) -> Result<CreateChatCompletionResponse, OpenAIError>
where
    Config: ConfigLike,
{
    let request = args.clone().messages(messages).build()?;
    client.chat().create(request).await
}
