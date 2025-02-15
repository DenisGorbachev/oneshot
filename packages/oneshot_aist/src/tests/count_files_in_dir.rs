use std::io;
use std::path::Path;

/// Counts the files in a directory recursively
///
/// Returns `Ok(None)` if the count of files is larger than [`u64::MAX`]
pub fn count_files_in_dir(_dir: impl AsRef<Path>) -> io::Result<Option<u64>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use tempfile::{tempdir, tempdir_in, tempfile_in};

    #[test]
    fn count_in_empty_directory_must_be_zero() -> io::Result<()> {
        let dir = tempdir()?;
        let count = count_files_in_dir(&dir)?;
        assert_eq!(count, Some(0));
        Ok(())
    }

    #[test]
    fn count_in_non_empty_directory_must_be_one() -> io::Result<()> {
        let dir = tempdir()?;
        let _file = tempfile_in(&dir)?;
        let count = count_files_in_dir(&dir)?;
        assert_eq!(count, Some(1));
        Ok(())
    }

    #[test]
    fn must_count_recursively() -> io::Result<()> {
        let dir = tempdir()?;
        let dir_nested = tempdir_in(&dir)?;
        let _file_nested = tempfile_in(&dir_nested)?;
        let count = count_files_in_dir(&dir)?;
        assert_eq!(count, Some(1));
        Ok(())
    }
}
