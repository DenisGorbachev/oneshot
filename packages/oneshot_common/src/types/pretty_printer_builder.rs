use bat::PrettyPrinter;
use derive_getters::{Dissolve, Getters};
use derive_new::new;

use crate::functions::pretty_printer::pretty_printer;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrettyPrinterBuilder {
    theme: Option<String>,
    language: Option<String>,
}

impl PrettyPrinterBuilder {}

impl<'a> From<&'a PrettyPrinterBuilder> for PrettyPrinter<'a> {
    fn from(builder: &'a PrettyPrinterBuilder) -> Self {
        let mut printer = pretty_printer();
        if let Some(theme) = &builder.theme {
            printer.theme(theme);
        }
        if let Some(language) = &builder.language {
            printer.language(language.as_str());
        }
        printer
    }
}
