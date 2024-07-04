use std::io;

use oneshot_common::types::source_file::SourceFile;

use crate::types::package_path_buf::PackagePathBuf;

impl TryInto<SourceFile> for PackagePathBuf {
    type Error = io::Error;

    fn try_into(self) -> Result<SourceFile, Self::Error> {
        SourceFile::from_path_buf(self.inner)
    }
}
