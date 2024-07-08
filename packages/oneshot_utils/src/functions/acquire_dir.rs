use std::io;
use std::io::ErrorKind;
use std::path::Path;

use fs_err::create_dir_all;

pub fn acquire_dir<T: AsRef<Path>>(dir: T) -> io::Result<T> {
    if dir.as_ref().exists() {
        Err(io::Error::new(ErrorKind::AlreadyExists, "Conversation directory already exists"))
    } else {
        create_dir_all(dir.as_ref())?;
        Ok(dir)
    }
}
