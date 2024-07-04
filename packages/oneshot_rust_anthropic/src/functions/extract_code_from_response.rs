use derive_more::{Error, From};
use fmt_derive::Display;

/// AI: The response is sent by the LLM. The response may or may not contain code snippets. Extract and return all code snippets.
/// AI: If there are no code snippets, return an error
pub fn extract_code_from_response(_response: &str) -> Result<Vec<String>, ExtractCodeFromResponse> {
    todo!()
}

#[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ExtractCodeFromResponse {}
