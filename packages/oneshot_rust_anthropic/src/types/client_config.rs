use clap::Parser;

#[derive(Parser, Debug)]
pub struct ClientConfig {
    #[arg(long, short)]
    verbose: bool,
}
