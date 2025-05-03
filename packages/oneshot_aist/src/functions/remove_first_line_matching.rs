use std::io;
use std::path::Path;

// TODO: use the Lines iterator to filter out the first line that, after .trim(), is equal to `_filter`
pub fn remove_first_line_matching(_path: &Path, _filter: &str) -> io::Result<()> {
    todo!()
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
