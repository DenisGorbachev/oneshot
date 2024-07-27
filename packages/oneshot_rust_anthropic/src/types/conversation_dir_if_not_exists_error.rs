use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Debug)]
pub enum ConversationDirIfNotExistsError {
    TheIoError(std::io::Error),
    TheTimeFormatError(time::error::Format),
}

impl ConversationDirIfNotExistsError {}
