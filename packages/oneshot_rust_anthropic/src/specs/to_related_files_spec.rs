use std::hash::BuildHasherDefault;
use std::path::{Path, PathBuf};

use derive_more::From;
use fmt_derive::Display;
use fs_err::read_to_string;
use indexmap::IndexMap;
use rustc_hash::FxHasher;
use syn::{parse_file, UseTree};

use oneshot_common::types::source_file::SourceFile;

use crate::specs::into_file_paths::into_file_paths;
use crate::types::file_item::FileItem;

pub type FileContent = String;

pub type FileContentMap = IndexMap<PathBuf, FileContent, BuildHasherDefault<FxHasher>>;

pub fn get_source_files_map(package_root: &Path, path_buf: PathBuf) -> anyhow::Result<FileContentMap> {
    let mut map = FileContentMap::default();
    let mut path_bufs_to_process = vec![path_buf];
    while let Some(path_buf) = path_bufs_to_process.pop() {
        let path = path_buf.as_path();
        let content = read_to_string(path)?;
        let file = parse_file(&content)?;
        let path_bufs_raw = into_file_paths_from_syn_file(package_root, path, file)?;
        let path_bufs_filtered = path_bufs_raw.into_iter().filter(|pb| !map.contains_key(pb));
        path_bufs_to_process.extend(path_bufs_filtered);
        map.insert(path_buf, content);
    }
    Ok(map)
}

pub fn get_unique_source_files_from_path_buf(package_root: &Path, path_buf: PathBuf) -> Result<Vec<SourceFile>, anyhow::Error> {
    get_source_files_map(package_root, path_buf).map(|map| map.into_iter().map(SourceFile::from).collect())
}

// /// The LLM should see the final source_file after its related files
// pub fn get_all_source_files_from_path_buf(path_buf: PathBuf) -> Result<Vec<SourceFile>, anyhow::Error> {
//     let source_file = SourceFile::from_path_buf(path_buf)?;
//     let mut source_files = to_all_related_files_from_source_file(&source_file)?;
//     source_files.push(source_file);
//     Ok(source_files)
// }
//
// /// Optimize: in return type, replace `Vec` with `impl Iterator<Item=SourceFile>`
// pub fn to_all_related_files_from_source_file(source_file: &SourceFile) -> Result<Vec<SourceFile>, anyhow::Error> {
//     let file = parse_file(source_file.content())?;
//     let use_path_bufs = into_file_paths_from_syn_file(source_file.path_buf().as_path(), file)?;
//     let source_files_vec = use_path_bufs
//         .into_iter()
//         .map(get_all_source_files_from_path_buf)
//         .collect::<anyhow::Result<Vec<Vec<SourceFile>>>>()?;
//     let source_files = source_files_vec.into_iter().flatten().collect();
//     Ok(source_files)
// }

/// The dependencies may be specified within any item by using a fully qualified path (e.g. `std::path::PathBuf`), which won't show ut in the use statements
fn into_file_paths_from_syn_file(package_root: &Path, _path: &Path, file: syn::File) -> anyhow::Result<Vec<PathBuf>> {
    let item_paths = into_item_paths(file)?;
    into_file_paths(package_root, item_paths)
}

fn into_item_paths(file: syn::File) -> anyhow::Result<Vec<syn::Path>> {
    todo!()
}

/// A related file is any file that is mentioned in the use statement
pub fn into_related_items(path: &Path, file: syn::File) -> Result<Vec<FileItem>, anyhow::Error> {
    todo!()
}

pub fn into_use_trees(file: syn::File) -> Result<Vec<UseTree>, syn::Error> {
    todo!()
}

#[derive(derive_more::Error, Display, From, Clone, Debug)]
pub enum ToRelatedFilesFromContentError {
    TheSynError(syn::Error),
}

mod tests {
    use quote::quote;

    fn it_must_not_contain_well_known_items() {
        let contents = quote! {
            pub fn get_first_line(opt: Option<File>)
        };
        todo!()
    }

    fn it_must_not_contain_duplicates() {
        todo!()
    }
}

pub mod test_files;
