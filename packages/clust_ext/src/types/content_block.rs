use clust::messages::ContentBlock;

pub fn get_text_from_content_block(block: ContentBlock) -> Option<String> {
    match block {
        ContentBlock::Text(text_content_block) => Some(text_content_block.text),
        ContentBlock::Image(_) => None,
        ContentBlock::ToolUse(_) => None,
        ContentBlock::ToolResult(_) => None,
    }
}
