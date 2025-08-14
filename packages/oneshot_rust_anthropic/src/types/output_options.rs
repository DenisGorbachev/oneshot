use std::path::PathBuf;

use clap::{Parser, value_parser};
use save_load::format::Format;

use crate::constants::{CONVERSATIONS_SUBDIR, PROJECT_SUBDIR};
use crate::statics::PROJECT_DIRS;
use crate::types::output_target::OutputTarget;

#[derive(Parser, Debug)]
pub struct OutputOptions {
    #[arg(name = "output-target", help = "This option is overridden by the --output-dir option", long, value_enum, default_value_t)]
    pub target: OutputTarget,

    #[arg(name = "output-dir", long, value_parser = value_parser!(PathBuf))]
    pub dir: Option<PathBuf>,

    #[arg(name = "output-format", long, value_enum, default_value_t = Format::Yaml)]
    pub format: Format,
}

impl OutputOptions {
    pub fn dir(&self, package_root_opt: Option<impl Into<PathBuf>>) -> Option<PathBuf> {
        match &self.dir {
            None => match &self.target {
                OutputTarget::None => None,
                OutputTarget::Package => package_root_opt.map(|package_root| package_root.into().join(PROJECT_SUBDIR)),
                OutputTarget::Home => PROJECT_DIRS
                    .clone()
                    .map(|dirs| dirs.data_local_dir().to_path_buf()),
            }
            .map(|dir| dir.join(CONVERSATIONS_SUBDIR)),
            Some(path_buf) => Some(path_buf.to_owned()),
        }
    }
}
