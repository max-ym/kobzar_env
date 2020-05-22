//! Each resource in the system has it's own path and unique identifier. Path is used
//! to identify the resource as a class with similar properties whereas ID identify each
//! instance of it. ID is big enough to be used in complex systems with multiple computers with
//! very little chance to have overlapping IDs. In this case if one system has the same ID
//! each instance will be assigned additional 'duplicate` ID to separate instances.
//! All other systems in the network will get notified about ID update. This case is extremely
//! rare and most likely will never occur.
//!
//! Due to different policies individual systems in the network may have copy of all
//! available resources on each system or may have data about only
//! interesting resources
//! or only resources currently in use.

use smallvec::SmallVec;
use crate::{kobzar_env, KobzarEnv};
use crate::thread::{ThreadBuilder, OwnedThread, ThreadBuildError, PerformancePolicy};
use alloc::sync::Arc;
use alloc::rc::Rc;
use core::time::Duration;
use crate::msg::{Receiver, Output, Sender, Input, SendError, ReceiveError};
use core::ops::Range;
use arrayvec::ArrayVec;

type NodeVec<'a> = ArrayVec<[&'a str; 8]>;

/// Unique identifier of the object inside of the network. These include threads and interfaces.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Uid(pub u64);

/// Path of some resource. It consists of up to 8 nodes. Path entry
/// contains only pointers to string slices and generally Clone operation is
/// cheap.
///
/// This path is never created by developer and instead it is always provided by
/// the network to describe existing resources.
// TODO impl node types.
#[derive(Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    nodes: NodeVec<'static>,
}

impl Path {
    pub fn nodes(&self) -> &NodeVec<'static> {
        &self.nodes
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.nodes.len() {
            if self.nodes[i] != other.nodes[i] {
                return false;
            }
        }
        true
    }
}

impl<'a> From<&'a Path> for LocalPath<'static> {
    fn from(p: &'a Path) -> Self {
        LocalPath {
            nodes: p.nodes.clone(),
        }
    }
}

/// Local path is a path constructed from local str slices. It is functionally the same as
/// [Path].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LocalPath<'a> {
    nodes: NodeVec<'a>,
}

impl<'a> LocalPath<'a> {
    pub fn nodes(&self) -> &NodeVec<'a> {
        &self.nodes
    }

    pub fn new(nodes: NodeVec<'a>) -> Self {
        LocalPath {
            nodes,
        }
    }
}

/// Version of the app in form `major.minor.patch`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub u32, pub u32, pub u32);

/// Interface instance. Thread ID that is an implementor of selected interface.
#[derive(Clone, PartialEq)]
pub struct InstanceId {
    interface: Rc<Interface>,
    uid: Uid,
}

impl InstanceId {
    /// Path of an interface that define given instance.
    pub fn path(&self) -> &Path {
        &self.interface.path
    }

    /// Version of the interface of this instance.
    pub fn version(&self) -> Version {
        self.interface.version
    }

    /// UID of the instance.
    pub fn uid(&self) -> Uid {
        self.uid
    }
}

/// Interface defines a function that should be provided by the server that implements it.
/// Each thread implements set of interfaces. At least, thread implements an interface that
/// declares this thread type so it can be instantiated.
#[derive(Clone, PartialEq)]
pub struct Interface {
    path: Path,
    version: Version,
}

impl Interface {
    /// Path of given interface.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Version of given interface.
    pub fn version(&self) -> Version {
        self.version
    }
}

/// Request to find instances of given interface with specific version.
pub struct FindInstanceRequest<'a> {
    path: LocalPath<'a>,
    version: Option<Range<Version>>,
}

impl<'a> FindInstanceRequest<'a> {
    /// Create new request to search for interface with given path.
    pub fn new(path: LocalPath<'a>) -> Self {
        FindInstanceRequest {
            path,
            version: None,
        }
    }

    /// Search for implementers that match version constraints.
    pub fn with_version(mut self, version: Range<Version>) -> Self {
        self.version = Some(version);
        self
    }

    /// Execute request and find all implementers.
    pub fn find(&self) -> SmallVec<[Arc<InstanceId>; 16]> {
        kobzar_env().network().find_package_instances(self)
    }

    /// Path of the interface.
    pub fn path(&self) -> &LocalPath<'a> {
        &self.path
    }

    /// Version constraints of the interface.
    pub fn version(&self) -> &Option<Range<Version>> {
        &self.version
    }
}

pub(crate) trait Network {
    /// Find instances that have this package name.
    fn find_package_instances(&self, find: &FindInstanceRequest)
                              -> SmallVec<[Arc<InstanceId>; 16]>;

    fn create_thread(&self, t: &ThreadBuilder) -> Result<OwnedThread, ThreadBuildError>;

    fn allow_run(&self, t: &OwnedThread);

    fn request_pause(&self, t: &OwnedThread);

    fn request_cease(&self, t: &OwnedThread);

    fn brutal_kill(&self, t: &OwnedThread) -> Result<(), ()>;

    fn sleep(&self, t: &OwnedThread, duration: Duration);

    fn set_performance_policy(&self, t: &OwnedThread, policy: PerformancePolicy)
                              -> Result<(), PerformancePolicy>;

    fn current_thread(&self) -> &'static mut OwnedThread;

    fn send<O: Output>(&self, sender: &Sender<O>, msg: &O) -> Result<(), SendError>;

    fn rendezvous<O: Output>(&self, sender: &Sender<O>, msg: &O) -> Result<(), SendError>;

    fn rendezvous_for<O: Output>(&self, sender: &Sender<O>, msg: &O, duration: Duration)
                                 -> Result<Option<()>, SendError>;

    fn recv<I: Input>(&self, recv: &Receiver<I>) -> Result<Option<I>, ReceiveError>;

    fn recv_sync<I: Input>(&self, recv: &Receiver<I>) -> Result<I, ReceiveError>;

    fn recv_sync_for<I: Input>(&self, recv: &Receiver<I>, duration: Duration)
                               -> Result<Option<I>, ReceiveError>;

    unsafe fn new_receiver<I: Input>(&self, interface: &Rc<Interface>) -> Option<Receiver<I>>;

    unsafe fn new_receiver_sync<I: Input>(&self, interface: &Rc<Interface>) -> Receiver<I>;

    unsafe fn new_receiver_sync_for<I: Input>(&self, time: Duration, int: &Rc<Interface>)
                                              -> Option<Receiver<I>>;

    fn has_incoming(&self) -> bool;

    fn wait_any<'a>(&self, interfaces: impl Iterator<Item=&'a Interface>);

    fn wait_any_for<'a>(&self, wait: Duration, interfaces: impl Iterator<Item=&'a Interface>)
                        -> Option<()>;
}
