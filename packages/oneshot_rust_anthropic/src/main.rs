use clap::Parser;
use time::OffsetDateTime;

use types::cli::Cli;

pub mod functions;
pub mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_env_logger();
    init_tracing_subscriber();
    let now = OffsetDateTime::now_utc();
    Cli::parse().execute(now).await
}

fn init_env_logger() {
    // env_logger::init();
}

fn init_tracing_subscriber() {
    use tracing_subscriber::util::SubscriberInitExt;
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        // .with_max_level(tracing::Level::TRACE) // Set the maximum log level to TRACE
        .finish();
    // dbg!(&subscriber);
    subscriber.init();
}

pub mod constants;
pub mod specs;
pub mod statics;
mod tasks;
mod test_files;
