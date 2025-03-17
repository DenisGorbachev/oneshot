use derive_more::From;
use fmt_derive::Display;
use std::io;

// Can't derive Error due to Vec<u8>. Maybe replace with String?
#[derive(Display, From, Debug)]
pub enum StrategyRunError {
    // TODO: Should we return the error or just let it be output to stderr by the subprocess?
    RunStartCmdError,
    RunTestCmdError(io::Error),
    TestCmdOutputToStderr(Vec<u8>),
}

impl StrategyRunError {}
