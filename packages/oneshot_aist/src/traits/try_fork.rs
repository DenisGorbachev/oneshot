/// May perform filesystem/network operations (more generally: mutate the external state)
pub trait TryFork {
    type Output;
    type Error;

    fn try_fork(&self) -> Result<Self::Output, Self::Error>;
}
