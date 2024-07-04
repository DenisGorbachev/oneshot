use clap::Parser;

use types::cli::Cli;

pub mod functions;
pub mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    Cli::parse().run().await
}
