use std::{
    path::{Path, PathBuf},
    sync::MutexGuard,
};

use log::warn;
use refs::{Own, Weak};

use crate::{misc::hash, DataStorage, Managed};

pub trait DataManager<T: Managed> {
    fn root_path() -> &'static Path;
    fn set_root_path(path: impl Into<PathBuf>);

    fn storage() -> MutexGuard<'static, DataStorage<T>>;

    fn add_with_name(name: impl ToString, resource: T) -> Weak<T> {
        Self::add_with_hash(hash(name.to_string()), resource)
    }

    fn add_with_hash(hash: u64, resource: T) -> Weak<T> {
        let mut storage = Self::storage();
        if storage.contains_key(&hash) {
            warn!(
                "Object of type '{}' with hash: '{}' already exists",
                std::any::type_name::<T>().to_string(),
                hash
            );
        }
        let resource = Own::new(resource);
        let weak = resource.weak();
        storage.insert(hash, resource);
        weak
    }

    fn weak_with_name(name: impl ToString) -> Option<Weak<T>> {
        Self::weak_with_hash(hash(name.to_string()))
    }

    fn weak_with_hash(hash: u64) -> Option<Weak<T>> {
        Self::storage().get(&hash).map(Own::weak)
    }

    fn free(weak: Weak<T>) {
        if weak.is_null() {
            return;
        }
        let mut storage = Self::storage();
        let key = *storage
            .iter()
            .find(|(_, val)| val.addr() == weak.addr())
            .expect("Failed to find object to free.")
            .0;
        storage.remove(&key);
    }

    fn remove_with_name(name: impl ToString) {
        Self::remove_with_hash(hash(name.to_string()))
    }

    fn remove_with_hash(hash: u64) {
        Self::storage().remove(&hash);
    }

    fn get_weak_by_hash(hash: u64) -> Weak<T> {
        Self::storage().get(&hash).unwrap().weak()
    }

    fn get(name: impl ToString) -> Weak<T> {
        let name = name.to_string();
        let hash = hash(&name);
        let mut storage = Self::storage();
        let val = storage
            .entry(hash)
            .or_insert_with(|| Own::new(T::load_path(&Self::root_path().join(name))));
        val.weak()
    }

    fn load(data: &[u8], name: impl ToString) -> Weak<T> {
        let name = name.to_string();
        let hash = hash(&name);
        let mut storage = Self::storage();
        let val = storage.entry(hash).or_insert_with(|| Own::new(T::load_data(data, name)));
        val.weak()
    }
}
