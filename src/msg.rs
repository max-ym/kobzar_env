//! Messaging

use core::marker::PhantomData;
use alloc::rc::Rc;
use crate::thread::Thread;
use crate::path::{Interface, Network};
use core::time::Duration;
use crate::{kobzar_env, KobzarEnv};

pub struct Receiver<I: Input> {
    src: Rc<Thread>,
    interface: Rc<Interface>,
    _input: PhantomData<I>,
}

pub struct Sender<O: Output> {
    dest: Rc<Thread>,
    interface: Rc<Interface>,
    _output: PhantomData<O>,
}

/// Message that can be sent through the pipe.
pub trait Input: Sized {
    fn interface() -> &'static Rc<Interface>;

    fn get() -> Option<Receiver<Self>> {
        unsafe { Receiver::new(Self::interface()) }
    }

    fn get_sync() -> Receiver<Self> {
        unsafe { Receiver::new_sync(Self::interface()) }
    }

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

    /// Interface used to communicate is not supported by receiver.
    Unsupported,
}

impl<O: Output> Sender<O> {
    /// Send message into mailbox. Note that this does not guarantee that the message
    /// will be received. Receiver may also discard the message or cease without reading.
    /// This method does not block and message will be buffered in current Pipe output buffer.
    /// If pipe has no buffer that this is the same as [`rendezvous`] method.
    ///
    /// Error will occur if thread is no longer alive or connection is lost.
    pub fn send(&self, msg: &O) -> Result<(), Error> {
        kobzar_env().network().send(self, msg)
    }

    /// Send the message by making rendezvous with the receiver. This makes a guarantee that
    /// receiver actually acquired the message but it still does not guarantee that the
    /// message was read as receiver can discard it. This method will execute
    /// as soon as all previous messages will get received.
    pub fn rendezvous(&self, msg: &O) -> Result<(), Error> {
        kobzar_env().network().rendezvous(self, msg)
    }

    pub fn rendezvous_for(&self, msg: &O, duration: Duration) -> Result<Option<()>, Error> {
        kobzar_env().network().rendezvous_for(self, msg, duration)
    }
}

impl<I: Input> Receiver<I> {
    /// Try creating receiver for given interface. It will return None if mailbox has no
    /// mail that matches interface.
    ///
    /// # Safety
    /// Input type cannot be verified when receiving message and thus if an input type does not
    /// match the one interface requires undefined behaviour will take place.
    pub unsafe fn new(interface: &Rc<Interface>) -> Option<Self> {
        kobzar_env().network().new_receiver(interface)
    }

    /// The same as [new] but waits until given interface mail is received.
    pub unsafe fn new_sync(interface: &Rc<Interface>) -> Self {
        kobzar_env().network().new_receiver_sync(interface)
    }

    pub fn recv(&self) -> Result<Option<I>, Error> {
        kobzar_env().network().recv(self)
    }

    pub fn recv_sync(&self) -> Result<I, Error> {
        kobzar_env().network().recv_sync(self)
    }

    pub fn recv_sync_for(&self, wait: Duration) -> Result<Option<I>, Error> {
        kobzar_env().network().recv_sync_for(self, wait)
    }
}

pub fn has_incoming() -> bool {
    kobzar_env().network().has_incoming()
}

pub fn wait_any<'a>(interfaces: impl Iterator<Item=&'a Interface>) {
    kobzar_env().network().wait_any(interfaces)
}

pub fn wait_any_for<'a>(interfaces: impl Iterator<Item=&'a Interface>, wait: Duration)
                        -> Option<()> {
    kobzar_env().network().wait_any_for(interfaces, wait)
}
