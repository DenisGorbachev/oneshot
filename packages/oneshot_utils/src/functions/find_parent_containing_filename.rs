use std::path::Path;

pub fn find_parent_containing_filename(start: &Path, filename: impl AsRef<Path>) -> Option<&Path> {
    let filename = filename.as_ref();
    start.ancestors().find(|path| path.join(filename).exists())
}
