#![no_std]
extern crate alloc;

pub mod thread;

pub use path::Uid;
use crate::path::Network;

pub mod path;

/// KobzarEnv resources mapped to app's memory.
mod rsc;

pub mod msg;

/// Unimplemented environment. Is used while developing library to check for compilation
/// errors in implementer-agnostic code.
mod unimpled;
pub use unimpled::*;
use crate::rsc::Variable;
use core::time::Duration;

trait KobzarEnv {
    type Network: Network;

    fn network(&self) -> &Self::Network;

    fn network_mut(&mut self) -> &mut Self::Network;

    /// Download latest updates for the data in the snapshot.
    fn download_new_snapshot<T: Variable>(&self, var: &T) -> T;
}

trait PrivateKobzarEnv: KobzarEnv {
    fn release_info_resource(&mut self, uid: Uid);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
