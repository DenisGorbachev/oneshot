use crate::{Outcome, Strategy};
use clap::{value_parser, Parser};
use itertools::Itertools;
use std::io::Write;
use std::path::PathBuf;

/// This command attempts to fix a single failing test with multiple different approaches in parallel. This speeds up the fixing process, giving you a fixed test in less time. It also increases the probability of actually fixing the test because the approaches may be vastly different (e.g. different LLMs, different tools).
/// This command creates a separate git branch for each fix attempt. It commits the changes even if those changes do not fix the test.
/// This command does not push the local branches to remotes
#[derive(Parser, Clone, Debug)]
pub struct DebugCommand {
    /// Base branch name for newly created branches
    #[arg(short = 'b')]
    base_branch_name: String,

    /// The files with configuration options for different approaches
    /// If multiple files are passed, they will be merged together
    #[arg(trailing_var_arg = true, value_parser = value_parser!(PathBuf))]
    strategies: Vec<PathBuf>,
}

impl DebugCommand {
    /// The user may change the approach name while keeping the same config
    /// The user may change the approach config while keeping the same name
    /// We can't assume that for any name the approach config between runs will be the same (because the user may change the approach config while keeping the same name). Thus, we need to hash the approach config to get a more reliable identifier. A hash collision is still possible but unlikely.
    pub async fn run(self, stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        let Self {
            base_branch_name: _,
            strategies: strategies_path_bufs,
        } = self;
        let strategies: Vec<Strategy> = Strategy::try_from_many_path_bufs(strategies_path_bufs).try_collect()?;
        let _results = strategies.into_iter().map(async |s| s.run().await);
        // TODO: The approaches config must contain a timeout for both the test cmd and the fix cmd
        writeln!(stdout, "Calculate the hash of the approach configuration")?;
        writeln!(stdout, "Run the test cmd")?;
        Ok(())
    }
}
