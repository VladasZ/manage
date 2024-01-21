use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) fn hash(obj: impl ToString + Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
