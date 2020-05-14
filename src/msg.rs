//! Messaging

use crate::thread::Thread;
use core::borrow::Borrow;
use crate::harc::Darc;
use crate::path::Interface;

pub enum SendError {
    /// Receiver does not implement interface for which this message was formed.
    InterfaceNotImplemented,
}

pub struct Message<T> {
    msg: T,
    interface: Darc<Interface>,
}

impl<T> Message<T> {
    pub fn send_to(self, to: impl Borrow<Thread>) -> Result<(), (Self, SendError)> {
        unimplemented!()
    }
}
