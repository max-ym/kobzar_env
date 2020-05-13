use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};
use core::borrow::{Borrow, BorrowMut};
use crate::{kobzar_env, PrivateKobzarEnv, Uid};
use core::cmp::Ordering;

pub type Harc<T> = Arc<DeallocHandle<T>>;

pub struct DeallocHandle<T> {
    uid: Uid,
    t: T,
}

impl<T> DeallocHandle<T> {
    pub fn kobzar_uid(&self) -> Uid {
        self.uid
    }
}

impl<T> Deref for DeallocHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for DeallocHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T> Borrow<T> for DeallocHandle<T> {
    fn borrow(&self) -> &T {
        &self.t
    }
}

impl<T> BorrowMut<T> for DeallocHandle<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.t
    }
}

impl<T> PartialEq for DeallocHandle<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.uid == other.uid {
            true
        } else {
            self.t == other.t
        }
    }
}

impl<T> Eq for DeallocHandle<T> where T: Eq {}

impl<T> PartialOrd for DeallocHandle<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T> Ord for DeallocHandle<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl<T> Drop for DeallocHandle<T> {
    fn drop(&mut self) {
        kobzar_env().release_harc_resource(self.uid);
    }
}
