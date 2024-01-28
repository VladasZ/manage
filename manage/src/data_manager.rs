use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::{DataStorage, Managed};

pub trait DataManager<T: Managed> {
    fn root_path() -> &'static Path;
    fn set_root_path(path: impl Into<PathBuf>);

    fn storage() -> &'static mut DataStorage<T>;

    fn add_with_name(name: &str, create: impl FnOnce() -> T) -> &'static T {
        Self::storage().entry(name.to_string()).or_insert_with(|| create().into())
    }

    fn remove_with_name(name: &str) {
        Self::storage().remove(name).expect("This name '{name}' is not managed.");
    }

    fn get_existing(name: impl ToString) -> Option<&'static T> {
        Self::storage().get(&name.to_string()).map(|a| a.deref())
    }

    fn get(name: impl ToString) -> &'static T {
        let name = name.to_string();
        let storage = Self::storage();
        let val = storage
            .entry(name.clone())
            .or_insert_with(|| T::load_path(&Self::root_path().join(name)).into());
        val
    }

    fn load(data: &[u8], name: impl ToString) -> &'static T {
        let name = name.to_string();
        let storage = Self::storage();
        let val = storage.entry(name.clone()).or_insert_with(|| T::load_data(data, name).into());
        val
    }
}
