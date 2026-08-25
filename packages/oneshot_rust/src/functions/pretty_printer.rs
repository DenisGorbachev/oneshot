use bat::PrettyPrinter;
use oneshot_common::functions::pretty_printer::pretty_printer as common_pretty_printer;

pub fn pretty_printer<'a>() -> PrettyPrinter<'a> {
    let mut printer = common_pretty_printer();
    printer.language("rust");
    printer
}
