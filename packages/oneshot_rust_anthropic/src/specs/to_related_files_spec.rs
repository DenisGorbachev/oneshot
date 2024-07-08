use std::path::Path;

use derive_more::From;
use fmt_derive::Display;
use syn::{parse_file, UseTree};

use oneshot_common::types::source_file::SourceFile;

use crate::types::file_item::FileItem;

pub fn to_related_files_from_content(path: &Path, content: &str) -> Result<Vec<SourceFile>, anyhow::Error> {
    let file = parse_file(content)?;
    let related_items = into_related_items(path, file)?;
    todo!()
}

/// A related file is any file that is mentioned in the use statement
pub fn into_related_items(path: &Path, file: syn::File) -> Result<Vec<FileItem>, anyhow::Error> {
    todo!()
}

pub fn to_use_trees(file: syn::File) -> Result<Vec<UseTree>, syn::Error> {
    todo!()
}

#[derive(derive_more::Error, Display, From, Clone, Debug)]
pub enum ToRelatedFilesFromContentError {
    TheSynError(syn::Error),
}

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn must_not_contain_well_known_items() {
        let contents = quote! {
            pub fn get_first_line(opt: Option<File>)
        };
    }
}

pub mod test_files;
