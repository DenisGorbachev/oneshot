use clust::messages::MessagesError;
use derive_more::{Error, From};
use fmt_derive::Display;
use save_load::errors::save_error::SaveError;

use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;

#[derive(Error, Display, From, Debug)]
pub enum StrunkError {
    TheIoError(std::io::Error),
    TheSynError(syn::Error),
    TheMessagesError(MessagesError),
    TheSerializeToFileError(SaveError),
    TheConversationDirIfNotExistsError(ConversationDirIfNotExistsError),
}

impl StrunkError {}
