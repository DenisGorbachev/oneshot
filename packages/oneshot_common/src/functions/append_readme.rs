use std::io;
use std::path::Path;

use oneshot_utils::constants::README_FILE_NAME;

use crate::functions::append_file::append_file;

pub fn append_readme(cwd: impl AsRef<Path>, target: &mut String) -> io::Result<()> {
    append_file(cwd.as_ref().join(README_FILE_NAME), target)
}
