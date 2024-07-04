use bat::PrettyPrinter;

pub fn pretty_printer<'a>() -> PrettyPrinter<'a> {
    let mut pretty_printer = PrettyPrinter::new();
    pretty_printer
        .true_color(false)
        .header(false)
        .line_numbers(false)
        .grid(false);
    pretty_printer
}
