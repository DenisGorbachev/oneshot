use std::hash::{Hash, Hasher};

pub fn hash<H: Hasher + Default>(value: &impl Hash) -> u64 {
    let mut hasher = H::default();
    value.hash(&mut hasher);
    hasher.finish()
}
