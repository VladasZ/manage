#![feature(arbitrary_self_types)]

use std::collections::HashMap;

use crate::{data_manager::DataManager, resource_loader::ResourceLoader};

pub mod data_manager;
mod exists_managed;
pub mod managed;
pub mod resource_loader;

pub use exists_managed::*;
pub use refs::MainLock;
use refs::Own;

pub type DataStorage<T> = HashMap<String, Own<T>>;

pub trait Managed: 'static + ResourceLoader + DataManager<Self> {}
