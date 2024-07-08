use anyhow::ensure;
use bat::PrettyPrinter;

use crate::types::color::Color;

pub fn do_print<'a>(response_text: &String, color: Color, printer_builder: impl Into<PrettyPrinter<'a>>) -> anyhow::Result<()> {
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
