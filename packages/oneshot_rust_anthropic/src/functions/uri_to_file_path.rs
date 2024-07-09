use std::path::PathBuf;

use lsp_types::Uri;

pub fn to_file_path_from_uri(uri: &Uri) -> Option<PathBuf> {
    let scheme = uri.scheme()?;

    if !scheme.eq_lowercase("file") {
        return None;
    }

    let path = uri.path().as_str().trim_start_matches('/');

    #[cfg(windows)]
    let path = path.replace('/', "\\");

    Some(PathBuf::from(path))
}
