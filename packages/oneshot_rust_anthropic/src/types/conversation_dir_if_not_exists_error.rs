use std::io::Error as IoError;

use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Debug)]
pub enum ConversationDirIfNotExistsError {
    TheIoError(IoError),
    TheTimeFormatError(time::error::Format),
}

impl ConversationDirIfNotExistsError {}
