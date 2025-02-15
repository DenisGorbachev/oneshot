pub trait GetErrors {
    type Error;

    fn get_errors(&self) -> impl Iterator<Item = Self::Error>;
}
