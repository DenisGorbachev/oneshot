use std::path::Path;

use clust::messages::{MessagesRequestBody, MessagesResponseBody};

use clust_ext::types::markdown_text::MarkdownText;
use serialize::format::Format;
use serialize::functions::serialize_to_file::serialize_to_file;

use crate::functions::write_message;
use crate::types::strunk_error::StrunkError;

pub fn output_request(
    dir: impl AsRef<Path>,
    format: Format,
    request_body: &MessagesRequestBody,
) -> Result<(), StrunkError> {
    serialize_to_file(request_body, dir.as_ref(), "request", format)?;
    write_message::write_message(
        dir.as_ref(),
        "request.md",
        MarkdownText::from(request_body).as_str(),
    )?;
    Ok(())
}

pub fn output_response(
    dir: impl AsRef<Path>,
    format: Format,
    response_body: &MessagesResponseBody,
) -> Result<(), StrunkError> {
    serialize_to_file(response_body, dir.as_ref(), "response", format)?;
    write_message::write_message(
        dir.as_ref(),
        "response.md",
        MarkdownText::from(response_body).as_str(),
    )?;
    Ok(())
}
