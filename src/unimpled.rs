#![allow(dead_code)]
use crate::{KobzarEnv, PrivateKobzarEnv, Uid};
use crate::path::{Network, FindInstanceRequest, InstanceId, Interface};
use crate::rsc::{Variable};
use smallvec::SmallVec;
use crate::thread::{OwnedThread, ThreadBuildError, ThreadBuilder, PerformancePolicy};
use alloc::sync::Arc;
use core::time::Duration;
use crate::msg::{Sender, Receiver, ReceiveError, Output, Input, SendError, MailboxSendError};
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

pub(crate) fn kobzar_env() -> &'static mut UnimplementedEnv {
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

    fn allow_run(&self, _: &OwnedThread) {
        unimplemented!()
    }

    fn request_pause(&self, _: &OwnedThread) {
        unimplemented!()
    }

    fn request_cease(&self, _: &OwnedThread) {
        unimplemented!()
    }

    fn brutal_kill(&self, _: &OwnedThread) -> Result<(), ()> {
        unimplemented!()
    }

    fn sleep(&self, _: &OwnedThread, _: Duration) {
        unimplemented!()
    }

    fn set_performance_policy(&self, _: &OwnedThread, _: PerformancePolicy)
                              -> Result<(), PerformancePolicy> {
        unimplemented!()
    }

    fn current_thread(&self) -> &'static mut OwnedThread {
        unimplemented!()
    }

    fn send<O: Output>(&self, _: &Sender<O>, _: &O) -> Result<(), MailboxSendError> {
        unimplemented!()
    }

    fn send_when_available<O: Output>(&self, _: &Sender<O>, _: &O) -> Result<(), SendError> {
        unimplemented!()
    }

    fn transfer_time<O: Output>(&self, _: &Sender<O>) -> Result<(), SendError> {
        unimplemented!()
    }

    fn rendezvous<O: Output>(&self, _: &Sender<O>, _: &O) -> Result<(), SendError> {
        unimplemented!()
    }

    fn rendezvous_for<O: Output>(&self, _: &Sender<O>, _: &O, _: Duration)
                                 -> Result<Option<()>, SendError> {
        unimplemented!()
    }

    fn recv<I: Input>(&self, _: &Receiver<I>) -> Result<Option<I>, ReceiveError> {
        unimplemented!()
    }

    fn recv_sync<I: Input>(&self, _: &Receiver<I>) -> Result<I, ReceiveError> {
        unimplemented!()
    }

    fn recv_sync_for<I: Input>(&self, _: &Receiver<I>, _: Duration) -> Result<Option<I>, ReceiveError> {
        unimplemented!()
    }

    unsafe fn new_receiver<I: Input>(&self, _: &Rc<Interface>) -> Option<Receiver<I>> {
        unimplemented!()
    }

    unsafe fn new_receiver_sync<I: Input>(&self, _: &Rc<Interface>) -> Receiver<I> {
        unimplemented!()
    }

    unsafe fn new_receiver_sync_for<I: Input>(&self, time: Duration, int: &Rc<Interface>)
                                              -> Option<Receiver<I>> {
        unimplemented!()
    }

    fn has_incoming(&self) -> bool {
        unimplemented!()
    }

    fn wait_any<'a>(&self, _: impl Iterator<Item=&'a Interface>) {
        unimplemented!()
    }

    fn wait_any_for<'a>(&self, _: Duration, _: impl Iterator<Item=&'a Interface>) -> Option<()> {
        unimplemented!()
    }
}
