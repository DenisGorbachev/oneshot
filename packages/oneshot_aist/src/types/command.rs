use crate::Outcome;
use Command::*;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Print(PrintCommand),
    Debug(DebugCommand),
    Fork(ForkCommand),
}

impl Command {
    pub async fn run(self) -> Outcome {
        match self {
            Print(command) => command.run().await,
            Debug(command) => command.run().await,
            Fork(command) => command.run().await,
        }
    }
}

mod debug_command;
mod fork_command;
mod print_command;

pub use debug_command::*;
pub use fork_command::*;
pub use print_command::*;
