use std::fs::File;
use std::io::Write;
use std::path::Path;

use derive_more::{Display, Error, From};
use serde::Serialize;

use crate::format::Format;
use crate::functions::serialize;
use crate::functions::serialize::SerializeError;

pub fn serialize_to_file<T: Serialize>(value: &T, file_dir: &Path, file_stem: &str, format: Format) -> Result<(), SerializeToFileError> {
    let file_path = file_dir.join(format.get_file_name(file_stem));
    let mut file = File::create(file_path)?;
    let output = serialize::serialize(value, format)?;
    file.write_all(output.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[derive(Error, Display, From, Debug)]
pub enum SerializeToFileError {
    Io(std::io::Error),
    Serialize(SerializeError),
}
