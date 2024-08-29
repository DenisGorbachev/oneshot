use derive_getters::{Dissolve, Getters};
use derive_new::new;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Named<T> {
    #[new(into)]
    name: String,

    #[new(into)]
    #[serde(flatten)]
    value: T,
}

impl<T> Named<T> {}
