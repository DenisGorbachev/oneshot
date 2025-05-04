use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Removes the first line from a file that, after trimming, matches the specified filter.
///
/// # Arguments
///
/// * `path` - The path to the file to modify
/// * `filter` - The string to match against each trimmed line
///
/// # Returns
///
/// * `io::Result<()>` - Success or error
pub fn remove_first_line_matching(path: &Path, filter: &str) -> io::Result<()> {
    // Read the file content
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    // Create a temporary file to write the filtered content
    let temp_path = path.with_extension("temp");
    let mut temp_file = File::create(&temp_path)?;

    // Track if we've found and removed a matching line yet
    let mut first_match_removed = false;

    // Process each line
    for line_result in lines {
        let line = line_result?;

        // If we haven't removed a line yet and this line matches after trimming, skip it
        if !first_match_removed && line.trim() == filter {
            first_match_removed = true;
            continue;
        }

        // Write the line and a newline character to the temp file
        writeln!(temp_file, "{line}")?;
    }

    // Close the temp file to ensure all data is written
    drop(temp_file);

    // Replace the original file with the temp file
    fs::rename(temp_path, path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    #[ignore]
    fn test_remove_ignore_attribute() {
        // Create a temporary file with test content
        let mut file = NamedTempFile::new().unwrap();
        let test_content = r#"
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn must_print() {
        assert_eq!(1, 1);
    }

    #[test]
    fn another_test() {
        assert_eq!(2, 2);
    }
}
"#;
        file.write_all(test_content.as_bytes()).unwrap();

        // Apply our function
        remove_first_line_matching(file.path(), "#[ignore]").unwrap();

        // Read the modified content
        let modified_content = fs::read_to_string(file.path()).unwrap();

        // Verify the #[ignore] attribute was removed
        let expected = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn must_print() {
        assert_eq!(1, 1);
    }

    #[test]
    fn another_test() {
        assert_eq!(2, 2);
    }
}
"#;
        assert_eq!(modified_content, expected);
    }
}
