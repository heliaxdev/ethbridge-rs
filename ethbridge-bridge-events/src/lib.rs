#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_structs::*;
#[derive(
    Clone,
    :: ethers_contract :: EthEvent,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethevent(
    name = "InvalidTransferToNamada",
    abi = "InvalidTransferToNamada(address,string,uint256)"
)]
pub struct InvalidTransferToNamadaFilter {
    pub from: ::ethers::core::types::Address,
    pub to: ::std::string::String,
    pub amount: ::ethers::core::types::U256,
}
#[derive(
    Clone,
    :: ethers_contract :: EthEvent,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethevent(
    name = "TransferToErc",
    abi = "TransferToErc(uint256,(address,address,uint256,string,uint256,string)[],bool[],string)"
)]
pub struct TransferToErcFilter {
    #[ethevent(indexed)]
    pub nonce: ::ethers::core::types::U256,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub valid_map: ::std::vec::Vec<bool>,
    pub relayer_address: ::std::string::String,
}
#[derive(
    Clone,
    :: ethers_contract :: EthEvent,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethevent(
    name = "TransferToNamada",
    abi = "TransferToNamada(uint256,(address,uint256,string)[],bool[],uint256)"
)]
pub struct TransferToNamadaFilter {
    pub nonce: ::ethers::core::types::U256,
    pub trasfers: ::std::vec::Vec<NamadaTransfer>,
    pub valid_map: ::std::vec::Vec<bool>,
    pub confirmations: ::ethers::core::types::U256,
}
#[doc = "Container type for all of the contract's events"]
#[derive(Clone, :: ethers_contract :: EthAbiType, Debug, PartialEq, Eq, Hash)]
pub enum BridgeEvents {
    InvalidTransferToNamadaFilter(InvalidTransferToNamadaFilter),
    TransferToErcFilter(TransferToErcFilter),
    TransferToNamadaFilter(TransferToNamadaFilter),
}
impl ::ethers_contract::EthLogDecode for BridgeEvents {
    fn decode_log(
        log: &::ethers::core::abi::RawLog,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
        if let Ok(decoded) = InvalidTransferToNamadaFilter::decode_log(log) {
            return Ok(BridgeEvents::InvalidTransferToNamadaFilter(decoded));
        }
        if let Ok(decoded) = TransferToErcFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToErcFilter(decoded));
        }
        if let Ok(decoded) = TransferToNamadaFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToNamadaFilter(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData)
    }
}
impl ::core::fmt::Display for BridgeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::InvalidTransferToNamadaFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToErcFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToNamadaFilter(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<InvalidTransferToNamadaFilter> for BridgeEvents {
    fn from(value: InvalidTransferToNamadaFilter) -> Self {
        Self::InvalidTransferToNamadaFilter(value)
    }
}
impl ::core::convert::From<TransferToErcFilter> for BridgeEvents {
    fn from(value: TransferToErcFilter) -> Self {
        Self::TransferToErcFilter(value)
    }
}
impl ::core::convert::From<TransferToNamadaFilter> for BridgeEvents {
    fn from(value: TransferToNamadaFilter) -> Self {
        Self::TransferToNamadaFilter(value)
    }
}
