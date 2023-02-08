#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_structs::*;
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthEvent,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethevent(name = "NewContract", abi = "NewContract(string,address)")]
pub struct NewContractFilter {
    #[ethevent(indexed)]
    pub name: ethers::core::types::H256,
    pub addr: ethers::core::types::Address,
}
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthEvent,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethevent(
    name = "UpdateBridgeWhitelist",
    abi = "UpdateBridgeWhitelist(uint256,address[],uint256[])"
)]
pub struct UpdateBridgeWhitelistFilter {
    #[ethevent(indexed)]
    pub nonce: ethers::core::types::U256,
    pub tokens: Vec<ethers::core::types::Address>,
    pub token_cap: Vec<ethers::core::types::U256>,
}
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthEvent,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethevent(name = "UpgradedContract", abi = "UpgradedContract(string,address)")]
pub struct UpgradedContractFilter {
    #[ethevent(indexed)]
    pub name: ethers::core::types::H256,
    pub addr: ethers::core::types::Address,
}
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthEvent,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethevent(
    name = "ValidatorSetUpdate",
    abi = "ValidatorSetUpdate(uint256,bytes32,bytes32)"
)]
pub struct ValidatorSetUpdateFilter {
    #[ethevent(indexed)]
    pub validator_set_nonce: ethers::core::types::U256,
    pub bridge_validatore_set_hash: [u8; 32],
    pub governance_validatore_set_hash: [u8; 32],
}
#[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
pub enum GovernanceEvents {
    NewContractFilter(NewContractFilter),
    UpdateBridgeWhitelistFilter(UpdateBridgeWhitelistFilter),
    UpgradedContractFilter(UpgradedContractFilter),
    ValidatorSetUpdateFilter(ValidatorSetUpdateFilter),
}
impl ethers::contract::EthLogDecode for GovernanceEvents {
    fn decode_log(
        log: &ethers::core::abi::RawLog,
    ) -> ::std::result::Result<Self, ethers::core::abi::Error>
    where
        Self: Sized,
    {
        if let Ok(decoded) = NewContractFilter::decode_log(log) {
            return Ok(GovernanceEvents::NewContractFilter(decoded));
        }
        if let Ok(decoded) = UpdateBridgeWhitelistFilter::decode_log(log) {
            return Ok(GovernanceEvents::UpdateBridgeWhitelistFilter(decoded));
        }
        if let Ok(decoded) = UpgradedContractFilter::decode_log(log) {
            return Ok(GovernanceEvents::UpgradedContractFilter(decoded));
        }
        if let Ok(decoded) = ValidatorSetUpdateFilter::decode_log(log) {
            return Ok(GovernanceEvents::ValidatorSetUpdateFilter(decoded));
        }
        Err(ethers::core::abi::Error::InvalidData)
    }
}
impl ::std::fmt::Display for GovernanceEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            GovernanceEvents::NewContractFilter(element) => element.fmt(f),
            GovernanceEvents::UpdateBridgeWhitelistFilter(element) => element.fmt(f),
            GovernanceEvents::UpgradedContractFilter(element) => element.fmt(f),
            GovernanceEvents::ValidatorSetUpdateFilter(element) => element.fmt(f),
        }
    }
}
