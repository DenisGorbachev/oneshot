use std::hash::BuildHasherDefault;
use std::path::{Path, PathBuf};

use derive_more::From;
use fmt_derive::Display;
use fs_err::read_to_string;
use indexmap::IndexMap;
use rustc_hash::FxHasher;
use syn::{parse_file, UseTree};

use oneshot_common::types::source_file::SourceFile;

use crate::types::file_item::FileItem;

pub type FileContent = String;

pub type FileContentMap = IndexMap<PathBuf, FileContent, BuildHasherDefault<FxHasher>>;

pub fn get_source_files_map(path_buf: PathBuf) -> anyhow::Result<FileContentMap> {
    let mut map = FileContentMap::default();
    let mut path_bufs_to_process = vec![path_buf];
    while let Some(path_buf) = path_bufs_to_process.pop() {
        let path = path_buf.as_path();
        let content = read_to_string(path)?;
        let file = parse_file(&content)?;
        let path_bufs_raw = into_file_paths_from_syn_file(path, file)?;
        let path_bufs_filtered = path_bufs_raw.into_iter().filter(|pb| !map.contains_key(pb));
        path_bufs_to_process.extend(path_bufs_filtered);
        map.insert(path_buf, content);
    }
    Ok(map)
}

/// The LLM should see the final source_file after its related files
pub fn get_source_files_from_path_buf(path_buf: PathBuf) -> Result<Vec<SourceFile>, anyhow::Error> {
    let source_file = SourceFile::from_path_buf(path_buf)?;
    let mut source_files = to_related_files_from_source_file(&source_file)?;
    source_files.push(source_file);
    Ok(source_files)
}

/// Optimize: in return type, replace `Vec` with `impl Iterator<Item=SourceFile>`
pub fn to_related_files_from_source_file(source_file: &SourceFile) -> Result<Vec<SourceFile>, anyhow::Error> {
    let file = parse_file(source_file.content())?;
    let use_path_bufs = into_file_paths_from_syn_file(source_file.path_buf().as_path(), file)?;
    let source_files_vec = use_path_bufs
        .into_iter()
        .map(get_source_files_from_path_buf)
        .collect::<anyhow::Result<Vec<Vec<SourceFile>>>>()?;
    let source_files = source_files_vec.into_iter().flatten().collect();
    Ok(source_files)
}

fn into_file_paths_from_syn_file(path: &Path, file: syn::File) -> anyhow::Result<Vec<PathBuf>> {
    into_file_paths(path, into_use_trees(file)?)
}

/// # Arguments
///
/// * `file_path` - the path of the file that contains the [`use_trees`](UseTree)
/// * `use_trees` - the use trees that were extracted from the file specified by the path
fn into_file_paths(file_path: &Path, use_trees: Vec<UseTree>) -> anyhow::Result<Vec<PathBuf>> {
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