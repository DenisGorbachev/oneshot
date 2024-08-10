use clust::messages::{MessagesRequestBody, MessagesResponseBody};
use clust_ext::types::markdown_text::MarkdownText;
use save_load::format::Format;
use std::path::Path;

use crate::functions::write_message::write_message;
use crate::types::strunk_error::StrunkError;

pub fn output_request(dir: impl AsRef<Path>, format: Format, request_body: &MessagesRequestBody) -> Result<(), StrunkError> {
    format.save_to(dir.as_ref(), "request", request_body)?;
    write_message(dir.as_ref(), "request.md", MarkdownText::from(request_body).as_str())?;
    Ok(())
}

pub fn output_response(dir: impl AsRef<Path>, format: Format, response_body: &MessagesResponseBody) -> Result<(), StrunkError> {
    format.save_to(dir.as_ref(), "response", response_body)?;
    write_message(dir.as_ref(), "response.md", MarkdownText::from(response_body).as_str())?;
    Ok(())
}
