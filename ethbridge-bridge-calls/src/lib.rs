#![allow(dead_code)]
use ::ethbridge_structs::*;
#[doc = "Container type for all input parameters for the `authorize` function with signature `authorize((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],bytes32)` and selector `[26, 140, 190, 69]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
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
#[doc = "Container type for all input parameters for the `currentValidatorSetHash` function with signature `currentValidatorSetHash()` and selector `[248, 150, 241, 165]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(name = "currentValidatorSetHash", abi = "currentValidatorSetHash()")]
pub struct CurrentValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `getWhitelistAmountFor` function with signature `getWhitelistAmountFor(address)` and selector `[98, 162, 89, 159]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(name = "getWhitelistAmountFor", abi = "getWhitelistAmountFor(address)")]
pub struct GetWhitelistAmountForCall {
    pub token_address: ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `nextValidatorSetHash` function with signature `nextValidatorSetHash()` and selector `[117, 45, 59, 137]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(name = "nextValidatorSetHash", abi = "nextValidatorSetHash()")]
pub struct NextValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `transferToErc` function with signature `transferToErc(((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],(address,address,uint256,string,uint256,string)[],bytes32,bytes32[],bool[],uint256,string))` and selector `[210, 81, 118, 26]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(
    name = "transferToErc",
    abi = "transferToErc(((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],(address,address,uint256,string,uint256,string)[],bytes32,bytes32[],bool[],uint256,string))"
)]
pub struct TransferToErcCall {
    pub relay_proof: RelayProof,
}
#[doc = "Container type for all input parameters for the `transferToNamada` function with signature `transferToNamada((address,uint256,string)[],uint256)` and selector `[7, 46, 119, 203]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(
    name = "transferToNamada",
    abi = "transferToNamada((address,uint256,string)[],uint256)"
)]
pub struct TransferToNamadaCall {
    pub transfers: ::std::vec::Vec<NamadaTransfer>,
    pub confirmations: ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `updateTokenWhitelist` function with signature `updateTokenWhitelist(address[],uint256[])` and selector `[105, 128, 221, 21]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(
    name = "updateTokenWhitelist",
    abi = "updateTokenWhitelist(address[],uint256[])"
)]
pub struct UpdateTokenWhitelistCall {
    pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    pub tokens_cap: ::std::vec::Vec<ethers::core::types::U256>,
}
#[doc = "Container type for all input parameters for the `updateValidatorSetHash` function with signature `updateValidatorSetHash(bytes32)` and selector `[21, 212, 9, 198]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
)]
#[ethcall(
    name = "updateValidatorSetHash",
    abi = "updateValidatorSetHash(bytes32)"
)]
pub struct UpdateValidatorSetHashCall {
    pub validator_set_hash: [u8; 32],
}
#[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
pub enum BridgeCalls {
    Authorize(AuthorizeCall),
    CurrentValidatorSetHash(CurrentValidatorSetHashCall),
    GetWhitelistAmountFor(GetWhitelistAmountForCall),
    NextValidatorSetHash(NextValidatorSetHashCall),
    TransferToErc(TransferToErcCall),
    TransferToNamada(TransferToNamadaCall),
    UpdateTokenWhitelist(UpdateTokenWhitelistCall),
    UpdateValidatorSetHash(UpdateValidatorSetHashCall),
}
impl ethers::core::abi::AbiDecode for BridgeCalls {
    fn decode(data: impl AsRef<[u8]>) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
        if let Ok(decoded) = <AuthorizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::Authorize(decoded));
        }
        if let Ok(decoded) =
            <CurrentValidatorSetHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::CurrentValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <GetWhitelistAmountForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::GetWhitelistAmountFor(decoded));
        }
        if let Ok(decoded) =
            <NextValidatorSetHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::NextValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <TransferToErcCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::TransferToErc(decoded));
        }
        if let Ok(decoded) =
            <TransferToNamadaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::TransferToNamada(decoded));
        }
        if let Ok(decoded) =
            <UpdateTokenWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::UpdateTokenWhitelist(decoded));
        }
        if let Ok(decoded) =
            <UpdateValidatorSetHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(BridgeCalls::UpdateValidatorSetHash(decoded));
        }
        Err(ethers::core::abi::Error::InvalidData.into())
    }
}
impl ethers::core::abi::AbiEncode for BridgeCalls {
    fn encode(self) -> Vec<u8> {
        match self {
            BridgeCalls::Authorize(element) => element.encode(),
            BridgeCalls::CurrentValidatorSetHash(element) => element.encode(),
            BridgeCalls::GetWhitelistAmountFor(element) => element.encode(),
            BridgeCalls::NextValidatorSetHash(element) => element.encode(),
            BridgeCalls::TransferToErc(element) => element.encode(),
            BridgeCalls::TransferToNamada(element) => element.encode(),
            BridgeCalls::UpdateTokenWhitelist(element) => element.encode(),
            BridgeCalls::UpdateValidatorSetHash(element) => element.encode(),
        }
    }
}
impl ::std::fmt::Display for BridgeCalls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            BridgeCalls::Authorize(element) => element.fmt(f),
            BridgeCalls::CurrentValidatorSetHash(element) => element.fmt(f),
            BridgeCalls::GetWhitelistAmountFor(element) => element.fmt(f),
            BridgeCalls::NextValidatorSetHash(element) => element.fmt(f),
            BridgeCalls::TransferToErc(element) => element.fmt(f),
            BridgeCalls::TransferToNamada(element) => element.fmt(f),
            BridgeCalls::UpdateTokenWhitelist(element) => element.fmt(f),
            BridgeCalls::UpdateValidatorSetHash(element) => element.fmt(f),
        }
    }
}
impl ::std::convert::From<AuthorizeCall> for BridgeCalls {
    fn from(var: AuthorizeCall) -> Self {
        BridgeCalls::Authorize(var)
    }
}
impl ::std::convert::From<CurrentValidatorSetHashCall> for BridgeCalls {
    fn from(var: CurrentValidatorSetHashCall) -> Self {
        BridgeCalls::CurrentValidatorSetHash(var)
    }
}
impl ::std::convert::From<GetWhitelistAmountForCall> for BridgeCalls {
    fn from(var: GetWhitelistAmountForCall) -> Self {
        BridgeCalls::GetWhitelistAmountFor(var)
    }
}
impl ::std::convert::From<NextValidatorSetHashCall> for BridgeCalls {
    fn from(var: NextValidatorSetHashCall) -> Self {
        BridgeCalls::NextValidatorSetHash(var)
    }
}
impl ::std::convert::From<TransferToErcCall> for BridgeCalls {
    fn from(var: TransferToErcCall) -> Self {
        BridgeCalls::TransferToErc(var)
    }
}
impl ::std::convert::From<TransferToNamadaCall> for BridgeCalls {
    fn from(var: TransferToNamadaCall) -> Self {
        BridgeCalls::TransferToNamada(var)
    }
}
impl ::std::convert::From<UpdateTokenWhitelistCall> for BridgeCalls {
    fn from(var: UpdateTokenWhitelistCall) -> Self {
        BridgeCalls::UpdateTokenWhitelist(var)
    }
}
impl ::std::convert::From<UpdateValidatorSetHashCall> for BridgeCalls {
    fn from(var: UpdateValidatorSetHashCall) -> Self {
        BridgeCalls::UpdateValidatorSetHash(var)
    }
}
#[doc = "Container type for all return fields from the `authorize` function with signature `authorize((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],bytes32)` and selector `[26, 140, 190, 69]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
)]
pub struct AuthorizeReturn(pub bool);
#[doc = "Container type for all return fields from the `currentValidatorSetHash` function with signature `currentValidatorSetHash()` and selector `[248, 150, 241, 165]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
)]
pub struct CurrentValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `getWhitelistAmountFor` function with signature `getWhitelistAmountFor(address)` and selector `[98, 162, 89, 159]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
)]
pub struct GetWhitelistAmountForReturn(pub ethers::core::types::U256);
#[doc = "Container type for all return fields from the `nextValidatorSetHash` function with signature `nextValidatorSetHash()` and selector `[117, 45, 59, 137]`"]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
)]
pub struct NextValidatorSetHashReturn(pub [u8; 32]);
