#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_structs::*;
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(
        name = "TransferToChain",
        abi = "TransferToChain(uint256,(uint256,address,string)[],uint256)"
    )
)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct TransferToChainFilter {
    pub nonce: ::ethabi::ethereum_types::U256,
    pub transfers: ::std::vec::Vec<ChainTransfer>,
    pub confirmations: ::ethabi::ethereum_types::U256,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(
        name = "TransferToErc",
        abi = "TransferToErc(uint256,(bytes32,uint256,address,address)[],string)"
    )
)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct TransferToErcFilter {
    #[cfg_attr(feature = "ethers-derive", ethevent(indexed))]
    pub nonce: ::ethabi::ethereum_types::U256,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub relayer_address: ::std::string::String,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(
        name = "ValidatorSetUpdate",
        abi = "ValidatorSetUpdate(uint256,bytes32,bytes32)"
    )
)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct ValidatorSetUpdateFilter {
    #[cfg_attr(feature = "ethers-derive", ethevent(indexed))]
    pub validator_set_nonce: ::ethabi::ethereum_types::U256,
    pub bridge_validator_set_hash: [u8; 32],
    pub governance_validator_set_hash: [u8; 32],
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[doc = "Container type for all of the contract's events"]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BridgeEvents {
    TransferToChainFilter(TransferToChainFilter),
    TransferToErcFilter(TransferToErcFilter),
    ValidatorSetUpdateFilter(ValidatorSetUpdateFilter),
}
#[cfg(feature = "ethers-derive")]
impl ::ethers_contract::EthLogDecode for BridgeEvents {
    fn decode_log(
        log: &::ethers::core::abi::RawLog,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
        if let Ok(decoded) = TransferToChainFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToChainFilter(decoded));
        }
        if let Ok(decoded) = TransferToErcFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToErcFilter(decoded));
        }
        if let Ok(decoded) = ValidatorSetUpdateFilter::decode_log(log) {
            return Ok(BridgeEvents::ValidatorSetUpdateFilter(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData)
    }
}
#[cfg(feature = "ethers-derive")]
impl ::core::fmt::Display for BridgeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::TransferToChainFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToErcFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::ValidatorSetUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<TransferToChainFilter> for BridgeEvents {
    fn from(value: TransferToChainFilter) -> Self {
        Self::TransferToChainFilter(value)
    }
}
impl ::core::convert::From<TransferToErcFilter> for BridgeEvents {
    fn from(value: TransferToErcFilter) -> Self {
        Self::TransferToErcFilter(value)
    }
}
impl ::core::convert::From<ValidatorSetUpdateFilter> for BridgeEvents {
    fn from(value: ValidatorSetUpdateFilter) -> Self {
        Self::ValidatorSetUpdateFilter(value)
    }
}
