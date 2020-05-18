#![allow(dead_code)]
use crate::{KobzarEnv, PrivateKobzarEnv, Uid};
use crate::path::{Network, FindInstanceRequest, InstanceId, Interface};
use crate::rsc::{Variable};
use smallvec::SmallVec;
use crate::thread::{OwnedThread, ThreadBuildError, ThreadBuilder, PerformancePolicy};
use alloc::sync::Arc;
use core::time::Duration;
use crate::msg::{Sender, Receiver, Error, Output, Input};
use smallvec::alloc::rc::Rc;

pub struct UnimplementedEnv;
pub struct UnimplementedNetwork;

impl KobzarEnv for UnimplementedEnv {
    type Network = UnimplementedNetwork;

    fn network(&self) -> &Self::Network {
        unimplemented!()
    }

    fn network_mut(&mut self) -> &mut Self::Network {
        unimplemented!()
    }

    fn download_new_snapshot<T: Variable>(&self, _: &T) -> T {
        unimplemented!()
    }
}

impl PrivateKobzarEnv for UnimplementedEnv {
    fn release_info_resource(&mut self, _: Uid) {
        unimplemented!()
    }
}

pub fn kobzar_env() -> &'static mut UnimplementedEnv {
    unimplemented!()
}

impl Network for UnimplementedNetwork {
    fn find_package_instances(&self, _: &FindInstanceRequest)
                              -> SmallVec<[Arc<InstanceId>; 16]> {
        unimplemented!()
    }

    fn create_thread(&self, _: &ThreadBuilder) -> Result<OwnedThread, ThreadBuildError> {
        unimplemented!()
    }

    fn allow_run(&self, t: &OwnedThread) {
        unimplemented!()
    }

    fn request_pause(&self, t: &OwnedThread) {
        unimplemented!()
    }

    fn request_cease(&self, t: &OwnedThread) {
        unimplemented!()
    }

    fn brutal_kill(&self, t: &OwnedThread) -> Result<(), ()> {
        unimplemented!()
    }

    fn sleep(&self, t: &OwnedThread, duration: Duration) {
        unimplemented!()
    }

    fn set_performance_policy(&self, t: &OwnedThread, policy: PerformancePolicy) -> Result<(), PerformancePolicy> {
        unimplemented!()
    }

    fn current_thread(&self) -> &'static mut OwnedThread {
        unimplemented!()
    }

    fn send<O: Output>(&self, sender: &Sender<O>, msg: &O) -> Result<(), Error> {
        unimplemented!()
    }

    fn rendezvous<O: Output>(&self, sender: &Sender<O>, msg: &O) -> Result<(), Error> {
        unimplemented!()
    }

    fn rendezvous_for<O: Output>(&self, sender: &Sender<O>, msg: &O, duration: Duration) -> Result<Option<()>, Error> {
        unimplemented!()
    }

    fn recv<I: Input>(&self, recv: &Receiver<I>) -> Result<Option<I>, Error> {
        unimplemented!()
    }

    fn recv_sync<I: Input>(&self, recv: &Receiver<I>) -> Result<I, Error> {
        unimplemented!()
    }

    fn recv_sync_for<I: Input>(&self, recv: &Receiver<I>, duration: Duration) -> Result<Option<I>, Error> {
        unimplemented!()
    }

    unsafe fn new_receiver<I: Input>(&self, interface: &Rc<Interface>) -> Option<Receiver<I>> {
        unimplemented!()
    }

    unsafe fn new_receiver_sync<I: Input>(&self, interface: &Rc<Interface>) -> Receiver<I> {
        unimplemented!()
    }

    fn has_incoming(&self) -> bool {
        unimplemented!()
    }

    fn wait_any<'a>(&self, interfaces: impl Iterator<Item=&'a Interface>) {
        unimplemented!()
    }

    fn wait_any_for<'a>(&self, interfaces: impl Iterator<Item=&'a Interface>, wait: Duration) -> Option<()> {
        unimplemented!()
    }
}
