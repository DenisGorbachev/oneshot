use std::path::{Path, PathBuf};

use time::OffsetDateTime;

use oneshot_utils::types::counter::Counter;

use crate::functions::get_real_conversation_writer_from_dir_and_time::acquire_conversation_dir;
use crate::types::conversation_dir_if_not_exists_error::ConversationDirIfNotExistsError;

pub fn acquire_conversation_dir_from_options(parent: Option<impl AsRef<Path>>, now: OffsetDateTime, request_counter: &mut Counter<u64>) -> Result<Option<PathBuf>, ConversationDirIfNotExistsError> {
    parent
        .map(|dir| acquire_conversation_dir(dir, now, request_counter.take()))
        .transpose()
}
