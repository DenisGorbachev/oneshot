use std::path::{Path, PathBuf};

pub fn strip_line_number_from_path(path: &Path) -> &Path {
    let path_str = path.to_str().unwrap_or("");
    if let Some((path_without_line, line_number)) = path_str.rsplit_once(':')
        && line_number.chars().all(char::is_numeric)
    {
        return Path::new(path_without_line);
    }
    path
}

pub fn strip_line_number_from_path_buf_owned(mut path_buf: PathBuf) -> PathBuf {
    strip_line_number_from_path_buf(&mut path_buf);
    path_buf
}

pub fn strip_line_number_from_path_buf(path_buf: &mut PathBuf) {
    if let Some(new_file_name) = path_buf
        .file_name()
        .and_then(|s| s.to_str())
        .and_then(|file_name| {
            let (file_name_without_line, line_number) = file_name.rsplit_once(':')?;
            line_number
                .chars()
                .all(|character| character.is_ascii_digit())
                .then(|| file_name_without_line.to_string())
        })
    {
        path_buf.set_file_name(new_file_name)
    }
}

pub fn strip_line_numbers_from_path_bufs(path_bufs: &mut [PathBuf]) {
    path_bufs
        .iter_mut()
        .for_each(strip_line_number_from_path_buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_strip_line_number() {
        let mut path_with_number = PathBuf::from("/some/path/to/file.rs:38");
        let expected = PathBuf::from("/some/path/to/file.rs");
        strip_line_number_from_path_buf(&mut path_with_number);
        assert_eq!(path_with_number, expected);
    }

    #[test]
    fn it_should_not_strip_when_no_number() {
        let mut path = PathBuf::from("/some/path/to/file.rs");
        let expected = path.clone();
        strip_line_number_from_path_buf(&mut path);
        assert_eq!(path, expected);
    }

    #[test]
    fn it_should_not_strip_number_in_middle() {
        let mut path = PathBuf::from("/some/path:123/to/file.rs");
        let expected = path.clone();
        strip_line_number_from_path_buf(&mut path);
        assert_eq!(path, expected);
    }

    #[test]
    fn it_should_handle_multiple_colons() {
        let mut path = PathBuf::from("/some:path:to/file.rs:456");
        let expected = PathBuf::from("/some:path:to/file.rs");
        strip_line_number_from_path_buf(&mut path);
        assert_eq!(path, expected);
    }

    #[test]
    fn it_should_not_strip_non_numeric_suffix() {
        let mut path = PathBuf::from("/path/to/file.rs:abc");
        let expected = path.clone();
        strip_line_number_from_path_buf(&mut path);
        assert_eq!(path, expected);
    }

    #[test]
    fn it_should_handle_empty_path() {
        let mut path = PathBuf::from("");
        let expected = path.clone();
        strip_line_number_from_path_buf(&mut path);
        assert_eq!(path, expected);
    }

    #[test]
    fn it_should_handle_just_number() {
        let mut path = PathBuf::from(":123");
        let expected = PathBuf::from("");
        strip_line_number_from_path_buf(&mut path);
        assert_eq!(path, expected);
    }
}
