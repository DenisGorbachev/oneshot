use crate::{GetErrors, TryFork};

pub trait Context: GetErrors + TryFork {}
