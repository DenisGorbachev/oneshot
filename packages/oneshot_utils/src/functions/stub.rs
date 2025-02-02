/// Used to avoid triggering "unreachable code" warning in client code
pub fn stub<T>() -> T {
    todo!()
}

pub fn stub_iter<T>() -> impl Iterator<Item = T> {
    todo!();
    #[allow(unreachable_code)]
    [].into_iter()
}

pub fn fixme<T>(msg: &str) -> T {
    todo!("{}", msg)
}

#[macro_export]
macro_rules! stub {
    // Case when the first argument is a string literal
    ($msg:literal, $($args:tt)*) => {{
        $crate::stub::fixme::<_>($msg)
    }};
    // Default case for when the first argument is not a string literal
    ($($args:tt)*) => {{
        $crate::stub::stub::<_>()
    }};
}
