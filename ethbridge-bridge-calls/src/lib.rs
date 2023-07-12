#![allow(dead_code)]
use ::ethbridge_structs::*;
#[doc = "Container type for all input parameters for the `authorize` function with signature `authorize((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],bytes32)` and selector `0x1a8cbe45`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(
    name = "authorize",
    abi = "authorize((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],bytes32)"
)]
pub struct AuthorizeCall {
    pub validator_set_args: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub message: [u8; 32],
}
#[doc = "Container type for all input parameters for the `currentValidatorSetHash` function with signature `currentValidatorSetHash()` and selector `0xf896f1a5`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(name = "currentValidatorSetHash", abi = "currentValidatorSetHash()")]
pub struct CurrentValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `getWhitelistAmountFor` function with signature `getWhitelistAmountFor(address)` and selector `0x62a2599f`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(name = "getWhitelistAmountFor", abi = "getWhitelistAmountFor(address)")]
pub struct GetWhitelistAmountForCall {
    pub token_address: ::ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `nextValidatorSetHash` function with signature `nextValidatorSetHash()` and selector `0x752d3b89`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(name = "nextValidatorSetHash", abi = "nextValidatorSetHash()")]
pub struct NextValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `transferToErc` function with signature `transferToErc(((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],(uint8,address,address,uint256,string,uint256,string)[],bytes32,bytes32[],bool[],uint256,string))` and selector `0xb9094041`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(
    name = "transferToErc",
    abi = "transferToErc(((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],(uint8,address,address,uint256,string,uint256,string)[],bytes32,bytes32[],bool[],uint256,string))"
)]
pub struct TransferToErcCall {
    pub relay_proof: RelayProof,
}
#[doc = "Container type for all input parameters for the `transferToErc20Nonce` function with signature `transferToErc20Nonce()` and selector `0x5f9fd577`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(name = "transferToErc20Nonce", abi = "transferToErc20Nonce()")]
pub struct TransferToErc20NonceCall;
#[doc = "Container type for all input parameters for the `transferToNamada` function with signature `transferToNamada((address,uint256,string)[],uint256)` and selector `0x072e77cb`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(
    name = "transferToNamada",
    abi = "transferToNamada((address,uint256,string)[],uint256)"
)]
pub struct TransferToNamadaCall {
    pub transfers: ::std::vec::Vec<NamadaTransfer>,
    pub confirmations: ::ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `transferToNamadaNonce` function with signature `transferToNamadaNonce()` and selector `0xc2d0a82b`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(name = "transferToNamadaNonce", abi = "transferToNamadaNonce()")]
pub struct TransferToNamadaNonceCall;
#[doc = "Container type for all input parameters for the `updateTokenWhitelist` function with signature `updateTokenWhitelist(address[],uint256[])` and selector `0x6980dd15`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(
    name = "updateTokenWhitelist",
    abi = "updateTokenWhitelist(address[],uint256[])"
)]
pub struct UpdateTokenWhitelistCall {
    pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    pub tokens_cap: ::std::vec::Vec<::ethers::core::types::U256>,
}
#[doc = "Container type for all input parameters for the `updateValidatorSetHash` function with signature `updateValidatorSetHash(bytes32)` and selector `0x15d409c6`"]
#[derive(
    Clone,
    :: ethers_contract :: EthCall,
    :: ethers_contract :: EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
#[ethcall(
    name = "updateValidatorSetHash",
    abi = "updateValidatorSetHash(bytes32)"
)]
pub struct UpdateValidatorSetHashCall {
    pub validator_set_hash: [u8; 32],
}
#[doc = "Container type for all of the contract's call "]
#[derive(Clone, :: ethers_contract :: EthAbiType, Debug, PartialEq, Eq, Hash)]
pub enum BridgeCalls {
    Authorize(AuthorizeCall),
    CurrentValidatorSetHash(CurrentValidatorSetHashCall),
    GetWhitelistAmountFor(GetWhitelistAmountForCall),
    NextValidatorSetHash(NextValidatorSetHashCall),
    TransferToErc(TransferToErcCall),
    TransferToErc20Nonce(TransferToErc20NonceCall),
    TransferToNamada(TransferToNamadaCall),
    TransferToNamadaNonce(TransferToNamadaNonceCall),
    UpdateTokenWhitelist(UpdateTokenWhitelistCall),
    UpdateValidatorSetHash(UpdateValidatorSetHashCall),
}
impl ::ethers::core::abi::AbiDecode for BridgeCalls {
    fn decode(
        data: impl AsRef<[u8]>,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
        let data = data.as_ref();
        if let Ok(decoded) = <AuthorizeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::Authorize(decoded));
        }
        if let Ok(decoded) =
            <CurrentValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::CurrentValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <GetWhitelistAmountForCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::GetWhitelistAmountFor(decoded));
        }
        if let Ok(decoded) =
            <NextValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::NextValidatorSetHash(decoded));
        }
        if let Ok(decoded) = <TransferToErcCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::TransferToErc(decoded));
        }
        if let Ok(decoded) =
            <TransferToErc20NonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::TransferToErc20Nonce(decoded));
        }
        if let Ok(decoded) = <TransferToNamadaCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::TransferToNamada(decoded));
        }
        if let Ok(decoded) =
            <TransferToNamadaNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::TransferToNamadaNonce(decoded));
        }
        if let Ok(decoded) =
            <UpdateTokenWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::UpdateTokenWhitelist(decoded));
        }
        if let Ok(decoded) =
            <UpdateValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::UpdateValidatorSetHash(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData.into())
    }
}
impl ::ethers::core::abi::AbiEncode for BridgeCalls {
    fn encode(self) -> Vec<u8> {
        match self {
            Self::Authorize(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::CurrentValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
            Self::GetWhitelistAmountFor(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::NextValidatorSetHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToErc(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToErc20Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToNamada(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToNamadaNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpdateTokenWhitelist(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpdateValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
        }
    }
}
impl ::core::fmt::Display for BridgeCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Authorize(element) => ::core::fmt::Display::fmt(element, f),
            Self::CurrentValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::GetWhitelistAmountFor(element) => ::core::fmt::Display::fmt(element, f),
            Self::NextValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToErc(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToErc20Nonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToNamada(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToNamadaNonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpdateTokenWhitelist(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpdateValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<AuthorizeCall> for BridgeCalls {
    fn from(value: AuthorizeCall) -> Self {
        Self::Authorize(value)
    }
}
impl ::core::convert::From<CurrentValidatorSetHashCall> for BridgeCalls {
    fn from(value: CurrentValidatorSetHashCall) -> Self {
        Self::CurrentValidatorSetHash(value)
    }
}
impl ::core::convert::From<GetWhitelistAmountForCall> for BridgeCalls {
    fn from(value: GetWhitelistAmountForCall) -> Self {
        Self::GetWhitelistAmountFor(value)
    }
}
impl ::core::convert::From<NextValidatorSetHashCall> for BridgeCalls {
    fn from(value: NextValidatorSetHashCall) -> Self {
        Self::NextValidatorSetHash(value)
    }
}
impl ::core::convert::From<TransferToErcCall> for BridgeCalls {
    fn from(value: TransferToErcCall) -> Self {
        Self::TransferToErc(value)
    }
}
impl ::core::convert::From<TransferToErc20NonceCall> for BridgeCalls {
    fn from(value: TransferToErc20NonceCall) -> Self {
        Self::TransferToErc20Nonce(value)
    }
}
impl ::core::convert::From<TransferToNamadaCall> for BridgeCalls {
    fn from(value: TransferToNamadaCall) -> Self {
        Self::TransferToNamada(value)
    }
}
impl ::core::convert::From<TransferToNamadaNonceCall> for BridgeCalls {
    fn from(value: TransferToNamadaNonceCall) -> Self {
        Self::TransferToNamadaNonce(value)
    }
}
impl ::core::convert::From<UpdateTokenWhitelistCall> for BridgeCalls {
    fn from(value: UpdateTokenWhitelistCall) -> Self {
        Self::UpdateTokenWhitelist(value)
    }
}
impl ::core::convert::From<UpdateValidatorSetHashCall> for BridgeCalls {
    fn from(value: UpdateValidatorSetHashCall) -> Self {
        Self::UpdateValidatorSetHash(value)
    }
}
#[doc = "Container type for all return fields from the `authorize` function with signature `authorize((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],bytes32)` and selector `0x1a8cbe45`"]
#[derive(
    Clone,
    :: ethers_contract :: EthAbiType,
    :: ethers_contract :: EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct AuthorizeReturn(pub bool);
#[doc = "Container type for all return fields from the `currentValidatorSetHash` function with signature `currentValidatorSetHash()` and selector `0xf896f1a5`"]
#[derive(
    Clone,
    :: ethers_contract :: EthAbiType,
    :: ethers_contract :: EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CurrentValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `getWhitelistAmountFor` function with signature `getWhitelistAmountFor(address)` and selector `0x62a2599f`"]
#[derive(
    Clone,
    :: ethers_contract :: EthAbiType,
    :: ethers_contract :: EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct GetWhitelistAmountForReturn(pub ::ethers::core::types::U256);
#[doc = "Container type for all return fields from the `nextValidatorSetHash` function with signature `nextValidatorSetHash()` and selector `0x752d3b89`"]
#[derive(
    Clone,
    :: ethers_contract :: EthAbiType,
    :: ethers_contract :: EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct NextValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `transferToErc20Nonce` function with signature `transferToErc20Nonce()` and selector `0x5f9fd577`"]
#[derive(
    Clone,
    :: ethers_contract :: EthAbiType,
    :: ethers_contract :: EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TransferToErc20NonceReturn(pub ::ethers::core::types::U256);
#[doc = "Container type for all return fields from the `transferToNamadaNonce` function with signature `transferToNamadaNonce()` and selector `0xc2d0a82b`"]
#[derive(
    Clone,
    :: ethers_contract :: EthAbiType,
    :: ethers_contract :: EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TransferToNamadaNonceReturn(pub ::ethers::core::types::U256);
