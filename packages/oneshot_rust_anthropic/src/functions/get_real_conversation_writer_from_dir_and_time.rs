use std::path::PathBuf;

use time::macros::format_description;
use time::OffsetDateTime;

use crate::types::real_conversation_writer::RealConversationWriter;

pub fn get_real_conversation_writer_from_dir_and_time(
    dir: PathBuf,
    time: OffsetDateTime,
) -> anyhow::Result<RealConversationWriter> {
    let format = format_description!("[year]-[month]-[day]-[hour]-[minute]-[second]-[subsecond]");
    let time_formatted = time.format(format)?;
    let path_buf = dir.join(format!("{time_formatted}.yaml"));
    let writer = RealConversationWriter::try_from(path_buf)?;
    Ok(writer)
}
