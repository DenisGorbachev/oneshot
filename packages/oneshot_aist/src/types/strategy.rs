use crate::{Goal, StrategyRunError, TestCmdStderrAction};
use clap::Parser;
use derive_getters::Getters;
use derive_more::{From, Into};
use std::ffi::OsStr;
use std::hash::Hash;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use stub_macro::stub;

/// A strategy represents an approach to fixing the test
#[derive(Parser, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Strategy {
    shell_prog: String,

    shell_args: Vec<String>,

    /// Command to run at the start of the strategy execution
    start_cmd: String,

    /// Command to run the tests
    /// This command must output the errors to stdout.
    #[arg(short = 't')]
    test_cmd: String,

    test_cmd_stderr_action: TestCmdStderrAction,

    /// Maximum duration (in seconds) for `--fix-cmd`
    #[arg(default_value_t = 600)]
    test_cmd_timeout: u64,

    /// Command to run for fixing errors.
    /// This command will be invoked only if the `--test-cmd` exists with a non-zero status code
    #[arg(short = 'f')]
    fix_cmd: String,

    /// Maximum duration (in seconds) for `--fix-cmd`
    #[arg(default_value_t = 600)]
    fix_cmd_timeout: u64,
}

impl Strategy {
    pub async fn run(&self) -> Result<(), StrategyRunError> {
        use StrategyRunError::*;

        let status = self.run_shell_cmd(&self.start_cmd)?;
        if !status.success() {
            return Err(RunStartCmdError);
        }
        let _initial_prompt = self.get_initial_prompt(stub!(), stub!())?;
        // let Output {
        //     status,
        //     mut stdout,
        //     stderr,
        // } = self.run_shell_cmd().map_err(RunTestCmdError)?;
        // if status.success() {
        //     return Ok(());
        // } else {
        //     match self.test_cmd_stderr_action {
        //         Append => stdout.extend(stderr),
        //         Abort => {}
        //     }
        // }
        todo!()
    }

    pub fn run_shell_cmd(&self, cmd: impl AsRef<OsStr>) -> io::Result<ExitStatus> {
        Command::new(&self.shell_prog)
            .args(&self.shell_args)
            .arg(cmd)
            .status()
    }

    /// TODO: Choose a specific hash function and remove the `hash` arg
    pub fn branch_name(&self, base_branch_name: &str, hash: impl FnOnce(&Self) -> String) -> String {
        let self_hash = hash(self);
        format!("{base_branch_name}/{self_hash}")
    }

    pub fn try_from_many_path_bufs(sources: impl IntoIterator<Item = PathBuf>) -> impl Iterator<Item = Result<Self, io::Error>> {
        sources.into_iter().map(Self::try_from)
    }

    // pub fn try_from_many_path_bufs(sources: impl IntoIterator<Item = PathBuf>) -> impl Iterator<Item = Result<Self, io::Error>> {
    //     Self::try_from_many_paths(sources.into_iter().map(PathBuf::as_path))
    // }

    pub fn get_initial_prompt(&self, _root: &Path, _ignores: &[GlobPattern]) -> io::Result<Goal> {
        // Asynchronously walk the children of root_dir
        // Ignore the hidden files
        // Ignore the files from .gitignore
        // Ignore the files from _ignores
        // Return the first file that contains a line with a comment that starts with "AI!"
        // Return both the PathBuf and the String that contains the task (everything on this line after "AI!", trimmed)
        todo!()
    }
}

type GlobPattern = Todo;

type Todo = ();

impl TryFrom<&Path> for Strategy {
    type Error = io::Error;

    fn try_from(_value: &Path) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<PathBuf> for Strategy {
    type Error = io::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        value.as_path().try_into()
    }
}
