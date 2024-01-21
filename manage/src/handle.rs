use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::Managed;

pub struct Handle<T: Managed> {
    hash:  u64,
    _data: PhantomData<T>,
}

impl<T: Managed> Handle<T> {
    pub fn is_null(&self) -> bool {
        self.hash == u64::MAX
    }

    pub fn is_ok(&self) -> bool {
        !self.is_null()
    }

    pub fn get(&self) -> Option<&T> {
        if self.is_null() {
            return None;
        }
        T::get_ref_by_hash(self.hash).into()
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn free(&mut self) {
        T::remove_with_hash(self.hash);
        *self = Self::default();
    }
}

impl<T: Managed> Deref for Handle<T> {
    type Target = T;
    fn deref(&self) -> &T {
        assert!(self.is_ok(), "Null Handle: {}", std::any::type_name::<T>());
        T::get_ref_by_hash(self.hash)
    }
}

impl<T: Managed> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(self.is_ok(), "Null Handle: {}", std::any::type_name::<T>());
        T::get_ref_by_hash_mut(self.hash)
    }
}

impl<T: Managed> Default for Handle<T> {
    fn default() -> Self {
        Self {
            hash:  u64::MAX,
            _data: Default::default(),
        }
    }
}

impl<T: Managed> From<u64> for Handle<T> {
    fn from(hash: u64) -> Self {
        Self {
            hash,
            _data: Default::default(),
        }
    }
}

impl<T: Managed> Copy for Handle<T> {}

impl<T: Managed> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Debug + Managed> Debug for Handle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
