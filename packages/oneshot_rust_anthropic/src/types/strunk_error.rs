use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;
use clust::messages::MessagesError;
use derive_more::{Error, From};
use fmt_derive::Display;
use save_load::errors::save_one_error::SaveOneError;

#[derive(Error, Display, From, Debug)]
pub enum StrunkError {
    TheIoError(std::io::Error),
    TheSynError(syn::Error),
    TheMessagesError(MessagesError),
    TheSerializeToFileError(SaveOneError),
    TheConversationDirIfNotExistsError(ConversationDirIfNotExistsError),
}

impl StrunkError {}
