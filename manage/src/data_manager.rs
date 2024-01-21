use std::path::{Path, PathBuf};

use log::warn;
use refs::Own;

use crate::{handle::Handle, misc::hash, DataStorage, Managed};

pub trait DataManager<T: Managed> {
    fn root_path() -> &'static Path;
    fn set_root_path(path: impl Into<PathBuf>);

    fn storage() -> &'static mut DataStorage<T>;

    fn add_with_name(name: impl ToString, resource: T) -> Handle<T> {
        Self::add_with_hash(hash(name.to_string()), resource)
    }

    fn add_with_hash(hash: u64, resource: T) -> Handle<T> {
        let storage = Self::storage();
        if storage.contains_key(&hash) {
            warn!(
                "Object of type '{}' with hash: '{}' already exists",
                std::any::type_name::<T>().to_string(),
                hash
            );
        }
        storage.insert(hash, Own::new(resource));
        hash.into()
    }

    fn handle_with_name(name: impl ToString) -> Option<Handle<T>> {
        Self::handle_with_hash(hash(name.to_string()))
    }

    fn handle_with_hash(hash: u64) -> Option<Handle<T>> {
        if Self::storage().contains_key(&hash) {
            Some(hash.into())
        } else {
            None
        }
    }

    fn remove_with_name(name: impl ToString) {
        Self::remove_with_hash(hash(name.to_string()))
    }

    fn remove_with_hash(hash: u64) {
        Self::storage().remove(&hash);
    }

    fn get_ref_by_hash(hash: u64) -> &'static T {
        Self::storage().get(&hash).unwrap()
    }

    fn get_ref_by_hash_mut(hash: u64) -> &'static mut T {
        Self::storage().get_mut(&hash).unwrap()
    }

    fn get(name: impl ToString) -> Handle<T> {
        let name = name.to_string();
        let hash = hash(&name);
        Self::storage()
            .entry(hash)
            .or_insert_with(|| Own::new(T::load_path(&Self::root_path().join(name))));
        hash.into()
    }

    fn load(data: &[u8], name: impl ToString) -> Handle<T> {
        let name = name.to_string();
        let hash = hash(&name);
        Self::storage()
            .entry(hash)
            .or_insert_with(|| Own::new(T::load_data(data, name)));
        hash.into()
    }
}
