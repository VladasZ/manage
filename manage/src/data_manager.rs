use std::{
    any::type_name,
    path::{Path, PathBuf},
    sync::MutexGuard,
};

use log::error;
use refs::{Own, Weak};

use crate::{DataStorage, Managed};

pub trait DataManager<T: Managed> {
    fn root_path() -> &'static Path;
    fn set_root_path(path: impl Into<PathBuf>);

    fn storage() -> MutexGuard<'static, DataStorage<T>>;

    fn add_with_name(name: impl ToString, resource: T) -> Weak<T> {
        let name = name.to_string();
        let mut storage = Self::storage();
        if storage.contains_key(&name) {
            error!(
                "Object of type '{}' with name: '{name}' already exists",
                type_name::<T>(),
            );
            panic!(
                "Object of type '{}' with name: '{name}' already exists",
                type_name::<T>(),
            )
        }
        let resource = Own::new(resource);
        let weak = resource.weak();
        storage.insert(name, resource);
        weak
    }

    fn weak_with_name(name: &str) -> Option<Weak<T>> {
        Self::storage().get(name).map(Own::weak)
    }

    fn free(weak: Weak<T>) {
        if weak.is_null() {
            return;
        }
        let mut storage = Self::storage();
        let key = storage
            .iter()
            .find(|(_, val)| val.addr() == weak.addr())
            .expect("Failed to find object to free.")
            .0
            .clone();
        storage.remove(&key);
    }

    fn remove_with_name(name: &str) {
        Self::storage().remove(name);
    }

    fn get(name: impl ToString) -> Weak<T> {
        let name = name.to_string();
        let mut storage = Self::storage();
        let val = storage
            .entry(name.clone())
            .or_insert_with(|| Own::new(T::load_path(&Self::root_path().join(name))));
        val.weak()
    }

    fn load(data: &[u8], name: impl ToString) -> Weak<T> {
        let name = name.to_string();
        let mut storage = Self::storage();
        let val = storage
            .entry(name.clone())
            .or_insert_with(|| Own::new(T::load_data(data, name)));
        val.weak()
    }
}
