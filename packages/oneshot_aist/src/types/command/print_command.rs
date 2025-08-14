use crate::{Outcome, execute_v3, user_text};
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::CreateChatCompletionRequest;
use clap::{Parser, value_parser};
use std::sync::Arc;
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct PrintCommand {
    #[arg(short = 'u', long, value_parser = value_parser!(Url))]
    base_url: Url,

    #[arg(short = 'k', long, env)]
    api_key: String,

    #[arg(short = 'm', long)]
    model: String,
}

impl PrintCommand {
    pub async fn run(self) -> Outcome {
        let Self {
            base_url,
            api_key,
            model,
        } = self;

        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);

        let client = Arc::new(Client::with_config(config));

        let request = CreateChatCompletionRequest {
            model,
            messages: vec![user_text("Write a short poem about heavy cream")],
            n: Some(3),
            ..Default::default()
        };

        let output = execute_v3(
            &mut |choice| {
                use ControlFlow::*;
                use std::ops::ControlFlow;
                match &choice.message.content {
                    None => Continue(vec![user_text("Please respond with text content")]),
                    Some(content) => {
                        if content.contains("mathematics") {
                            Break(())
                        } else {
                            Continue(vec![user_text(
                                "Your response must contain the word \"mathematics\"",
                            )])
                        }
                    }
                }
            },
            3,
            client,
            request,
        )
        .await;

        println!("{:#?}", &output);
        Ok(())
    }
}
