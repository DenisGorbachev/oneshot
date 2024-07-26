use std::path::Path;

use clust::messages::{MessagesRequestBody, MessagesResponseBody};
use clust::Client;
use save_load::format::Format;

use crate::functions::output::{output_request, output_response};
use crate::types::strunk_error::StrunkError;

pub async fn create_a_message_with_output(dir: impl AsRef<Path>, format: Format, client: &Client, request_body: MessagesRequestBody) -> Result<MessagesResponseBody, StrunkError> {
    let dir = dir.as_ref();
    output_request(dir, format, &request_body)?;
    let response_body = client.create_a_message(request_body).await?;
    output_response(dir, format, &response_body)?;
    Ok(response_body)
}
