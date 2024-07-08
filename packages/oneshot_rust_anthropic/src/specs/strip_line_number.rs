use std::path::Path;

pub fn strip_line_number(path: &Path) -> &Path {
    todo!()
}

mod tests {
    use super::*;

    fn it_must_strip_line_number() {
        let path_with_number = Path::new("/some/path/to/file.rs:38");
        let path_without_number = Path::new("/some/path/to/file.rs");
        assert_eq!(strip_line_number(path_with_number), path_without_number);
    }
}
