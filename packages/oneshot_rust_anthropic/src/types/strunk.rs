use std::path::PathBuf;

use anyhow::ensure;
use bat::PrettyPrinter;
use clap::{value_parser, Parser};
use clust::Client;
use constcat::concat;
use fs_err::{read_to_string, write};
use prettyplease::unparse;
use time::OffsetDateTime;

use clust_ext::functions::into_response_text::into_response_text;
use clust_ext::functions::message::{assistant_message, user_message};
use oneshot_common::functions::default_user_content::default_user_content;
use oneshot_common::functions::role_from_language_maybe::role_from_language_maybe;
use oneshot_common::types::language::Language;
use oneshot_common::types::pretty_printer_builder::PrettyPrinterBuilder;
use oneshot_utils::functions::find_package_root::find_package_root;
use oneshot_utils::traits::maybe_from::MaybeFrom;
use oneshot_utils::types::counter::Counter;

use crate::functions::create_a_message_with_output::create_a_message_with_output;
use crate::functions::get_real_conversation_writer_from_dir_and_time::acquire_conversation_dir;
use crate::functions::messages_request_body::messages_request_body;
use crate::types::color::Color;
use crate::types::output_options::OutputOptions;
use crate::types::print_options::PrintOptions;

#[derive(Parser, Debug)]
pub struct Strunk {
    #[clap(flatten)]
    pub print_options: PrintOptions,

    #[clap(flatten)]
    pub output_options: OutputOptions,

    #[arg(long, short)]
    pub overwrite: bool,

    #[arg(name = "path", value_parser = value_parser!(PathBuf))]
    pub file_path_buf: PathBuf,
}

impl Strunk {
    pub async fn execute(self, client: Client, now: OffsetDateTime) -> anyhow::Result<()> {
        let Self {
            file_path_buf,
            output_options,
            print_options,
            overwrite,
        } = self;
        // let info = FileInfo::from(path_buf);
        let file_path = file_path_buf.as_path();
        let package_root_opt = find_package_root(file_path);
        let language_opt = Language::maybe_from(file_path);

        let user_content = default_user_content(file_path, package_root_opt, language_opt)?;

        let mut request_counter = Counter::<u64>::default();

        // let source_file_xml = SourceFile::new(path, file)?.serialize_to_xml()?;
        let file_content = read_to_string(file_path)?;

        let role = role_from_language_maybe(language_opt);
        let mut request_body = messages_request_body(role, vec![user_message(user_content)]);

        let mut file = syn::parse_file(&file_content)?;
        file.items = vec![];
        let mut text = unparse(&file).trim_end().to_string();

        request_body.messages.push(assistant_message(text.clone()));

        let response_body = if let Some(output_dir) = output_options.dir(package_root_opt) {
            let conversation_dir =
                acquire_conversation_dir(output_dir, now, request_counter.take())?;
            create_a_message_with_output(
                conversation_dir,
                output_options.format,
                &client,
                request_body,
            )
            .await?
        } else {
            client.create_a_message(request_body).await?
        };

        let response_text = into_response_text(response_body);

        text.push_str(&response_text);

        if print_options.print {
            let printer_builder = PrettyPrinterBuilder::new(
                print_options.theme,
                language_opt.as_ref().map(ToString::to_string),
            );
            do_print(&response_text, print_options.color, &printer_builder)?;
        }

        if overwrite {
            write(file_path, &response_text)?;
        }

        // let mut stream = client.create_a_message_stream(body).await?;
        //
        // while let Some(chunk_result) = stream.next().await {
        //     let chunk = chunk_result?;
        //     println!("{chunk}");
        // }

        Ok(())
    }
}

fn do_print<'a>(
    response_text: &String,
    color: Color,
    printer_builder: impl Into<PrettyPrinter<'a>>,
) -> anyhow::Result<()> {
    if color.into() {
        let no_errors = printer_builder
            .into()
            .input_from_bytes(response_text.as_bytes())
            .print()?;
        ensure!(no_errors, "Errors were encountered while pretty-printing");
    } else {
        println!("{response_text}");
    }
    Ok(())
}
