use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConversationDirIfNotExistsError {
    TheIoError(std::io::Error),
    TheTimeFormatError(time::error::Format),
}

impl ConversationDirIfNotExistsError {}
