use alloy_primitives::{Address, B256, ChainId, address, b256};
use alloy_sol_types::Eip712Domain;

use crate::v4::{UNISWAP_V4_CONSTANTS_MAINNET, UNISWAP_V4_CONSTANTS_SEPOLIA_TESTNET, UniswapV4Constants};

#[derive(Debug, Clone, Copy)]
pub struct AngstromL1Constants {
    angstrom_address:         Address,
    controller_v1_address:    Address,
    angstrom_adapter_address: Address,
    gas_token_address:        Address,
    angstrom_deploy_block:    u64,
    chain_id:                 u64,
    uniswap_constants:        UniswapV4Constants,
    angstrom_mainnet_pools:   Option<AngstromMainnetStaticPools>,
    angstrom_node_addresses:  Option<AngstromNodeAddresses>
}

impl AngstromL1Constants {
    pub const fn by_chain(chain_id: ChainId) -> Option<Self> {
        match chain_id {
            1 => Some(ANGSTROM_L1_CONSTANTS_MAINNET),
            11155111 => Some(ANGSTROM_L1_CONSTANTS_SEPOLIA_TESTNET),
            _ => None
        }
    }

    #[inline]
    pub const fn angstrom_address(&self) -> Address {
        self.angstrom_address
    }

    #[inline]
    pub const fn controller_v1_address(&self) -> Address {
        self.controller_v1_address
    }

    #[inline]
    pub const fn angstrom_adapter_address(&self) -> Address {
        self.angstrom_adapter_address
    }

    #[inline]
    pub const fn gas_token_address(&self) -> Address {
        self.gas_token_address
    }

    #[inline]
    pub const fn angstrom_deploy_block(&self) -> u64 {
        self.angstrom_deploy_block
    }

    #[inline]
    pub const fn chain_id(&self) -> u64 {
        self.chain_id
    }

    #[inline]
    pub const fn angstrom_eip712_domain(&self) -> Eip712Domain {
        alloy_sol_types::eip712_domain!(
            name: "Angstrom",
            version: "v1",
            chain_id: self.chain_id,
            verifying_contract:self.angstrom_address,
        )
    }

    #[inline]
    pub const fn uniswap_constants(&self) -> UniswapV4Constants {
        self.uniswap_constants
    }

    #[inline]
    pub const fn angstrom_mainnet_pools(&self) -> Option<AngstromMainnetStaticPools> {
        self.angstrom_mainnet_pools
    }

    #[inline]
    pub const fn angstrom_node_addresses(&self) -> Option<AngstromNodeAddresses> {
        self.angstrom_node_addresses
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AngstromMainnetStaticPools {
    pub usdc_weth_pool_id: B256,
    pub weth_usdt_pool_id: B256
}

impl AngstromMainnetStaticPools {
    pub const fn new_mainnet() -> Self {
        Self {
            usdc_weth_pool_id: b256!("0xe500210c7ea6bfd9f69dce044b09ef384ec2b34832f132baec3b418208e3a657"),
            weth_usdt_pool_id: b256!("0x90078845bceb849b171873cfbc92db8540e9c803ff57d9d21b1215ec158e79b3")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AngstromNodeAddresses {
    nodes: [Address; 3]
}

impl AngstromNodeAddresses {
    pub const fn new_mainnet() -> Self {
        Self {
            nodes: [
                address!("0xc917c3fa468f2c4b9c84c72caa46420eb9825249"),
                address!("0x2252f216f4a494a87025123425181ca1bb754fb8"),
                address!("0x5875db54cd9ae2b2a875e09bb731772297ae9d92")
            ]
        }
    }

    pub fn all_nodes(&self) -> Vec<Address> {
        self.nodes.to_vec()
    }

    /// index starts at 0
    pub fn get_node(&self, idx: usize) -> Address {
        if idx >= self.nodes.len() {
            panic!("invalid node index, max index is {}", self.nodes.len() - 1)
        }
        self.nodes[idx]
    }
}

pub const ANGSTROM_L1_CONSTANTS_MAINNET: AngstromL1Constants = AngstromL1Constants {
    angstrom_address:         address!("0x0000000aa232009084Bd71A5797d089AA4Edfad4"),
    controller_v1_address:    address!("0x1746484EA5e11C75e009252c102C8C33e0315fD4"),
    gas_token_address:        address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
    angstrom_deploy_block:    22971781,
    chain_id:                 1,
    uniswap_constants:        UNISWAP_V4_CONSTANTS_MAINNET,
    angstrom_adapter_address: address!("0xb535aEB27335B91e1B5bcCbd64888bA7574eFBF8"),
    angstrom_mainnet_pools:   Some(AngstromMainnetStaticPools::new_mainnet()),
    angstrom_node_addresses:  Some(AngstromNodeAddresses::new_mainnet())
};

pub const ANGSTROM_L1_CONSTANTS_SEPOLIA_TESTNET: AngstromL1Constants = AngstromL1Constants {
    angstrom_address:         address!("0x3B9172ef12bd245A07DA0d43dE29e09036626AFC"),
    controller_v1_address:    address!("0x977c67e6CEe5b5De090006E87ADaFc99Ebed2a7A"),
    gas_token_address:        address!("0xfff9976782d46cc05630d1f6ebab18b2324d6b14"),
    angstrom_deploy_block:    8578780,
    chain_id:                 11155111,
    uniswap_constants:        UNISWAP_V4_CONSTANTS_SEPOLIA_TESTNET,
    angstrom_adapter_address: address!("0xb535aEB27335B91e1B5bcCbd64888bA7574eFBF8"),
    angstrom_mainnet_pools:   None,
    angstrom_node_addresses:  None
};
