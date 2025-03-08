use crate::{BranchName, Outcome};
use clap::{value_parser, Parser};
use duct::cmd;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Output;
use stub_macro::stub;

/// This command attempts to fix a single failing test with multiple different approaches in parallel. This speeds up the fixing process, giving you a fixed test in less time. It also increases the probability of actually fixing the test because the approaches may be vastly different (e.g. different LLMs, different tools).
/// This command creates a separate git branch for each fix attempt. It commits the changes even if those changes do not fix the test.
/// This command does not push the local branches to remotes
#[derive(Parser, Clone, Debug)]
pub struct ForkCommand {
    #[arg(long, short = 'p', value_parser = value_parser!(BranchName))]
    parent_branch: BranchName,

    #[arg(long, short = 'c', value_parser = value_parser!(BranchName))]
    child_branch: BranchName,

    /// Command to execute after checkout
    /// The `cwd` of the command is the repository directory
    #[arg(long)]
    post_checkout_cmd: String,

    /// A directory for cloned repositories
    /// Must include the project name
    #[arg(long, short, default_value = "/tmp", value_parser = value_parser!(PathBuf))]
    dir: PathBuf,

    /// Git repository url
    #[arg(long, short)]
    repo_url: String,
}

impl ForkCommand {
    /// The user may change the approach name while keeping the same config
    /// The user may change the approach config while keeping the same name
    /// We can't assume that for any name the approach config between runs will be the same (because the user may change the approach config while keeping the same name). Thus, we need to hash the approach config to get a more reliable identifier. A hash collision is still possible but unlikely.
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        let Self {
            parent_branch,
            child_branch: _,
            post_checkout_cmd,
            dir,
            repo_url,
        } = self;
        let child_branch_path_buf = stub!(PathBuf);
        let clone_dir = dir.join(child_branch_path_buf);
        let _clone_output = cmd!("git", "clone", "--recurse-submodules", "--branch", parent_branch, &repo_url, clone_dir).run()?;
        exec(&post_checkout_cmd)?;
        Ok(())
    }
}

pub fn exec(_cmd: &str) -> io::Result<Output> {
    todo!()
}
