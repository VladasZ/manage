#[macro_export]
macro_rules! managed {
    ($type:ident) => {
        static _MANAGED_ROOT_PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
        static _STORAGE: std::sync::OnceLock<std::sync::Mutex<manage::DataStorage<$type>>> =
            std::sync::OnceLock::new();

        impl manage::Managed for $type {}

        impl manage::data_manager::DataManager<$type> for $type {
            fn root_path() -> &'static std::path::Path {
                _MANAGED_ROOT_PATH.get().expect(&format!(
                    "Managed root path for type {} is not set.",
                    stringify!($type)
                ))
            }

            fn set_root_path(path: impl Into<std::path::PathBuf>) {
                let path = path.into();
                let res = _MANAGED_ROOT_PATH.set(path.to_path_buf());
                if let Err(err) = res {
                    log::warn!(
                        "Managed root path for type {} was already set set.",
                        stringify!($type)
                    )
                }
            }

            fn storage() -> std::sync::MutexGuard<'static, manage::DataStorage<$type>> {
                _STORAGE
                    .get_or_init(|| std::sync::Mutex::new(Default::default()))
                    .lock()
                    .unwrap()
            }
        }
    };
}
