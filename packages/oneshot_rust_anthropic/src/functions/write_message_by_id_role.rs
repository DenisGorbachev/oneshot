use std::io;
use std::path::Path;

use crate::functions::write_message::write_message;

#[allow(dead_code)]
pub fn write_message_by_id_role(dir: impl AsRef<Path>, id: u64, role: &str, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let filename = format!("{id}-{role}.md");
    write_message(dir, filename, contents)
}
