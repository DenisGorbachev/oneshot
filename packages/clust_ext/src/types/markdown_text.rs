use clust::messages::{Content, ContentBlock, Message, MessagesRequestBody, MessagesResponseBody, TextContentBlock};
use derive_more::{Deref, DerefMut, Display, From, Into};
use derive_new::new;
use itertools::Itertools;

#[derive(new, Deref, DerefMut, Into, Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct MarkdownText {
    inner: String,
}

impl From<TextContentBlock> for MarkdownText {
    fn from(value: TextContentBlock) -> Self {
        Self::new(value.text)
    }
}

impl<'a> From<&'a TextContentBlock> for MarkdownText {
    fn from(value: &'a TextContentBlock) -> Self {
        Self::new(value.text.to_owned())
    }
}

impl TryFrom<ContentBlock> for MarkdownText {
    type Error = ();

    fn try_from(value: ContentBlock) -> Result<Self, Self::Error> {
        match value {
            ContentBlock::Text(block) => Ok(From::from(block)),
            ContentBlock::Image(_) => Err(()),
            ContentBlock::ToolUse(_) => Err(()),
            ContentBlock::ToolResult(_) => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a ContentBlock> for MarkdownText {
    type Error = ();

    fn try_from(value: &'a ContentBlock) -> Result<Self, Self::Error> {
        match value {
            ContentBlock::Text(block_ref) => Ok(From::from(block_ref)),
            ContentBlock::Image(_) => Err(()),
            ContentBlock::ToolUse(_) => Err(()),
            ContentBlock::ToolResult(_) => Err(()),
        }
    }
}

impl From<Content> for MarkdownText {
    fn from(value: Content) -> Self {
        match value {
            Content::SingleText(text) => Self::new(text),
            Content::MultipleBlocks(blocks) => Self::new(
                blocks
                    .iter()
                    .map(Self::try_from)
                    .filter_map(Result::ok)
                    .map(String::from)
                    .join("\n"),
            ),
        }
    }
}

impl<'a> From<&'a Content> for MarkdownText {
    fn from(value: &'a Content) -> Self {
        match value {
            Content::SingleText(text) => Self::new(text.to_owned()),
            Content::MultipleBlocks(blocks) => Self::new(
                blocks
                    .iter()
                    .map(Self::try_from)
                    .filter_map(Result::ok)
                    .map(String::from)
                    .join("\n"),
            ),
        }
    }
}

impl From<Message> for MarkdownText {
    fn from(value: Message) -> Self {
        Self::from(value.content)
    }
}

impl<'a> From<&'a Message> for MarkdownText {
    fn from(value: &'a Message) -> Self {
        Self::from(&value.content)
    }
}

impl From<MessagesRequestBody> for MarkdownText {
    fn from(value: MessagesRequestBody) -> Self {
        Self::new(
            value
                .messages
                .iter()
                .map(Self::from)
                .join(MESSAGES_SEPARATOR),
        )
    }
}

impl<'a> From<&'a MessagesRequestBody> for MarkdownText {
    fn from(value: &'a MessagesRequestBody) -> Self {
        Self::new(
            value
                .messages
                .iter()
                .map(Self::from)
                .join(MESSAGES_SEPARATOR),
        )
    }
}

impl<'a> From<&'a MessagesResponseBody> for MarkdownText {
    fn from(value: &'a MessagesResponseBody) -> Self {
        Self::from(&value.content)
    }
}

pub const MESSAGES_SEPARATOR: &str = "\n------------\n\n";
