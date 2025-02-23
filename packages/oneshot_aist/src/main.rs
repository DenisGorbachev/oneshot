use clap::Parser;
use oneshot_aist::{Cli, Outcome};
use std::io::{stderr, stdout};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    let mut stdout = stdout();
    let mut stderr = stderr();
    args.run(&mut stdout, &mut stderr).await
}
