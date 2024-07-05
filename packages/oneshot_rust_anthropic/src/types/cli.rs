use std::path::PathBuf;

use clap::{value_parser, Parser};
use time::OffsetDateTime;

use crate::functions::client::client;
use crate::functions::get_real_conversation_writer_from_dir_and_time::get_real_conversation_writer_from_dir_and_time;
use crate::types::client_config::ClientConfig;
use crate::types::command::Command;
use crate::types::real_conversation_writer::RealConversationWriter;

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
    pub async fn execute(self, now: OffsetDateTime) -> anyhow::Result<()> {
        let client = client(self.anthropic_api_key);
        let _conversation_writer: Option<RealConversationWriter> = self
            .conversations_dir
            .map(|cd| get_real_conversation_writer_from_dir_and_time(cd, now))
            .transpose()?;
        match self.command {
            Command::Strunk(command) => command.execute(client).await,
        }
    }
}
