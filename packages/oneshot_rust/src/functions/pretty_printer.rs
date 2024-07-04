use bat::PrettyPrinter;

pub fn pretty_printer<'a>() -> PrettyPrinter<'a> {
    let mut printer = oneshot_common::functions::pretty_printer::pretty_printer();
    printer.language("rust");
    printer
}
