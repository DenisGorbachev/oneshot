use std::path::PathBuf;

use clap::{value_parser, Parser};
use clust::Client;
use fs_err::read_to_string;
use time::OffsetDateTime;

use clust_ext::functions::message::user_message;
use oneshot_common::functions::default_user_content::default_user_content_vec;
use oneshot_common::functions::get_parts_from_maybe_strings::collect_maybe_strings;
use oneshot_common::functions::role_from_language_maybe::role_from_language_maybe;
use oneshot_common::types::language::Language;
use oneshot_common::types::source_file::SourceFile;
use oneshot_utils::functions::find_package_root::{find_package_root, find_workspace_root_from_canonical_package_root_opt};
use oneshot_utils::traits::maybe_from::MaybeFrom;
use oneshot_utils::types::counter::Counter;

use crate::functions::acquire_conversation_dir_from_options::acquire_conversation_dir_from_options;
use crate::functions::create_a_message_with_output::create_a_message_with_output;
use crate::functions::messages_request_body::messages_request_body;
use crate::types::output_options::OutputOptions;
use crate::types::print_options::PrintOptions;

#[derive(Parser, Debug)]
pub struct Run {
    #[clap(flatten)]
    pub print_options: PrintOptions,

    #[clap(flatten)]
    pub output_options: OutputOptions,

    #[arg(name = "path", value_parser = value_parser!(PathBuf))]
    pub file_path_buf: PathBuf,
}

impl Run {
    #[allow(unused_variables)]
    pub async fn execute(self, client: Client, now: OffsetDateTime) -> anyhow::Result<()> {
        let Self {
            file_path_buf,
            output_options,
            print_options,
        } = self;
        let file_path = file_path_buf.as_path();
        let package_root_opt = find_package_root(file_path)?;
        let package_root_path_opt = package_root_opt.as_deref();
        let workspace_root_opt = find_workspace_root_from_canonical_package_root_opt(package_root_path_opt);
        let workspace_root_path_opt = workspace_root_opt.as_deref();
        let language_opt = Language::maybe_from(file_path);
        let role = role_from_language_maybe(language_opt);
        let file_content = read_to_string(file_path)?;
        // let related_source_files = get_related_files(file_path, &file_content)?;
        let source_file = SourceFile::new(file_path_buf.clone(), file_content);
        let output_dir_opt = output_options.dir(package_root_path_opt);
        let format = output_options.format;
        let mut request_counter = Counter::<u64>::default();

        let mut user_content_parts = default_user_content_vec(file_path, language_opt, package_root_path_opt, workspace_root_path_opt)?;
        user_content_parts.push(Some(source_file.serialize_to_xml()?));
        let user_content = collect_maybe_strings(user_content_parts);

        let request_body = messages_request_body(role, vec![user_message(user_content)]);

        let conversation_dir_opt = acquire_conversation_dir_from_options(output_dir_opt, now, &mut request_counter)?;
        let response_body = match conversation_dir_opt {
            Some(conversation_dir) => create_a_message_with_output(conversation_dir, format, &client, request_body).await?,
            None => client.create_a_message(request_body).await?,
        };

        Ok(())
    }
}
