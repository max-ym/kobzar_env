//! Messaging

use core::marker::PhantomData;
use alloc::rc::Rc;
use crate::thread::Thread;
use crate::path::Interface;
use core::time::Duration;

pub struct InputPipe<I: Input> {
    src: Rc<Thread>,
    interface: Rc<Interface>,
    _input: PhantomData<I>,
}

pub struct OutputPipe<O: Output> {
    dest: Rc<Thread>,
    interface: Rc<Interface>,
    _output: PhantomData<O>,
}

/// Message that can be sent through the pipe.
pub trait Input {
    /// Construct object from bytes received from the Receiver.
    fn from_msg_bytes(b: &[u8]) -> Self;
}

pub trait Output {
    /// Bytes that will be sent by the pipe.
    fn as_msg_bytes(&self) -> &[u8];
}

pub enum Error {
    /// Received has died.
    Died,

    /// Connection with this thread was lost.
    ConnectionLost,
}

impl<O: Output> OutputPipe<O> {
    /// Send message into mailbox. Note that this does not guarantee that the message
    /// will be received. Receiver may also discard the message or cease without reading.
    /// This method does not block and message will be buffered in current Pipe output buffer.
    /// If pipe has no buffer that this is the same as [`rendezvous`] method.
    ///
    /// Error will occur if thread is no longer alive or connection is lost.
    pub fn send(&self, msg: &O) -> Result<(), Error> {
        unimplemented!()
    }

    /// Send the message by making rendezvous with the receiver. This makes a guarantee that
    /// receiver actually acquired the message but it still does not guarantee that the
    /// message was read as receiver can discard it. This method will execute
    /// as soon as all previous messages will get received.
    pub fn rendezvous(&self, msg: &O) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn rendezvous_for(&self, msg: &O, duration: Duration) -> Result<Option<()>, Error> {
        unimplemented!()
    }
}

impl<I: Input> InputPipe<I> {
    pub fn recv(&self) -> Result<Option<I>, Error> {
        unimplemented!()
    }

    pub fn recv_sync(&self) -> Result<I, Error> {
        unimplemented!()
    }

    pub fn recv_sync_for(&self, duration: Duration) -> Result<Option<I>, Error> {
        unimplemented!()
    }
}
