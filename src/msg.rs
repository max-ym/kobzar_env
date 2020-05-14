//! Messaging

use crate::thread::Thread;
use crate::path::{Interface};
use alloc::rc::Rc;
use crate::rsc::{Handle, Constant};
use core::ops::Deref;
use crate::Uid;

pub enum SendError {
    /// Receiver does not implement interface for which this message was formed.
    InterfaceNotImplemented,
}

pub enum ShareError {
    /// Receiver does not implement interface for which this message was formed.
    InterfaceNotImplemented,
}

#[derive(Clone)]
pub struct Message<T: Send + Copy> {
    msg: T,
    interface: Rc<Interface>,
}

impl<T> Message<T> where T: Send + Copy {
    /// Copy message into destination thread's memory.
    pub fn copy_to(&self, to: &Thread) -> Result<(), SendError> {
        unimplemented!()
    }
}

impl Message<()> {
    /// Create new signal. Signal is a message with no body. It just signals the receiver
    /// by the interface.
    pub fn new_signal(interface: Rc<Interface>) -> Self {
        Message {
            msg: (),
            interface,
        }
    }
}

#[derive(Clone)]
pub struct SharedMessage<T: Send + Copy> {
    msg: Message<T>,
    uid: Uid,
}

impl<T> Handle for SharedMessage<T> where T: Send + Copy {
    fn uid(&self) -> Uid {
        self.uid
    }
}

impl<T> Constant for SharedMessage<T> where T: Send + Copy {}

impl<T> Deref for SharedMessage<T> where T: Send + Copy {
    type Target = Message<T>;

    fn deref(&self) -> &Self::Target {
        &self.msg
    }
}

impl<T> SharedMessage<T> where T: Send + Copy {
    pub fn share_with(&self, with: &Thread) -> Result<(), ShareError> {
        unimplemented!()
    }
}
