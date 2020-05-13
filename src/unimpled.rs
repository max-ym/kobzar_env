#![allow(dead_code)]
use crate::{KobzarEnv, PrivateKobzarEnv, Uid};
use crate::path::{Network, FindInstanceRequest, InstanceId};
use crate::harc::Darc;
use smallvec::SmallVec;

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
                              -> SmallVec<[Darc<InstanceId>; 16]> {
        unimplemented!()
    }
}
