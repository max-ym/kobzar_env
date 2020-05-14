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

#[derive(Clone)]
pub struct Message<T: Send + Copy> {
    msg: T,
    interface: Rc<Interface>,
}

impl<T> Message<T> where T: Send + Copy {
    /// Copy message into destination thread's memory.
    pub fn send_to(&self, to: &Thread) -> Result<(), SendError> {
        unimplemented!()
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
