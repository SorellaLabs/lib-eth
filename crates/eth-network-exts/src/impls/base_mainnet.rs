use std::marker::PhantomData;

use base_common_network::Base;
use base_node_core::BaseNode;

use crate::{AllExtensions, EthNetworkExt};

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseMainnetExt<Extension = ()>(PhantomData<Extension>);

impl<Extension: AllExtensions> EthNetworkExt for BaseMainnetExt<Extension> {
    type AlloyNetwork = Base;
    type RethNode = BaseNode;
    type TypeExt = Extension;

    const CHAIN_ID: u64 = 8453;
}
