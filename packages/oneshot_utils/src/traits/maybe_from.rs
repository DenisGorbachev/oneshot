pub trait MaybeFrom<T>: Sized {
    fn maybe_from(value: T) -> Option<Self>;
}
