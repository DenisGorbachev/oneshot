use clap::ValueEnum;
use derive_more::From;

/// What to do if test cmd outputs something to stderr
#[derive(From, ValueEnum, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Copy, Debug)]
pub enum TestCmdStderrAction {
    #[default]
    Append,
    Abort,
}
impl TestCmdStderrAction {}
