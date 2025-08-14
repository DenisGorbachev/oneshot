use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

use anyhow::anyhow;
use fs_err::read_to_string;
use indexmap::IndexMap;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, TextDocumentIdentifier, TextDocumentPositionParams, Uri};
use syn::{File, parse_file, visit::Visit};

use crate::functions::uri_to_file_path::to_file_path_from_uri;

#[derive(Default, Clone, Debug)]
struct IdentVisitor {
    idents: Vec<syn::Ident>,
}

impl<'ast> Visit<'ast> for IdentVisitor {
    fn visit_ident(&mut self, ident: &'ast syn::Ident) {
        self.idents.push(ident.clone());
    }
}

fn extract_idents(file: &File) -> Vec<syn::Ident> {
    let mut visitor = IdentVisitor::default();
    visitor.visit_file(file);
    visitor.idents
}

async fn get_definition_locations(idents: Vec<syn::Ident>, uri: &Uri, lsp_client: &LspClient) -> Vec<PathBuf> {
    let mut locations = Vec::new();

    for ident in idents {
        let start_location = ident.span().start();
        let params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier::new(uri.clone()),
                position: lsp_types::Position::new(start_location.line as u32, start_location.column as u32),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        if let Ok(GotoDefinitionResponse::Scalar(location)) = lsp_client.goto_definition(params).await {
            locations.push(to_file_path_from_uri(&location.uri).expect("Failed to convert URI to file path"));
        }
    }
    locations
}

fn deduplicate_files(files: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique_files = IndexMap::new();
    for file in files {
        unique_files.entry(file).or_insert(());
    }
    unique_files.into_keys().collect()
}

fn read_file_contents(path_bufs: Vec<PathBuf>) -> Result<IndexMap<PathBuf, String>, io::Error> {
    let mut file_contents = IndexMap::new();
    for path_buf in path_bufs {
        let contents = read_to_string(&path_buf)?;
        file_contents.insert(path_buf, contents);
    }
    Ok(file_contents)
}

async fn analyze_file(file_path: &Path, lsp_client: &LspClient) -> anyhow::Result<IndexMap<PathBuf, String>> {
    let file_path_str = file_path
        .as_os_str()
        .to_str()
        .ok_or(anyhow!("Could not convert file path to str"))?;
    let file_uri = Uri::from_str(file_path_str)?;
    let content = fs::read_to_string(file_path)?;
    let file = parse_file(&content).expect("Failed to parse file");
    let idents = extract_idents(&file);
    let definition_locations = get_definition_locations(idents, &file_uri, lsp_client).await;
    let unique_files = deduplicate_files(definition_locations);
    let contents = read_file_contents(unique_files)?;
    Ok(contents)
}

pub struct LspClient;

impl LspClient {
    pub async fn goto_definition(&self, params: GotoDefinitionParams) -> io::Result<GotoDefinitionResponse> {
        todo!()
    }
}
