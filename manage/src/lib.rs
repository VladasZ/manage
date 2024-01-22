use std::collections::HashMap;

use refs::Own;

use crate::{data_manager::DataManager, resource_loader::ResourceLoader};

pub mod data_manager;
pub mod managed;
mod misc;
pub mod resource_loader;

pub type DataStorage<T> = HashMap<u64, Own<T>>;

pub trait Managed: 'static + ResourceLoader + DataManager<Self> {}
