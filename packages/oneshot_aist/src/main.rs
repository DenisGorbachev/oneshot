use clap::Parser;
use oneshot_aist::{Cli, Outcome};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    args.run().await
}
