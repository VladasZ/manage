#[macro_export]
macro_rules! managed {
    ($type:ident) => {
        static _MANAGED_ROOT_PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
        static mut STORAGE: *mut rtools::data_manager::DataStorage<$type> = std::ptr::null_mut();

        impl rtools::data_manager::Managed for $type {}

        impl rtools::data_manager::DataManager<$type> for $type {
            fn root_path() -> &'static std::path::Path {
                _MANAGED_ROOT_PATH.get().expect(&format!(
                    "Managed root path for type {} is not set.",
                    stringify!($type)
                ))
            }

            fn set_root_path(path: &std::path::Path) {
                let res = _MANAGED_ROOT_PATH.set(path.to_path_buf());
                if let Err(err) = res {
                    log::warn!(
                        "Managed root path for type {} was already set set.",
                        stringify!($type)
                    )
                }
            }

            fn storage() -> &'static mut rtools::data_manager::DataStorage<$type> {
                unsafe {
                    if STORAGE.is_null() {
                        STORAGE = Box::into_raw(Box::new(Default::default()));
                    }
                    STORAGE.as_mut().unwrap()
                }
            }
        }
    };
}
