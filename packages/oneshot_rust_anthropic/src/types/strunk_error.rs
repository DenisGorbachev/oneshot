use clust::messages::MessagesError;
use derive_more::{Error, From};
use fmt_derive::Display;

use serialize::functions::serialize_to_file::SerializeToFileError;

use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;

#[derive(Error, Display, From, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrunkError {
    TheIoError(std::io::Error),
    TheMessagesError(MessagesError),
    TheSerializeToFileError(SerializeToFileError),
    TheConversationDirIfNotExistsError(ConversationDirIfNotExistsError),
}

impl StrunkError {}
