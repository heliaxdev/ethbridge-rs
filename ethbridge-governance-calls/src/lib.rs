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
#[doc = "Container type for all of the contract's call "]
#[derive(Clone, :: ethers_contract :: EthAbiType, Debug, PartialEq, Eq, Hash)]
pub enum GovernanceCalls {
    AddContract(AddContractCall),
    CurrentValidatorSetHash(CurrentValidatorSetHashCall),
    NextValidatorSetHash(NextValidatorSetHashCall),
    UpdateValidatorsSet(UpdateValidatorsSetCall),
    UpgradeBridgeContract(UpgradeBridgeContractCall),
    UpgradeContract(UpgradeContractCall),
    ValidatorSetNonce(ValidatorSetNonceCall),
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
            <CurrentValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::CurrentValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <NextValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::NextValidatorSetHash(decoded));
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
        if let Ok(decoded) = <ValidatorSetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::ValidatorSetNonce(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData.into())
    }
}
impl ::ethers::core::abi::AbiEncode for GovernanceCalls {
    fn encode(self) -> Vec<u8> {
        match self {
            Self::AddContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::CurrentValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
            Self::NextValidatorSetHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpdateValidatorsSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpgradeBridgeContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpgradeContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::ValidatorSetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
        }
    }
}
impl ::core::fmt::Display for GovernanceCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::AddContract(element) => ::core::fmt::Display::fmt(element, f),
            Self::CurrentValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::NextValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpdateValidatorsSet(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpgradeBridgeContract(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpgradeContract(element) => ::core::fmt::Display::fmt(element, f),
            Self::ValidatorSetNonce(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<AddContractCall> for GovernanceCalls {
    fn from(value: AddContractCall) -> Self {
        Self::AddContract(value)
    }
}
impl ::core::convert::From<CurrentValidatorSetHashCall> for GovernanceCalls {
    fn from(value: CurrentValidatorSetHashCall) -> Self {
        Self::CurrentValidatorSetHash(value)
    }
}
impl ::core::convert::From<NextValidatorSetHashCall> for GovernanceCalls {
    fn from(value: NextValidatorSetHashCall) -> Self {
        Self::NextValidatorSetHash(value)
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
impl ::core::convert::From<ValidatorSetNonceCall> for GovernanceCalls {
    fn from(value: ValidatorSetNonceCall) -> Self {
        Self::ValidatorSetNonce(value)
    }
}
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
