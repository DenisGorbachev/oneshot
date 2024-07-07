use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use time::macros::format_description;
use time::OffsetDateTime;

use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;
use crate::types::real_conversation_writer::RealConversationWriter;

pub fn get_real_conversation_writer_from_dir_and_time(
    dir: impl AsRef<Path>,
    time: OffsetDateTime,
    hash: u64,
) -> anyhow::Result<RealConversationWriter> {
    let path_buf = conversation_dir(dir, time, hash)?.join("conversation.yaml");
    let writer = RealConversationWriter::try_from(path_buf)?;
    Ok(writer)
}

pub fn conversation_dir(
    parent: impl AsRef<Path>,
    time: OffsetDateTime,
    id: u64,
) -> Result<PathBuf, time::error::Format> {
    let time_formatted = format_time_for_dirname(time)?;
    let dir = parent.as_ref().join(format!("{time_formatted}-{id}"));
    Ok(dir)
}

pub fn conversation_dir_if_not_exists(
    parent: impl AsRef<Path>,
    time: OffsetDateTime,
    id: u64,
) -> Result<PathBuf, ConversationDirIfNotExistsError> {
    let dir = conversation_dir(parent, time, id)?;
    if dir.exists() {
        Err(io::Error::new(
            ErrorKind::AlreadyExists,
            "Conversation directory already exists",
        )
        .into())
    } else {
        Ok(dir)
    }
}

pub fn format_time_for_dirname(time: OffsetDateTime) -> Result<String, time::error::Format> {
    time.format(format_description!(
        "[year]-[month]-[day]-[hour]-[minute]-[second]"
    ))
}
