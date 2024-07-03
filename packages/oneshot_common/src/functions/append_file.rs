use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub fn append_file(path: impl AsRef<Path>, target: &mut String) -> io::Result<()> {
    read_to_string(path).map(|contents| {
        target.push_str(&contents);
    })
}
