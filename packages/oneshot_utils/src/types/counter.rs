use std::ops::AddAssign;

use derive_getters::{Dissolve, Getters};
use derive_new::new;
use num_traits::One;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Counter<T: Clone + One + AddAssign> {
    value: T,
}

impl<T: Clone + One + AddAssign> Counter<T> {
    pub fn take(&mut self) -> T {
        let current = self.value.clone();
        self.value.add_assign(T::one());
        current
    }
}
