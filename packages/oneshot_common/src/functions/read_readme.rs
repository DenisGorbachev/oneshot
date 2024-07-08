use std::io;
use std::path::{Path, PathBuf};

use derive_more::{Display, Error};
use derive_new::new;
use fs_err::read_to_string;

use oneshot_utils::constants::README_FILE_NAME;

pub fn read_readme(package_root: &Path) -> io::Result<String> {
    read_to_string(package_root.join(README_FILE_NAME).as_path())
    // .map_err(|source| ReadToStringError::new(package_root, source))
}

#[derive(new, Error, Display, Debug)]
#[display("Could not read {path_buf:?} README.md")]
pub struct ReadToStringError {
    #[new(into)]
    path_buf: PathBuf,
    source: io::Error,
}
