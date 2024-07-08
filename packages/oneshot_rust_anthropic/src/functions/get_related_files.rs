use std::fs;
use std::path::Path;

use derive_more::Error;
use fmt_derive::Display;

use oneshot_common::types::source_file::SourceFile;

pub fn get_related_files(path: &Path, _content: &str) -> Result<Vec<SourceFile>, GetRelatedFilesError> {
    let mut related_files = Vec::new();

    // Get the parent directory of the current file
    let parent_dir = path
        .parent()
        .ok_or(GetRelatedFilesError::NoParentDirectory)?;

    // Read the directory contents
    let entries = fs::read_dir(parent_dir).map_err(GetRelatedFilesError::ReadDirError)?;

    // Iterate through directory entries
    for entry in entries {
        let entry = entry.map_err(GetRelatedFilesError::ReadEntryError)?;
        let file_path = entry.path();

        // Skip the current file
        if file_path == path {
            continue;
        }

        // Check if it's a file (not a directory)
        if file_path.is_file() {
            // Read the file content
            let file_content = fs::read_to_string(&file_path).map_err(GetRelatedFilesError::ReadFileError)?;

            // Create a SourceFile and add it to the list
            let source_file = SourceFile::new(file_path, file_content);
            related_files.push(source_file);
        }
    }

    Ok(related_files)
}

#[derive(Error, Display, Debug)]
pub enum GetRelatedFilesError {
    #[display("No parent directory found")]
    NoParentDirectory,

    #[display("Failed to read directory: {}", _0)]
    ReadDirError(std::io::Error),

    #[display("Failed to read directory entry: {}", _0)]
    ReadEntryError(std::io::Error),

    #[display("Failed to read file: {}", _0)]
    ReadFileError(std::io::Error),
}
