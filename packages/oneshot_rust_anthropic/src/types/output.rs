use std::path::PathBuf;

use clap::{value_parser, Parser};

use serialize::format::Format;

use crate::constants::{CONVERSATIONS_SUBDIR, PROJECT_SUBDIR};
use crate::statics::PROJECT_DIRS;
use crate::types::output_target::OutputTarget;

#[derive(Parser, Debug)]
pub struct Output {
    #[arg(
        name = "output-target",
        help = "This option is overridden by the --output-dir option",
        long,
        value_enum,
        default_value_t
    )]
    pub target: OutputTarget,

    #[arg(name = "output-dir", long, value_parser = value_parser!(PathBuf))]
    pub dir: Option<PathBuf>,

    #[arg(name = "output-format", long, value_enum, default_value_t = Format::Yaml)]
    pub format: Format,
}

impl Output {
    pub fn dir(&self, package_root: impl Into<PathBuf>) -> Option<PathBuf> {
        match &self.dir {
            None => match &self.target {
                OutputTarget::None => None,
                OutputTarget::Package => Some(package_root.into().join(PROJECT_SUBDIR)),
                OutputTarget::Home => PROJECT_DIRS
                    .clone()
                    .map(|dirs| dirs.data_local_dir().to_path_buf()),
            }
            .map(|dir| dir.join(CONVERSATIONS_SUBDIR)),
            Some(path_buf) => Some(path_buf.to_owned()),
        }
    }
}
