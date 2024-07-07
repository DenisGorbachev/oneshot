use std::hash::DefaultHasher;
use std::io::Write;
use std::path::PathBuf;

use anyhow::ensure;
use clap::{value_parser, Parser};
use clust::messages::MessagesRequestBody;
use clust::Client;
use constcat::concat;
use fs_err::{create_dir_all, read_to_string, File};
use prettyplease::unparse;
use time::OffsetDateTime;

use clust_ext::functions::into_text::into_text;
use clust_ext::functions::message::{assistant_message, user_message};
use oneshot_common::functions::intro::intro;
use oneshot_common::functions::pretty_printer::pretty_printer;
use oneshot_common::functions::readme::readme;
use oneshot_common::functions::role::role;
use oneshot_common::types::language::Language;
use oneshot_utils::functions::find_package_root::get_package_root;
use oneshot_utils::functions::hash::hash;
use serialize::functions::serialize_to_file::{serialize_to_file, SerializeToFileError};

use crate::functions::get_real_conversation_writer_from_dir_and_time::conversation_dir_if_not_exists;
use crate::functions::messages_request_body::messages_request_body;
use crate::types::color::Color;
use crate::types::output::Output;
use crate::types::strunk_error::StrunkError;

#[derive(Parser, Debug)]
pub struct Strunk {
    #[arg(long, short, env = "COLOR", value_enum, default_value_t)]
    pub color: Color,

    #[arg(long, short, env = "BAT_THEME")]
    pub theme: Option<String>,

    #[arg(long, short, default_value_t = true)]
    pub print: bool,

    #[arg(long, short)]
    pub overwrite: bool,

    #[clap(flatten)]
    pub output: Output,

    #[arg(name = "path", value_parser = value_parser!(PathBuf))]
    pub path_buf: PathBuf,
}

impl Strunk {
    pub async fn execute(self, client: Client, now: OffsetDateTime) -> anyhow::Result<()> {
        let Self {
            path_buf,
            color,
            theme,
            output,
            print,
            overwrite,
        } = self;
        let path = path_buf.as_path();
        let _cwd = env!("CARGO_MANIFEST_DIR");
        let package_root = get_package_root(path)?;
        let readme = readme(package_root);
        let language = Language::Rust;
        let role = role(language);

        let mut user_parts: Vec<String> = vec![];
        user_parts.push(intro(language));
        if readme.exists() {
            user_parts.push(read_to_string(readme)?)
        }
        user_parts.push(format!("A {language} source file with the path `{path:?}` is provided below. Write the code according to the specification in the file."));
        let user_content = user_parts.join("\n\n");

        let request_body = messages_request_body(role, vec![user_message(user_content)]);

        // let source_file_xml = SourceFile::new(path, file)?.serialize_to_xml()?;
        let file_content = read_to_string(path)?;
        let file = syn::parse_file(&file_content)?;

        let text = strunk(client, now, &output, package_root, request_body, file).await?;

        if print {
            if color.into() {
                let language_string = language.to_string();
                let mut printer = pretty_printer();
                if let Some(theme) = theme {
                    printer.theme(theme);
                }
                let no_errors = printer
                    .language(language_string.as_str())
                    .input_from_bytes(text.as_bytes())
                    .print()?;
                ensure!(no_errors, "Errors were encountered while pretty-printing");
            } else {
                println!("{text}");
            }
        }

        if overwrite {
            File::create(path)?.write_all(text.as_bytes())?;
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

async fn strunk(
    client: Client,
    now: OffsetDateTime,
    output: &Output,
    package_root: impl Into<PathBuf>,
    mut request_body: MessagesRequestBody,
    mut file: syn::File,
) -> Result<String, StrunkError> {
    file.items = vec![];

    let mut text = unparse(&file).trim_end().to_string();
    let assistant_content = text.clone();
    request_body
        .messages
        .push(assistant_message(assistant_content));

    let messages_hash = hash::<DefaultHasher>(&request_body.messages);

    let output_dir_opt = output
        .dir(package_root)
        .map(|output_dir| conversation_dir_if_not_exists(output_dir, now, messages_hash))
        .transpose()?;

    if let Some(output_dir) = output_dir_opt.as_ref() {
        create_dir_all(output_dir).map_err(SerializeToFileError::from)?;
        serialize_to_file(
            &request_body,
            output_dir.as_path(),
            "request",
            output.format,
        )?;
    }

    let response_body = client.create_a_message(request_body).await?;

    if let Some(output_dir) = output_dir_opt.as_ref() {
        serialize_to_file(
            &response_body,
            output_dir.as_path(),
            "response",
            output.format,
        )?;
    }

    let response_text = into_text(response_body);

    text.push_str(&response_text);

    Ok(text)
}
