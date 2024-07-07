use clust::messages::{Content, MessagesResponseBody};
use itertools::Itertools;

use crate::types::content_block::get_text_from_content_block;

pub fn into_response_text(response: MessagesResponseBody) -> String {
    match response.content {
        Content::SingleText(text) => text,
        Content::MultipleBlocks(blocks) => blocks
            .into_iter()
            .filter_map(get_text_from_content_block)
            .join(""),
    }
}
