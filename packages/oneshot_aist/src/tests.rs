//! This module is gated under obscure `target_env = "nto70"` to avoid running the tests for unimplemented functions, but at the same time keep the code the same as if it were in a real project

#![cfg(all(test, target_env = "nto70"))]

mod count_files_in_dir;

pub use count_files_in_dir::*;
