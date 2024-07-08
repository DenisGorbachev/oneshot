use std::io;
use std::path::{Path, PathBuf};

use fs_err::canonicalize;

use crate::functions::find_parent_containing_filename::find_parent_containing_filename;

pub fn find_dir_with_file_canonical(start: &Path, filename: &str) -> io::Result<Option<PathBuf>> {
    let start_canonical = canonicalize(start)?;
    let stop_canonical = find_parent_containing_filename(start_canonical.as_path(), filename).map(Path::to_path_buf);
    Ok(stop_canonical)
}
