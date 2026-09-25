#![allow(ambiguous_glob_reexports)]

#[cfg(feature = "reth-db")]
pub mod reth_libmdbx;

pub mod rpc;
pub mod traits;

#[cfg(feature = "base-reth-db")]
pub mod base_reth {
    use std::sync::{Arc, LazyLock};

    pub use base_common_consensus::*;
    pub use base_execution_chainspec::*;
    pub use base_node_core::*;

    /// The Base mainnet spec.
    pub static BASE_MAINNET: LazyLock<Arc<BaseChainSpec>> = LazyLock::new(|| Arc::new(BaseChainSpec::mainnet()));
}

// alias kept so downstream users of `lib_reth::op_reth` build unchanged
#[cfg(feature = "base-reth-db")]
pub use base_reth as op_reth;
#[cfg(feature = "reth-db")]
pub use regular_reth::*;

#[cfg(feature = "reth-db")]
mod regular_reth {
    pub use reth_chainspec::*;
    pub use reth_node_ethereum::EthereumNode;
    pub use reth_node_types::NodeTypes;
    pub use reth_rpc_eth_api::*;
    pub use reth_storage_api::*;
    pub use revm::*;
}

#[cfg(test)]
pub mod test_utils;
