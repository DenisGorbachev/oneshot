use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

/// Paths may be specified by the full path (as in `std::io::Result`)
/// Paths may be refer to the top-level use statement (e.g. `use std::io::Result; fn get_something() -> Result<u64, io::Error>`)
/// Use statements may refer to super-module items (e.g. `super::*`)
/// Some paths are "hidden": the use statements bring the trait in scope, which adds methods on the types that implement this trait. Such paths can only be extracted from use statements.
/// Some paths may refer to the use statements that are in a local scope (for example: `use MyEnum::*; let a = MyVariantA;`)
pub fn get_first_line_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    Ok(line)
}

#[cfg(test)]
mod tests {
    use std::fs::write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_get_first_line_unix() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Hello, World!\nSecond line").unwrap();

        let result = get_first_line_from_file(file.path()).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_get_first_line_windows() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Hello, World!\r\nSecond line").unwrap();

        let result = get_first_line_from_file(file.path()).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_get_first_line_no_newline() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Single line without newline").unwrap();

        let result = get_first_line_from_file(file.path()).unwrap();
        assert_eq!(result, "Single line without newline");
    }
}
