use clust::messages::MessagesError;
use derive_more::{Error, From};
use fmt_derive::Display;

use serialize::functions::save::SaveError;

use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;

#[derive(Error, Display, From, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrunkError {
    TheIoError(std::io::Error),
    TheSynError(syn::Error),
    TheMessagesError(MessagesError),
    TheSerializeToFileError(SaveError),
    TheConversationDirIfNotExistsError(ConversationDirIfNotExistsError),
}

impl StrunkError {}
