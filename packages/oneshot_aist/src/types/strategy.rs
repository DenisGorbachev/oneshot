use crate::{StrategyRunError, TestCmdStderrAction};
use clap::Parser;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::hash::Hash;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

/// A strategy represents an approach to fixing the test
#[derive(new, Parser, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Strategy {
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
        use TestCmdStderrAction::*;
        let Output {
            status,
            mut stdout,
            stderr,
        } = self.run_test_cmd().map_err(RunTestCmdError)?;
        if status.success() {
            return Ok(());
        } else {
            match self.test_cmd_stderr_action {
                Append => stdout.extend(stderr),
                Abort => {}
            }
        }
        todo!()
    }

    pub fn run_test_cmd(&self) -> io::Result<Output> {
        todo!()
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
}

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
