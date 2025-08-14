use std::path::{Path, PathBuf};

use time::OffsetDateTime;
use time::error::Format;
use time::macros::format_description;

use oneshot_utils::functions::acquire_dir::acquire_dir;

use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;
use crate::types::real_conversation_writer::RealConversationWriter;

pub fn get_real_conversation_writer_from_dir_and_time(dir: impl AsRef<Path>, time: OffsetDateTime, hash: u64) -> anyhow::Result<RealConversationWriter> {
    let path_buf = conversation_dir(dir, time, hash)?.join("conversation.yaml");
    let writer = RealConversationWriter::try_from(path_buf)?;
    Ok(writer)
}

pub fn conversation_dir(parent: impl AsRef<Path>, time: OffsetDateTime, id: u64) -> Result<PathBuf, Format> {
    let dir_name = conversation_dir_name(time, id)?;
    let dir = parent.as_ref().join(dir_name);
    Ok(dir)
}

pub fn conversation_dir_name(time: OffsetDateTime, id: u64) -> Result<String, Format> {
    let time_formatted = format_time_for_dirname(time)?;
    let dir_name = format!("{time_formatted}-{id}");
    Ok(dir_name)
}

pub fn acquire_conversation_dir(parent: impl AsRef<Path>, time: OffsetDateTime, id: u64) -> Result<PathBuf, ConversationDirIfNotExistsError> {
    let dir = conversation_dir(parent, time, id)?;
    acquire_dir(dir.as_path())?;
    Ok(dir)
}

pub fn format_time_for_dirname(time: OffsetDateTime) -> Result<String, Format> {
    time.format(format_description!("[year]-[month]-[day]-[hour]-[minute]-[second]"))
}
