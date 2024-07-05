use clap::Subcommand;

use crate::types::strunk::Run;

#[derive(Subcommand, Debug)]
pub enum Command {
    Run(Run),
}
