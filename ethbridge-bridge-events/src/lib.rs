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
#[ethevent(
    name = "InvalidTransferToNamada",
    abi = "InvalidTransferToNamada(address,string,uint256)"
)]
pub struct InvalidTransferToNamadaFilter {
    pub from: ethers::core::types::Address,
    pub to: String,
    pub amount: ethers::core::types::U256,
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
    name = "TransferToErc",
    abi = "TransferToErc(uint256,(address,address,uint256,string,uint256,string)[],string)"
)]
pub struct TransferToErcFilter {
    #[ethevent(indexed)]
    pub nonce: ethers::core::types::U256,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub relayer_address: String,
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
    name = "TransferToNamada",
    abi = "TransferToNamada(uint256,(address,uint256,string)[],uint256)"
)]
pub struct TransferToNamadaFilter {
    pub nonce: ethers::core::types::U256,
    pub trasfers: ::std::vec::Vec<NamadaTransfer>,
    pub confirmations: ethers::core::types::U256,
}
#[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
pub enum BridgeEvents {
    InvalidTransferToNamadaFilter(InvalidTransferToNamadaFilter),
    TransferToErcFilter(TransferToErcFilter),
    TransferToNamadaFilter(TransferToNamadaFilter),
}
impl ethers::contract::EthLogDecode for BridgeEvents {
    fn decode_log(
        log: &ethers::core::abi::RawLog,
    ) -> ::std::result::Result<Self, ethers::core::abi::Error>
    where
        Self: Sized,
    {
        if let Ok(decoded) = InvalidTransferToNamadaFilter::decode_log(log) {
            return Ok(BridgeEvents::InvalidTransferToNamadaFilter(decoded));
        }
        if let Ok(decoded) = TransferToErcFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToErcFilter(decoded));
        }
        if let Ok(decoded) = TransferToNamadaFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToNamadaFilter(decoded));
        }
        Err(ethers::core::abi::Error::InvalidData)
    }
}
impl ::std::fmt::Display for BridgeEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            BridgeEvents::InvalidTransferToNamadaFilter(element) => element.fmt(f),
            BridgeEvents::TransferToErcFilter(element) => element.fmt(f),
            BridgeEvents::TransferToNamadaFilter(element) => element.fmt(f),
        }
    }
}
