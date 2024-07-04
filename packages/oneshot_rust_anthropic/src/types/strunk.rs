use std::fs::read_to_string;
use std::path::PathBuf;

use clap::{value_parser, Parser};
use clust::Client;
use syn::__private::ToTokens;
use tokio_stream::StreamExt;

use clust_ext::functions::message::{assistant_message, user_message};
use oneshot_common::functions::intro::intro;
use oneshot_common::functions::read_readme::read_readme;
use oneshot_common::functions::role::role;
use oneshot_common::types::language::Language;
use oneshot_utils::functions::find_package_root::get_package_root;

use crate::functions::messages_request_body::messages_request_body;

#[derive(Parser, Debug)]
pub struct Strunk {
    #[arg(name = "path", value_parser = value_parser!(PathBuf))]
    path_buf: PathBuf,
}

impl Strunk {
    pub async fn run(self, client: Client) -> anyhow::Result<()> {
        let Self { path_buf } = self;
        let path = path_buf.as_path();
        let _cwd = env!("CARGO_MANIFEST_DIR");
        let package_root = get_package_root(path)?;
        let language = Language::Rust;
        let role = role(language);
        let file_content = read_to_string(path)?;
        let mut file = syn::parse_file(&file_content)?;
        file.items = vec![];
        // let source_file_xml = SourceFile::new(path, file)?.serialize_to_xml()?;
        let user_parts: Vec<String> = vec![
            intro(language),
            read_readme(package_root)?,
            format!("A {language} source file with the path `{path:?}` is provided below. Write the code according to the specification in the file."),
        ];
        let user_content = user_parts.join("\n\n");
        let assistant_content = file.to_token_stream().to_string();

        let body = messages_request_body(
            role,
            vec![
                user_message(user_content),
                assistant_message(assistant_content),
            ],
        );

        let mut stream = client.create_a_message_stream(body).await?;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            println!("{chunk}");
        }

        Ok(())
    }
}
