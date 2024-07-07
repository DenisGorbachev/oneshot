use std::path::PathBuf;

use clap::{value_parser, Parser};
use clust::Client;
use time::OffsetDateTime;

use crate::types::output_options::OutputOptions;
use crate::types::print_options::PrintOptions;

#[derive(Parser, Debug)]
pub struct Run {
    #[clap(flatten)]
    pub print_options: PrintOptions,

    #[clap(flatten)]
    pub output_options: OutputOptions,

    #[arg(name = "path", value_parser = value_parser!(PathBuf))]
    pub path_buf: PathBuf,
}

impl Run {
    #[allow(unused_variables)]
    pub async fn execute(self, client: Client, now: OffsetDateTime) -> anyhow::Result<()> {
        let Self {
            path_buf,
            output_options,
            print_options,
        } = self;
        Ok(())
    }
}
