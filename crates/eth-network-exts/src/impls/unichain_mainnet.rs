use std::marker::PhantomData;

use base_common_network::Base;
use base_node_core::BaseNode;

use crate::{AllExtensions, EthNetworkExt};

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnichainMainnetExt<Extension = ()>(PhantomData<Extension>);

// Unichain is an OP Stack chain, so Base's types (an OP Stack fork) stand in
// for the removed Optimism crates.
impl<Extension: AllExtensions> EthNetworkExt for UnichainMainnetExt<Extension> {
    type AlloyNetwork = Base;
    type RethNode = BaseNode;
    type TypeExt = Extension;

    const CHAIN_ID: u64 = 130;
}
