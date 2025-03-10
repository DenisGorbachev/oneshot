use crate::Outcome;
use clap::Parser;
use std::io::Write;
use Command::*;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Print(PrintCommand),
    Debug(DebugCommand),
    Fork(ForkCommand),
}

impl Command {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self {
            Print(command) => command.run(stdout, stderr).await,
            Debug(command) => command.run(stdout, stderr).await,
            Fork(command) => command.run(stdout, stderr).await,
        }
    }
}

mod debug_command;
mod fork_command;
mod print_command;

pub use debug_command::*;
pub use fork_command::*;
pub use print_command::*;
