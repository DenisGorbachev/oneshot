use std::io::IsTerminal;

use clap::ValueEnum;
use strum::Display;

use Color::*;

#[derive(ValueEnum, Display, Default, Debug, Clone, Copy)]
pub enum Color {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<Color> for bool {
    fn from(value: Color) -> Self {
        match value {
            Always => true,
            Never => false,
            Auto => std::io::stdout().is_terminal(),
        }
    }
}