//! Messaging

use crate::thread::Thread;
use crate::path::{Interface};
use alloc::rc::Rc;
use crate::rsc::{Handle, Constant};
use core::ops::Deref;
use crate::Uid;
use core::marker::PhantomData;

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

/// Information about received message and the message itself.
pub struct Envelop {
    /// Interface for this message.
    interface: Rc<Interface>,

    /// Sender (source) of the message.
    source: Rc<Thread>,
}

pub struct Received<T: Copy + Send> {
    source: Rc<Thread>,
    msg: Message<T>,
}

impl Envelop {
    /// Accept the envelop and read the message as given type.
    ///
    /// # Safety
    /// Receiver must guarantee the chosen type corresponds to the interface stated
    /// in the envelop. Also, there are no guarantee that sender used the correct type
    /// and maintained type consistency. For complex types it is best to have trusted
    /// message sources.
    pub unsafe fn accept<T: Send + Copy>(self) -> Received<T> {
        unimplemented!()
    }
}

/// Channel makes simpler to maintain intensive communication between threads.
pub struct Pipe<T: Send + Copy> {
    destination: Rc<Thread>,
    interface: Rc<Interface>,
    _msg_type: PhantomData<Message<T>>,
}

impl<T: Send + Copy> Pipe<T> {
    pub fn receive(&self) -> Option<Message<T>> {
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

/// Receive pending envelops. The envelops are received in FIFO order thus the oldest message
/// gets read first.
///
/// None is returned if mailbox is empty.
pub fn receive() -> Option<Envelop> {
    unimplemented!()
}

/// Whether the mailbox is empty.
pub fn is_empty() -> bool {
    unimplemented!()
}

// TODO remove functional style in favor of OOP.
/// Received the message from given thread if any.
pub fn receive_from(thread: &Thread) -> Option<Envelop> {
    unimplemented!()
}
