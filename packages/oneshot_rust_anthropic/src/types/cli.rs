use std::path::PathBuf;

use clap::{value_parser, Parser};

use crate::functions::client::client;
use crate::types::client_config::ClientConfig;
use crate::types::command::Command;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, short, env = "ANTHROPIC_API_KEY")]
    pub anthropic_api_key: String,

    #[clap(flatten)]
    pub anthropic_client_config: ClientConfig,

    #[arg(long, short, value_parser = value_parser!(PathBuf), help = "A directory for saving conversations with the LLM. Allows you to inspect the requests & responses.")]
    pub conversations_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        let client = client(self.anthropic_api_key);
        match self.command {
            Command::Strunk(command) => command.execute(client).await,
        }
    }
}
