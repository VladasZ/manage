#![feature(const_trait_impl)]
#![cfg(test)]

use std::{
    path::Path,
    sync::atomic::{AtomicU32, Ordering},
};

use manage::{data_manager::DataManager, managed, resource_loader::ResourceLoader, ExistsManaged};
use refs::{set_current_thread_as_main, Weak};

static COUNTER: AtomicU32 = AtomicU32::new(0);

struct Data {
    a:    u32,
    name: String,
}

impl ResourceLoader for Data {
    fn load_path(_path: &Path) -> Self {
        Data {
            a:    COUNTER.fetch_add(1, Ordering::Relaxed),
            name: "some_data".to_string(),
        }
    }

    fn load_data(_data: &[u8], _name: impl ToString) -> Self {
        unimplemented!()
    }
}

managed!(Data);

#[test]
fn test() {
    set_current_thread_as_main();

    Data::set_root_path("a");

    let data = Data::get("a");
    assert_eq!(data.a, 0);
    assert_eq!(data.name, "some_data");

    data.free();

    let data = Data::get("a");

    assert_eq!(data.a, 1);

    Data::add_with_name("b", || Data {
        a:    COUNTER.fetch_add(1, Ordering::Relaxed),
        name: "".to_string(),
    });

    assert_eq!(Data::get("b").a, 2);

    Data::add_with_name("b", || Data {
        a:    COUNTER.fetch_add(1, Ordering::Relaxed),
        name: "".to_string(),
    });

    assert_eq!(Data::get("b").a, 2);
    assert_eq!(Data::get("b").get_static().a, 2);

    assert!(data.exists_managed());
    assert!(!Weak::<Data>::default().exists_managed());
}
