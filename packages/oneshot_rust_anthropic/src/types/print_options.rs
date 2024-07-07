use clap::Parser;
use derive_getters::{Dissolve, Getters};
use derive_new::new;

use crate::types::color::Color;

#[derive(new, Getters, Dissolve, Parser, Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintOptions {
    #[arg(
        name = "print-color",
        long,
        short,
        env = "COLOR",
        value_enum,
        default_value_t
    )]
    pub color: Color,

    #[arg(name = "print-theme", long, short, env = "BAT_THEME")]
    pub theme: Option<String>,

    #[arg(long, short, default_value_t = true)]
    pub print: bool,
}

impl PrintOptions {}
