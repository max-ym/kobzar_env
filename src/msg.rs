//! Messaging

use core::marker::PhantomData;
use alloc::rc::Rc;
use crate::thread::Thread;
use crate::path::{Interface, Network};
use core::time::Duration;
use crate::{kobzar_env, KobzarEnv};

/// Receiver is used to receive messages from other selected thread by selected interface that
/// is supported by the sending thread. Receiver has exact input type which corresponds to
/// the one defined by the interface.
pub struct Receiver<I: Input> {
    src: Rc<Thread>,
    interface: Rc<Interface>,
    _input: PhantomData<I>,
}

/// Sender is used to send messages to some defined destination thread by selected interface
/// that is supported by receiving thread. Sender has exact input type which corresponds to
/// the one defined by the interface and this type is expected by receiver.
pub struct Sender<O: Output> {
    dest: Rc<Thread>,
    interface: Rc<Interface>,
    _output: PhantomData<O>,
}

/// Message that can be received.
pub trait Input: Sized {
    /// Interface that is used in communication.
    fn interface() -> &'static Rc<Interface>;

    /// Create an receiver for the new mail in the mailbox of given type. If no mail
    /// was found None is returned.
    fn get() -> Option<Receiver<Self>> {
        unsafe { Receiver::new(Self::interface()) }
    }

    /// Create an receiver for the new mail in the mailbox of given type. If no mail was found
    /// then wait until one arrives indefinitely.
    fn get_sync() -> Receiver<Self> {
        unsafe { Receiver::new_sync(Self::interface()) }
    }

    /// Create an receiver for the new mail in the mailbox of given type. If no mail was found
    /// then wait until one arrives for given amount of time. None is returned if time
    /// elapses.
    unsafe fn get_sync_for(time: Duration) -> Option<Receiver<Self>> {
        Receiver::new_sync_for(time, Self::interface())
    }

    /// Construct object from bytes received from the Receiver.
    fn from_msg_bytes(b: &[u8]) -> Self;
}

/// Data that can be sent by the Sender.
pub trait Output {
    /// Bytes that will be sent by the Sender.
    fn as_msg_bytes(&self) -> &[u8];
}

/// Error encountered on receiving.
pub enum ReceiveError {
    /// Received has died.
    Died,

    /// Connection with this thread was lost.
    ConnectionLost,

    /// Interface used to communicate is not supported by the receiver.
    Unsupported,
}

/// Error encountered on sending.
pub enum SendError {
    /// Received has died.
    Died,

    /// Connection with this thread was lost.
    ConnectionLost,
}

pub enum MailboxSendError {
    /// Normal send error.
    Send(SendError),

    /// Mailbox already has some mail pending. Mailbox can contain only one mail from
    /// the same sender type.
    Pending,
}

impl<O: Output> Sender<O> {
    /// Send message into mailbox. Note that this does not guarantee that the message
    /// will be received. Receiver may also discard the message or cease without reading.
    /// This method does not block and message will be buffered in current Pipe output buffer.
    /// If pipe has no buffer that this is the same as [`rendezvous`] method.
    pub fn send(&self, msg: &O) -> Result<(), MailboxSendError> {
        kobzar_env().network().send(self, msg)
    }

    /// The same as [send] but waits until mailbox is available. If any mail is pending this
    /// function will block the thread.
    pub fn send_when_available(&self, msg: &O) -> Result<(), SendError> {
        kobzar_env().network().send_when_available(self, msg)
    }

    /// Send it's remaining processor time.
    /// This will only work for threads on the same Computing Unit. In the case
    /// send is possible the receiving thread will immediately resume its execution. It can send
    /// remaining time back again. This can be used, for example, for calling memory manager
    /// to allocate memory without waiting for it to have its turn for scheduling.
    pub fn transfer_time(&self) -> Result<(), SendError> {
        kobzar_env().network().transfer_time(self)
    }

    /// Send the message by making rendezvous with the receiver. This makes a guarantee that
    /// receiver actually acquired the message but it still does not guarantee that the
    /// message was read as receiver can discard it. This method will execute
    /// as soon as all previous messages will get received.
    pub fn rendezvous(&self, msg: &O) -> Result<(), SendError> {
        kobzar_env().network().rendezvous(self, msg)
    }

    pub fn rendezvous_for(&self, msg: &O, duration: Duration) -> Result<Option<()>, SendError> {
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

    pub unsafe fn new_sync_for(time: Duration, interface: &Rc<Interface>) -> Option<Self> {
        kobzar_env().network().new_receiver_sync_for(time, interface)
    }

    /// The same as [new] but waits until given interface mail is received.
    pub unsafe fn new_sync(interface: &Rc<Interface>) -> Self {
        kobzar_env().network().new_receiver_sync(interface)
    }

    pub fn recv(&self) -> Result<Option<I>, ReceiveError> {
        kobzar_env().network().recv(self)
    }

    pub fn recv_sync(&self) -> Result<I, ReceiveError> {
        kobzar_env().network().recv_sync(self)
    }

    pub fn recv_sync_for(&self, wait: Duration) -> Result<Option<I>, ReceiveError> {
        kobzar_env().network().recv_sync_for(self, wait)
    }
}

/// Whether thread has any unread messages in the mailbox.
pub fn has_incoming() -> bool {
    kobzar_env().network().has_incoming()
}

/// Wait for any message by given interface from any thread indefinitely.
pub fn wait_any<'a>(interfaces: impl Iterator<Item=&'a Interface>) {
    kobzar_env().network().wait_any(interfaces)
}

/// Wait for any message by given interface from any thread fot given time.
pub fn wait_any_for<'a>(wait: Duration, interfaces: impl Iterator<Item=&'a Interface>)
                        -> Option<()> {
    kobzar_env().network().wait_any_for(wait, interfaces)
}
