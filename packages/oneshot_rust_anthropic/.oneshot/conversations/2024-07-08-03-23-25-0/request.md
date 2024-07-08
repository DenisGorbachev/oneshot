Implement a Rust package according to the specification provided in the <readme> tags.


# OneShot Anthropic API

OneShot Anthropic API is a library that provides access to Anthropic API.

It has the following

## Specification

Use

## Implementation details

* Use clap v4 derive to parse the command-line arguments
* Use `get_string_until_finished` from `oneshot_utils` package to work around the `max_tokens` limit in Anthropic API.


[package]
name = "oneshot_rust_anthropic"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive-new = { workspace = true }
derive_more = { workspace = true }
fmt-derive = { workspace = true }
derive-getters = { workspace = true }
oneshot_common = { workspace = true }
oneshot_utils = { workspace = true }
anyhow = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
clap = { workspace = true }
clust = { workspace = true }
clust_ext = { path = "../clust_ext" }
syn = { workspace = true }
env_logger = { workspace = true }
tokio-stream = { workspace = true }
fs-err = { workspace = true }
oneshot_rust = { path = "../oneshot_rust" }
strum = { workspace = true }
prettyplease = { workspace = true }
throbber-widgets-tui = { workspace = true }
ratatui = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
serde_yaml = { workspace = true }
time = { workspace = true }
serialize = { path = "../serialize", features = ["clap", "serde_json", "serde_yaml"] }
constcat = { workspace = true }
directories = { workspace = true }
lazy_static = { workspace = true }
tap = { workspace = true }
bat = { workspace = true }


[package]
name = "oneshot_rust_anthropic"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive-new = { workspace = true }
derive_more = { workspace = true }
fmt-derive = { workspace = true }
derive-getters = { workspace = true }
oneshot_common = { workspace = true }
oneshot_utils = { workspace = true }
anyhow = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
clap = { workspace = true }
clust = { workspace = true }
clust_ext = { path = "../clust_ext" }
syn = { workspace = true }
env_logger = { workspace = true }
tokio-stream = { workspace = true }
fs-err = { workspace = true }
oneshot_rust = { path = "../oneshot_rust" }
strum = { workspace = true }
prettyplease = { workspace = true }
throbber-widgets-tui = { workspace = true }
ratatui = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
serde_yaml = { workspace = true }
time = { workspace = true }
serialize = { path = "../serialize", features = ["clap", "serde_json", "serde_yaml"] }
constcat = { workspace = true }
directories = { workspace = true }
lazy_static = { workspace = true }
tap = { workspace = true }
bat = { workspace = true }


A Rust source file with the path `"packages/oneshot_rust_anthropic/src/functions/get_related_files.rs"` is provided below. Write the code according to the specification in the file. Implement items marked with `todo!()`.

<?xml version="1.0" encoding="UTF-8"?><SourceFile><path_buf>packages/oneshot_rust_anthropic/src/functions/get_related_files.rs</path_buf><content>#![allow(unused_variables)]

use std::path::Path;

use derive_more::{Error, From};
use fmt_derive::Display;

use oneshot_common::types::source_file::SourceFile;

pub fn get_related_files(path: &amp;Path, content: &amp;str) -&gt; Result&lt;Vec&lt;SourceFile&gt;, GetRelatedFilesError&gt; {
    // TODO
    Ok(vec![])
}

#[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum GetRelatedFilesError {}
</content></SourceFile>