use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};
use core::borrow::{Borrow, BorrowMut};
use crate::{kobzar_env, PrivateKobzarEnv, Uid, KobzarEnv};
use core::cmp::Ordering;

pub type Darc<T> = Arc<DeallocHandle<T>>;

pub type Sarc<T> = Arc<Snapshot<T>>;

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
        kobzar_env().release_info_resource(self.uid);
    }
}

/// Handle of variable metadata that may be externally changed. Contains snapshot of the data
/// at the time of creation of the handle or handle update.
#[derive(Clone)]
pub struct Snapshot<T: Clone> {
    uid: Uid,
    t: T,
}

impl<T> Snapshot<T> where T: Clone {
    /// Update information in the snapshot.
    pub fn update(&mut self) {
        *self = self.download_latest();
    }

    pub fn download_latest(&self) -> Self {
        kobzar_env().download_new_snapshot(self)
    }
}
impl<T> Deref for Snapshot<T> where T: Clone {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for Snapshot<T> where T: Clone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T> Borrow<T> for Snapshot<T> where T: Clone {
    fn borrow(&self) -> &T {
        &self.t
    }
}

impl<T> BorrowMut<T> for Snapshot<T> where T: Clone {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.t
    }
}

impl<T> PartialEq for Snapshot<T> where T: PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool {
        if self.uid == other.uid {
            true
        } else {
            self.t == other.t
        }
    }
}

impl<T> Eq for Snapshot<T> where T: Eq + Clone {}

impl<T> PartialOrd for Snapshot<T> where T: PartialOrd + Clone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T> Ord for Snapshot<T> where T: Ord + Clone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl<T> Drop for Snapshot<T> where T: Clone {
    fn drop(&mut self) {
        kobzar_env().release_info_resource(self.uid);
    }
}
