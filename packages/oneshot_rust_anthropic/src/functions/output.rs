use std::path::Path;

use clust::messages::{MessagesRequestBody, MessagesResponseBody};
use save_load::format::Format;

use clust_ext::types::markdown_text::MarkdownText;

use crate::functions::write_message::write_message;
use crate::types::strunk_error::StrunkError;

pub fn output_request(dir: impl AsRef<Path>, format: Format, request_body: &MessagesRequestBody) -> Result<(), StrunkError> {
    format.save_to(request_body, dir.as_ref(), "request")?;
    write_message(dir.as_ref(), "request.md", MarkdownText::from(request_body).as_str())?;
    Ok(())
}

pub fn output_response(dir: impl AsRef<Path>, format: Format, response_body: &MessagesResponseBody) -> Result<(), StrunkError> {
    format.save_to(response_body, dir.as_ref(), "response")?;
    write_message(dir.as_ref(), "response.md", MarkdownText::from(response_body).as_str())?;
    Ok(())
}
