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

use big_unsigned_ints::U256;
use core::num::NonZeroU16;
use crate::harc::Harc;
use smallvec::SmallVec;
use crate::{kobzar_env, KobzarEnv};
use crate::thread::{ThreadBuilder, OwnedThread, ThreadBuildError};

/// Unique identifier of the thread instance inside of the network. First field
/// indicates its ID and second is used as duplicate marker which is assigned non-zero value
/// in case duplicate of unique ID was found in the network which generally is almost
/// impossible.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Uid(pub U256, pub Option<NonZeroU16>);

/// Path of some resource. It consists of up to 8 nodes. Path entry
/// contains only pointers to string slices and generally Clone operation is
/// cheap.
#[derive(Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    nodes: [&'static str; 8],
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

impl<'a> From<&'a Path> for LocalPath<'a> {
    fn from(p: &'a Path) -> Self {
        LocalPath {
            nodes: p.nodes,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LocalPath<'a> {
    nodes: [&'a str; 8],
}

/// Version of the app in form `major.minor.patch`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32, u32, u32);

#[derive(Clone, PartialEq)]
pub struct InstanceId {
    path: Path,
    version: Version,
    uid: Uid,
}

impl InstanceId {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn uid(&self) -> Uid {
        self.uid
    }
}

// pub struct PackageInstances(SmallVec<[Harc<InstanceId>; 16]>);
//
// impl Deref for PackageInstances {
//     type Target = SmallVec<[Harc<InstanceId>; 16]>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl DerefMut for PackageInstances {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
//
// impl PackageInstances {}

pub struct FindInstanceRequest<'a> {
    path: LocalPath<'a>,
    version: Option<Version>,
}

impl<'a> FindInstanceRequest<'a> {
    pub fn new(path: LocalPath<'a>) -> Self {
        FindInstanceRequest {
            path,
            version: None,
        }
    }

    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn find(&self) -> SmallVec<[Harc<InstanceId>; 16]> {
        kobzar_env().network().find_package_instances(self)
    }

    pub fn path(&self) -> &LocalPath {
        &self.path
    }

    pub fn version(&self) -> &Option<Version> {
        &self.version
    }
}

pub trait Network {
    /// Find instances that have this package name.
    fn find_package_instances(&self, find: &FindInstanceRequest)
                              -> SmallVec<[Harc<InstanceId>; 16]>;
    
    fn create_thread(&mut self, t: &ThreadBuilder) -> Result<OwnedThread, ThreadBuildError>;
}
