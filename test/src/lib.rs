#![feature(const_trait_impl)]
#![feature(effects)]
#![cfg(test)]

use std::path::Path;

use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader};

struct Data {
    a: i32,
}

impl ResourceLoader for Data {
    fn load_path(_path: &Path) -> Self {
        Data { a: 5 }
    }

    fn load_data(_data: &[u8], _name: impl ToString) -> Self {
        Data { a: 10 }
    }
}

managed!(Data);

#[test]
fn test() {
    Data::set_root_path("a");

    let data = Data::get("a");
    assert_eq!(data.a, 5);
}
