use clust::messages::MessagesError;
use derive_more::{Error, From};
use fmt_derive::Display;

use serialize::functions::serialize_to_file::SerializeToFileError;

#[derive(Error, Display, From, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrunkError {
    TheMessagesError(MessagesError),
    TheSerializeToFileError(SerializeToFileError),
}

impl StrunkError {}
