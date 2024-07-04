use clust::messages::{Content, ContentBlock, MessagesResponseBody};
use itertools::Itertools;

pub fn into_text(response: MessagesResponseBody) -> String {
    match response.content {
        Content::SingleText(text) => text,
        Content::MultipleBlocks(blocks) => blocks
            .into_iter()
            .filter_map(|block| match block {
                ContentBlock::Text(text_content_block) => Some(text_content_block.text),
                ContentBlock::Image(_) => None,
                ContentBlock::ToolUse(_) => None,
                ContentBlock::ToolResult(_) => None,
            })
            .join(""),
    }
}
