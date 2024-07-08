use std::io;
use std::path::Path;

use fs_err::write;

pub fn write_message(dir: impl AsRef<Path>, filename: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let path_buf = dir.as_ref().join(filename);
    println!("Writing {path:?}", path = path_buf.as_path());
    write(path_buf, contents)
}
