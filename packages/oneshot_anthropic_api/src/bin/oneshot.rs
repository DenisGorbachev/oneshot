use std::path::PathBuf;

use anthropic_sdk::Client;
use clap::{value_parser, Parser};
use serde_json::json;

use oneshot_common::functions::implement_every_function::implement_every_function;
use oneshot_common::functions::intro::intro;
use oneshot_common::functions::read_readme::read_readme;
use oneshot_common::functions::role::role;
use oneshot_common::types::language::Language;
use oneshot_common::types::source_file::SourceFile;
use oneshot_utils::functions::find_package_root::get_package_root;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, short, env = "ANTHROPIC_API_KEY")]
    pub anthropic_api_key: String,

    #[clap(flatten)]
    pub anthropic_client_config: AnthropicClientConfig,

    #[arg(value_parser = value_parser!(PathBuf))]
    pub path: PathBuf,
}

#[derive(Parser, Debug)]
pub struct AnthropicClientConfig {
    #[arg(long, short)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _cwd = env!("CARGO_MANIFEST_DIR");
    let package_root = get_package_root(&args.path)?;
    let language = Language::Rust;
    let role = role(language);
    let source_file_xml = SourceFile::new(&args.path)?.serialize_to_xml()?;
    let parts: Vec<String> = vec![
        intro(language),
        read_readme(package_root)?,
        source_file_xml,
        implement_every_function(language),
    ];
    let content = parts.join("\n\n");

    let request = Client::new()
        .version("2023-06-01")
        .auth(args.anthropic_api_key.as_str())
        .model("claude-3-5-sonnet-20240620")
        .system(&role)
        .messages(&json!([
            {"role": "user", "content": content}
        ]))
        // .verbose(true) // Uncomment to return the response as it is from Anthropic
        .build()?;

    request
        .execute(|text| async move { process_response(text) })
        .await?;

    Ok(())
}

pub fn process_response(text: String) {
    println!("{text}");
    todo!()
}

#[cfg(test)]
mod tests {}
