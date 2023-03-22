#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_structs::*;
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(
        name = "TransferToErc",
        abi = "TransferToErc(uint256,(address,address,uint256,string,uint256,string)[],bool[],string)"
    )
)]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct TransferToErcFilter {
    #[cfg_attr(feature = "ethers-derive", ethevent(indexed))]
    pub nonce: ::ethabi::ethereum_types::U256,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub valid_map: ::std::vec::Vec<bool>,
    pub relayer_address: ::std::string::String,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(
        name = "TransferToNamada",
        abi = "TransferToNamada(uint256,(address,uint256,string)[],bool[],uint256)"
    )
)]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct TransferToNamadaFilter {
    pub nonce: ::ethabi::ethereum_types::U256,
    pub transfers: ::std::vec::Vec<NamadaTransfer>,
    pub valid_map: ::std::vec::Vec<bool>,
    pub confirmations: ::ethabi::ethereum_types::U256,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[doc = "Container type for all of the contract's events"]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BridgeEvents {
    TransferToErcFilter(TransferToErcFilter),
    TransferToNamadaFilter(TransferToNamadaFilter),
}
#[cfg(feature = "ethers-derive")]
impl ::ethers_contract::EthLogDecode for BridgeEvents {
    fn decode_log(
        log: &::ethers::core::abi::RawLog,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
        if let Ok(decoded) = TransferToErcFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToErcFilter(decoded));
        }
        if let Ok(decoded) = TransferToNamadaFilter::decode_log(log) {
            return Ok(BridgeEvents::TransferToNamadaFilter(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData)
    }
}
#[cfg(feature = "ethers-derive")]
impl ::core::fmt::Display for BridgeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::TransferToErcFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToNamadaFilter(element) => ::core::fmt::Display::fmt(element, f),
        }
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
#[doc = r"Retrieve all ABI event signatures."]
#[cfg(feature = "ethers-derive")]
pub fn abi_signatures() -> Vec<&'static str> {
    vec![
        {
            use ::ethers_contract::EthEvent;
            match TransferToErcFilter::abi_signature() {
                ::std::borrow::Cow::Borrowed(s) => s,
                _ => unreachable!(
                    "The Ethereum event ABI def for {} should be static",
                    TransferToErcFilter::name()
                ),
            }
        },
        {
            use ::ethers_contract::EthEvent;
            match TransferToNamadaFilter::abi_signature() {
                ::std::borrow::Cow::Borrowed(s) => s,
                _ => unreachable!(
                    "The Ethereum event ABI def for {} should be static",
                    TransferToNamadaFilter::name()
                ),
            }
        },
    ]
}
