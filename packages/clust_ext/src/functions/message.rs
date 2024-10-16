use clust::messages::{Content, Message, Role};

pub fn user_message(content: impl Into<String>) -> Message {
    Message {
        role: Role::User,
        content: Content::SingleText(content.into()),
    }
}

pub fn assistant_message(content: impl Into<String>) -> Message {
    Message {
        role: Role::Assistant,
        content: Content::SingleText(content.into()),
    }
}
