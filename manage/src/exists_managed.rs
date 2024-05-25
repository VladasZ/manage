use refs::Weak;

use crate::Managed;

pub trait ExistsManaged {
    fn exists_managed(&self) -> bool;
}

impl<T: Managed> ExistsManaged for Weak<T> {
    fn exists_managed(&self) -> bool {
        unsafe { self.was_initialized() }
    }
}
