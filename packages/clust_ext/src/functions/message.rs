use clust::messages::{Message, Role};

pub fn user_message(content: String) -> Message {
    Message {
        role: Role::User,
        content: content.into(),
    }
}

pub fn assistant_message(content: String) -> Message {
    Message {
        role: Role::Assistant,
        content: content.into(),
    }
}
