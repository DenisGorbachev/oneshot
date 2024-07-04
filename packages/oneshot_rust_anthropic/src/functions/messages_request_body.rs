use clust::messages::{ClaudeModel, Message, MessagesRequestBody, StreamOption, SystemPrompt};

pub fn messages_request_body_default() -> MessagesRequestBody {
    MessagesRequestBody {
        model: ClaudeModel::Claude35Sonnet20240620,
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    }
}

pub fn messages_request_body(
    system: impl Into<SystemPrompt>,
    messages: Vec<Message>,
) -> MessagesRequestBody {
    MessagesRequestBody {
        system: Some(system.into()),
        messages,
        ..messages_request_body_default()
    }
}
