use clap::Parser;
use time::OffsetDateTime;

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
    pub async fn execute(self, now: OffsetDateTime) -> anyhow::Result<()> {
        let client = client(self.anthropic_api_key);
        // let _conversation_writer: Option<RealConversationWriter> = self
        //     .conversations_dir
        //     .map(|cd| get_real_conversation_writer_from_dir_and_time(cd, now))
        //     .transpose()?;
        match self.command {
            Command::Strunk(strunk) => strunk.execute(client, now).await,
            Command::Run(run) => run.execute(client, now).await,
        }
    }
}
