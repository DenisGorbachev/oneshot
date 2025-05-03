use crate::{BranchName, Outcome};
use clap::{value_parser, Parser};
use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;
use stub_macro::stub;
use xshell::{cmd, Shell};

/// This command attempts to fix a single failing test with multiple different approaches in parallel. This speeds up the fixing process, giving you a fixed test in less time. It also increases the probability of actually fixing the test because the approaches may be vastly different (e.g. different LLMs, different tools).
/// This command creates a separate git branch for each fix attempt. It commits the changes even if those changes do not fix the test.
/// This command does not push the local branches to remotes
#[derive(Parser, Clone, Debug)]
pub struct ForkCommand {
    /// Directory of the parent repository (must already exist; must not have uncommitted changes)
    /// Defaults to the current directory
    #[arg(long, short = 'd', value_parser = value_parser!(PathBuf))]
    parent_dir: Option<PathBuf>,

    /// Directory for cloned repositories (children)
    /// If omitted, the directory be determined automatically (displayed in the output log)
    #[arg(long, short, value_parser = value_parser!(PathBuf))]
    children_dir: Option<PathBuf>,

    #[arg(long, short = 'b', value_parser = value_parser!(BranchName))]
    parent_branch: Option<BranchName>,

    #[arg(long, value_parser = value_parser!(BranchName))]
    child_branch: Option<BranchName>,

    /// The name of the parent repo
    /// If not provided, it will be determined automatically from `parent_dir`
    /// Used to generate the child repo dir
    /// If the name contains slashes, they will not be escaped (so that every part of the parent_name will become its own directory)
    /// For example, if the parent_name is "github.com/username/foo", then the the child repo dir resides in "github.com/username/foo" (each part is a directory)
    #[arg(long, short = 'n')]
    parent_name: Option<String>,

    /// Command to execute after checkout
    /// The `cwd` of the command is the repository directory
    #[arg(long)]
    post_checkout_cmd: String,
}

impl ForkCommand {
    /// The user may change the approach name while keeping the same config
    /// The user may change the approach config while keeping the same name
    /// We can't assume that for any name the approach config between runs will be the same (because the user may change the approach config while keeping the same name). Thus, we need to hash the approach config to get a more reliable identifier. A hash collision is still possible but unlikely.
    pub async fn run(self) -> Outcome {
        // TODO: data_local_dir
        let Self {
            parent_branch: _,
            child_branch: _,
            post_checkout_cmd,
            children_dir: _,
            parent_dir,
            parent_name: _,
        } = self;
        // TODO: Collect git utils from `personal`, `repoconf`, maybe other dirs (search for "\"git" (the start of the command))
        // TODO: Extract unwrap_or_current_dir
        let parent_dir = parent_dir.unwrap_or_else(|| current_dir().unwrap());
        let _parent_sh = Shell::new()?.with_current_dir(&parent_dir);
        // TODO: Check if parent_dir is a git repository
        // TODO: Check if parent_dir is clean
        let parent_name = stub!(String);
        let parent_url = stub!(String);
        let parent_branch = stub!(String, "Extract from existing parent_branch");
        let children_dir = stub!(PathBuf, "Extract from existing children_dir, apply defaults");
        let _child_branch_path_buf = stub!(PathBuf);
        let child_branch_as_dir = stub!(PathBuf, "May need a special conversion from the branch name, so that the slashes are not escaped");
        let child_dir = children_dir.join(parent_name).join(child_branch_as_dir);
        create_dir_all(&child_dir)?;
        let child_sh = Shell::new()?.with_current_dir(&child_dir);
        // TODO: If the directory already exists, check if we can restart the process
        cmd!(child_sh, "git clone --recurse-submodules --branch {parent_branch} {parent_url} {child_dir}").run_echo()?;
        cmd!(child_sh, "sh -c {post_checkout_cmd}").run_echo()?;
        Ok(())
    }
}

pub fn get_current_branch(sh: &Shell) -> xshell::Result<String> {
    cmd!(sh, "git rev-parse --abbrev-ref HEAD").read()
}

pub fn get_current_remote(sh: &Shell, branch_name: BranchName) -> xshell::Result<String> {
    let key = format!("branch.{branch_name}.remote");
    cmd!(sh, "git config --get {key}").read()
}
