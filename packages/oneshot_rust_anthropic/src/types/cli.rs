use clap::Parser;

use crate::functions::client::client;
use crate::types::client_config::ClientConfig;
use crate::types::command::Command;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, short, env = "ANTHROPIC_API_KEY")]
    pub anthropic_api_key: String,

    #[clap(flatten)]
    pub anthropic_client_config: ClientConfig,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        let client = client(self.anthropic_api_key);
        match self.command {
            Command::Strunk(command) => command.run(client).await,
        }
    }
}
