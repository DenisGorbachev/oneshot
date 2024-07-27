#[macro_export]
macro_rules! notes {
    ($($title:literal $({ $($children:tt)* })?)*) => {
        $(
            let _ = $title;
            $(
                $crate::notes!($($children)*);
            )?
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn must_expand_notes() {
        // trace_macros!(true);
        notes!("Single item");
        notes!(
            "First item"
            "Second item"
        );
        notes!(
            "1" {}
            "2" {
                "2.1" {
                    "2.1.1"
                }
                "2.2"
            }
            "3"
        );
        // trace_macros!(false);
    }
}
