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
fierce_nerds_node = { path = "../../../zenbox/fierce_nerds_node" }
quote = { workspace = true }
indexmap = { workspace = true }
rustc-hash = { workspace = true }


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
fierce_nerds_node = { path = "../../../zenbox/fierce_nerds_node" }
quote = { workspace = true }
indexmap = { workspace = true }
rustc-hash = { workspace = true }


A Rust source file with the path `"packages/oneshot_rust_anthropic/src/specs/into_file_paths.rs"` is provided below. Write the code according to the specification in the file. Implement items marked with `todo!()`.

<?xml version="1.0" encoding="UTF-8"?><SourceFile><path_buf>Justfile</path_buf><content>import? 'Justfile.local.just'

set dotenv-path := "dev.env"
set quiet

default:
  just --list

build *args:
  cargo build {{args}}

lint *args:
  cargo clippy --all-targets --all-features {{args}} -- -D warnings

test *args:
  cargo nextest run {{args}}

watch *args:
  #!/usr/bin/env bash
  set -euxo pipefail
  PWD=$(pwd)
  CMD_RAW="nextest run $*"
  CMD_NO_WHITESPACE="$(echo -e "${CMD_RAW}" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')"
  cargo watch --clear --watch "$PWD" --exec "$CMD_NO_WHITESPACE"

check *args:
  cargo check --all-targets "$@"

fix *args:
  cargo fix --workspace --allow-dirty --allow-staged {{args}}

reset *args:
  supabase db reset
  just migrate {{args}}
</content></SourceFile>

<?xml version="1.0" encoding="UTF-8"?><SourceFile><path_buf>packages/oneshot_rust_anthropic/src/types/run.rs</path_buf><content>use std::path::PathBuf;

use clap::{Parser, value_parser};
use clust::Client;
use time::OffsetDateTime;

use clust_ext::functions::message::user_message;
use oneshot_common::functions::default_user_content::default_user_content_vec;
use oneshot_common::functions::get_parts_from_maybe_strings::join_message_parts;
use oneshot_common::functions::role_from_language_maybe::role_from_language_maybe;
use oneshot_common::types::language::Language;
use oneshot_common::types::source_file::SourceFile;
use oneshot_utils::functions::find_package_root::{find_package_root, find_workspace_root_from_canonical_package_root_opt};
use oneshot_utils::traits::maybe_from::MaybeFrom;
use oneshot_utils::types::counter::Counter;

use crate::functions::acquire_conversation_dir_from_options::acquire_conversation_dir_from_options;
use crate::functions::create_a_message_with_output::create_a_message_with_output;
use crate::functions::messages_request_body::messages_request_body;
use crate::functions::strip_line_number::strip_line_number_from_path_buf;
use crate::specs::to_related_files_spec::get_source_files_from_path_buf;
use crate::types::output_options::OutputOptions;
use crate::types::print_options::PrintOptions;

#[derive(Parser, Debug)]
pub struct Run {
    #[clap(flatten)]
    pub print_options: PrintOptions,

    #[clap(flatten)]
    pub output_options: OutputOptions,

    #[arg(name = "extra-files", long, value_delimiter = ',', value_parser = value_parser!(PathBuf))]
    pub extra_file_path_bufs: Vec&lt;PathBuf&gt;,

    #[arg(name = "path", value_parser = value_parser!(PathBuf))]
    pub file_path_buf: PathBuf,
}

impl Run {
    #[allow(unused_variables)]
    pub async fn execute(self, client: Client, now: OffsetDateTime) -&gt; anyhow::Result&lt;()&gt; {
        let Self {
            output_options,
            print_options,
            mut extra_file_path_bufs,
            mut file_path_buf,
        } = self;
        strip_line_number_from_path_buf(&amp;mut file_path_buf);
        extra_file_path_bufs.iter_mut().for_each(strip_line_number_from_path_buf);
        let file_path = file_path_buf.as_path();
        let package_root_opt = find_package_root(file_path)?;
        let package_root_path_opt = package_root_opt.as_deref();
        let workspace_root_opt = find_workspace_root_from_canonical_package_root_opt(package_root_path_opt);
        let workspace_root_path_opt = workspace_root_opt.as_deref();
        let language_opt = Language::maybe_from(file_path);
        let role = role_from_language_maybe(language_opt);
        // let file_content = read_to_string(file_path)?;
        // related_source_files includes the current file
        // let related_source_files = get_source_files_from_path_buf(file_path_buf.clone())?;
        // let related_source_files_xml = SourceFile::to_xml_many(related_source_files.as_slice())?;
        let extra_source_files = SourceFile::from_path_bufs(extra_file_path_bufs)?;
        let extra_source_files_xml = SourceFile::to_xml_many(extra_source_files.as_slice())?;
        // let source_file = SourceFile::new(file_path_buf.clone(), file_content);
        let output_dir_opt = output_options.dir(package_root_path_opt);
        let format = output_options.format;
        let mut request_counter = Counter::&lt;u64&gt;::default();

        let user_content_parts = default_user_content_vec(file_path, language_opt, package_root_path_opt, workspace_root_path_opt)?
            .into_iter()
            .flatten()
            // .chain(related_source_files_xml)
            .chain(extra_source_files_xml);
        // .chain(once(source_file.to_xml()?));
        let user_content = join_message_parts(user_content_parts);

        let request_body = messages_request_body(role, vec![user_message(user_content)]);

        let conversation_dir_opt = acquire_conversation_dir_from_options(output_dir_opt, now, &amp;mut request_counter)?;
        let response_body = match conversation_dir_opt {
            Some(conversation_dir) =&gt; create_a_message_with_output(conversation_dir, format, &amp;client, request_body).await?,
            None =&gt; client.create_a_message(request_body).await?,
        };

        Ok(())
    }
}
</content></SourceFile>