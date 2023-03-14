#![allow(dead_code)]
use ::ethbridge_structs::*;
#[doc = "Container type for all input parameters for the `addContract` function with signature `addContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)` and selector `0x0f91cffd`"]
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
    name = "addContract",
    abi = "addContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)"
)]
pub struct AddContractCall {
    pub validators: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub name: ::std::string::String,
    pub address: ::ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `updateBridgeWhitelist` function with signature `updateBridgeWhitelist((address[],uint256[],uint256),address[],uint256[],(bytes32,bytes32,uint8)[])` and selector `0x9d8dcf32`"]
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
    name = "updateBridgeWhitelist",
    abi = "updateBridgeWhitelist((address[],uint256[],uint256),address[],uint256[],(bytes32,bytes32,uint8)[])"
)]
pub struct UpdateBridgeWhitelistCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    pub tokens_cap: ::std::vec::Vec<::ethers::core::types::U256>,
    pub signatures: ::std::vec::Vec<Signature>,
}
#[doc = "Container type for all input parameters for the `updateValidatorsSet` function with signature `updateValidatorsSet((address[],uint256[],uint256),bytes32,bytes32,(bytes32,bytes32,uint8)[],uint256)` and selector `0x06d66449`"]
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
    name = "updateValidatorsSet",
    abi = "updateValidatorsSet((address[],uint256[],uint256),bytes32,bytes32,(bytes32,bytes32,uint8)[],uint256)"
)]
pub struct UpdateValidatorsSetCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub bridge_validator_set_hash: [u8; 32],
    pub governance_validator_set_hash: [u8; 32],
    pub signatures: ::std::vec::Vec<Signature>,
    pub nonce: ::ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `upgradeBridgeContract` function with signature `upgradeBridgeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],address)` and selector `0xba07df0d`"]
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
    name = "upgradeBridgeContract",
    abi = "upgradeBridgeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],address)"
)]
pub struct UpgradeBridgeContractCall {
    pub validators: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub address: ::ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `upgradeContract` function with signature `upgradeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)` and selector `0x1e3fc46d`"]
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
    name = "upgradeContract",
    abi = "upgradeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)"
)]
pub struct UpgradeContractCall {
    pub validators: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub name: ::std::string::String,
    pub address: ::ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `validatorSetHash` function with signature `validatorSetHash()` and selector `0xcdea2912`"]
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
#[ethcall(name = "validatorSetHash", abi = "validatorSetHash()")]
pub struct ValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `validatorSetNonce` function with signature `validatorSetNonce()` and selector `0x486f2e46`"]
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
#[ethcall(name = "validatorSetNonce", abi = "validatorSetNonce()")]
pub struct ValidatorSetNonceCall;
#[doc = "Container type for all input parameters for the `whitelistNonce` function with signature `whitelistNonce()` and selector `0xb5c3030b`"]
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
#[ethcall(name = "whitelistNonce", abi = "whitelistNonce()")]
pub struct WhitelistNonceCall;
#[doc = "Container type for all of the contract's call "]
#[derive(Clone, :: ethers_contract :: EthAbiType, Debug, PartialEq, Eq, Hash)]
pub enum GovernanceCalls {
    AddContract(AddContractCall),
    UpdateBridgeWhitelist(UpdateBridgeWhitelistCall),
    UpdateValidatorsSet(UpdateValidatorsSetCall),
    UpgradeBridgeContract(UpgradeBridgeContractCall),
    UpgradeContract(UpgradeContractCall),
    ValidatorSetHash(ValidatorSetHashCall),
    ValidatorSetNonce(ValidatorSetNonceCall),
    WhitelistNonce(WhitelistNonceCall),
}
impl ::ethers::core::abi::AbiDecode for GovernanceCalls {
    fn decode(
        data: impl AsRef<[u8]>,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
        let data = data.as_ref();
        if let Ok(decoded) = <AddContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::AddContract(decoded));
        }
        if let Ok(decoded) =
            <UpdateBridgeWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::UpdateBridgeWhitelist(decoded));
        }
        if let Ok(decoded) =
            <UpdateValidatorsSetCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::UpdateValidatorsSet(decoded));
        }
        if let Ok(decoded) =
            <UpgradeBridgeContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::UpgradeBridgeContract(decoded));
        }
        if let Ok(decoded) = <UpgradeContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::UpgradeContract(decoded));
        }
        if let Ok(decoded) = <ValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::ValidatorSetHash(decoded));
        }
        if let Ok(decoded) = <ValidatorSetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::ValidatorSetNonce(decoded));
        }
        if let Ok(decoded) = <WhitelistNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::WhitelistNonce(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData.into())
    }
}
impl ::ethers::core::abi::AbiEncode for GovernanceCalls {
    fn encode(self) -> Vec<u8> {
        match self {
            Self::AddContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpdateBridgeWhitelist(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpdateValidatorsSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpgradeBridgeContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpgradeContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::ValidatorSetHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::ValidatorSetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::WhitelistNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
        }
    }
}
impl ::core::fmt::Display for GovernanceCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::AddContract(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpdateBridgeWhitelist(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpdateValidatorsSet(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpgradeBridgeContract(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpgradeContract(element) => ::core::fmt::Display::fmt(element, f),
            Self::ValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::ValidatorSetNonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::WhitelistNonce(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<AddContractCall> for GovernanceCalls {
    fn from(value: AddContractCall) -> Self {
        Self::AddContract(value)
    }
}
impl ::core::convert::From<UpdateBridgeWhitelistCall> for GovernanceCalls {
    fn from(value: UpdateBridgeWhitelistCall) -> Self {
        Self::UpdateBridgeWhitelist(value)
    }
}
impl ::core::convert::From<UpdateValidatorsSetCall> for GovernanceCalls {
    fn from(value: UpdateValidatorsSetCall) -> Self {
        Self::UpdateValidatorsSet(value)
    }
}
impl ::core::convert::From<UpgradeBridgeContractCall> for GovernanceCalls {
    fn from(value: UpgradeBridgeContractCall) -> Self {
        Self::UpgradeBridgeContract(value)
    }
}
impl ::core::convert::From<UpgradeContractCall> for GovernanceCalls {
    fn from(value: UpgradeContractCall) -> Self {
        Self::UpgradeContract(value)
    }
}
impl ::core::convert::From<ValidatorSetHashCall> for GovernanceCalls {
    fn from(value: ValidatorSetHashCall) -> Self {
        Self::ValidatorSetHash(value)
    }
}
impl ::core::convert::From<ValidatorSetNonceCall> for GovernanceCalls {
    fn from(value: ValidatorSetNonceCall) -> Self {
        Self::ValidatorSetNonce(value)
    }
}
impl ::core::convert::From<WhitelistNonceCall> for GovernanceCalls {
    fn from(value: WhitelistNonceCall) -> Self {
        Self::WhitelistNonce(value)
    }
}
#[doc = "Container type for all return fields from the `validatorSetHash` function with signature `validatorSetHash()` and selector `0xcdea2912`"]
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
pub struct ValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `validatorSetNonce` function with signature `validatorSetNonce()` and selector `0x486f2e46`"]
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
pub struct ValidatorSetNonceReturn(pub ::ethers::core::types::U256);
#[doc = "Container type for all return fields from the `whitelistNonce` function with signature `whitelistNonce()` and selector `0xb5c3030b`"]
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
pub struct WhitelistNonceReturn(pub ::ethers::core::types::U256);
