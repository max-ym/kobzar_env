use crate::{kobzar_env, KobzarEnv, Uid};

pub trait Handle {
    fn uid(&self) -> Uid;
}

/// Handle to a global data that has constant nature (opposite to Snapshot).
pub trait Constant: Handle {
}

/// Data that are variable system-wide and application should not assume it to be
/// correct all the time. Instead - it is correct for a time when the data was
/// downloaded. If newer information is needed - application should
/// manually download the newest one.
pub trait Variable: Clone + Handle {
    /// Update information in the snapshot.
    fn update(&mut self) {
        *self = self.download_latest();
    }

    fn download_latest(&self) -> Self {
        kobzar_env().download_new_snapshot(self)
    }
}
