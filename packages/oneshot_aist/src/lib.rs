//! # Idea
//!
//! A CLI app that tries to "fix" a directory.
//!
//! There is a [`subtype::Validate`] trait. This function returns a `Vec<impl Error>`. If the vector is empty, then the search is complete. If the vector is not empty, then it sends a request to the LLM. The LLM returns a response that is interpreted as a list of commands applied to files in context.
//!
//! It is possible

mod traits;
mod types;

pub use traits::*;

pub use types::*;

mod functions;

pub use functions::*;
mod utils;
pub use utils::*;
