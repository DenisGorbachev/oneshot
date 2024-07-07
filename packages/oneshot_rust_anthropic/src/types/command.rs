use clap::Subcommand;

use crate::types::run::Run;
use crate::types::strunk::Strunk;

#[derive(Subcommand, Debug)]
pub enum Command {
    Strunk(Strunk),
    Run(Run),
}
