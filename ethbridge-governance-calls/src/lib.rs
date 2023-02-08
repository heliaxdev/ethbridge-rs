#![allow(dead_code)]
use ::ethbridge_structs::*;
#[doc = "Container type for all input parameters for the `addContract` function with signature `addContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)` and selector `[15, 145, 207, 253]`"]
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
    name = "addContract",
    abi = "addContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)"
)]
pub struct AddContractCall {
    pub validators: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub name: String,
    pub addr: ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `updateBridgeWhitelist` function with signature `updateBridgeWhitelist((address[],uint256[],uint256),address[],uint256[],(bytes32,bytes32,uint8)[])` and selector `[157, 141, 207, 50]`"]
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
    name = "updateBridgeWhitelist",
    abi = "updateBridgeWhitelist((address[],uint256[],uint256),address[],uint256[],(bytes32,bytes32,uint8)[])"
)]
pub struct UpdateBridgeWhitelistCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    pub tokens_cap: ::std::vec::Vec<ethers::core::types::U256>,
    pub signatures: ::std::vec::Vec<Signature>,
}
#[doc = "Container type for all input parameters for the `updateValidatorsSet` function with signature `updateValidatorsSet((address[],uint256[],uint256),bytes32,bytes32,(bytes32,bytes32,uint8)[],uint256)` and selector `[6, 214, 100, 73]`"]
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
    name = "updateValidatorsSet",
    abi = "updateValidatorsSet((address[],uint256[],uint256),bytes32,bytes32,(bytes32,bytes32,uint8)[],uint256)"
)]
pub struct UpdateValidatorsSetCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub bridge_validator_set_hash: [u8; 32],
    pub governance_validator_set_hash: [u8; 32],
    pub signatures: ::std::vec::Vec<Signature>,
    pub nonce: ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `upgradeBridgeContract` function with signature `upgradeBridgeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],address)` and selector `[186, 7, 223, 13]`"]
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
    name = "upgradeBridgeContract",
    abi = "upgradeBridgeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],address)"
)]
pub struct UpgradeBridgeContractCall {
    pub validators: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub address: ethers::core::types::Address,
}
#[doc = "Container type for all input parameters for the `upgradeContract` function with signature `upgradeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)` and selector `[30, 63, 196, 109]`"]
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
    name = "upgradeContract",
    abi = "upgradeContract((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],string,address)"
)]
pub struct UpgradeContractCall {
    pub validators: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub name: String,
    pub addr: ethers::core::types::Address,
}
#[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
pub enum GovernanceCalls {
    AddContract(AddContractCall),
    UpdateBridgeWhitelist(UpdateBridgeWhitelistCall),
    UpdateValidatorsSet(UpdateValidatorsSetCall),
    UpgradeBridgeContract(UpgradeBridgeContractCall),
    UpgradeContract(UpgradeContractCall),
}
impl ethers::core::abi::AbiDecode for GovernanceCalls {
    fn decode(data: impl AsRef<[u8]>) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
        if let Ok(decoded) =
            <AddContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(GovernanceCalls::AddContract(decoded));
        }
        if let Ok(decoded) =
            <UpdateBridgeWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(GovernanceCalls::UpdateBridgeWhitelist(decoded));
        }
        if let Ok(decoded) =
            <UpdateValidatorsSetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(GovernanceCalls::UpdateValidatorsSet(decoded));
        }
        if let Ok(decoded) =
            <UpgradeBridgeContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(GovernanceCalls::UpgradeBridgeContract(decoded));
        }
        if let Ok(decoded) =
            <UpgradeContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
        {
            return Ok(GovernanceCalls::UpgradeContract(decoded));
        }
        Err(ethers::core::abi::Error::InvalidData.into())
    }
}
impl ethers::core::abi::AbiEncode for GovernanceCalls {
    fn encode(self) -> Vec<u8> {
        match self {
            GovernanceCalls::AddContract(element) => element.encode(),
            GovernanceCalls::UpdateBridgeWhitelist(element) => element.encode(),
            GovernanceCalls::UpdateValidatorsSet(element) => element.encode(),
            GovernanceCalls::UpgradeBridgeContract(element) => element.encode(),
            GovernanceCalls::UpgradeContract(element) => element.encode(),
        }
    }
}
impl ::std::fmt::Display for GovernanceCalls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            GovernanceCalls::AddContract(element) => element.fmt(f),
            GovernanceCalls::UpdateBridgeWhitelist(element) => element.fmt(f),
            GovernanceCalls::UpdateValidatorsSet(element) => element.fmt(f),
            GovernanceCalls::UpgradeBridgeContract(element) => element.fmt(f),
            GovernanceCalls::UpgradeContract(element) => element.fmt(f),
        }
    }
}
impl ::std::convert::From<AddContractCall> for GovernanceCalls {
    fn from(var: AddContractCall) -> Self {
        GovernanceCalls::AddContract(var)
    }
}
impl ::std::convert::From<UpdateBridgeWhitelistCall> for GovernanceCalls {
    fn from(var: UpdateBridgeWhitelistCall) -> Self {
        GovernanceCalls::UpdateBridgeWhitelist(var)
    }
}
impl ::std::convert::From<UpdateValidatorsSetCall> for GovernanceCalls {
    fn from(var: UpdateValidatorsSetCall) -> Self {
        GovernanceCalls::UpdateValidatorsSet(var)
    }
}
impl ::std::convert::From<UpgradeBridgeContractCall> for GovernanceCalls {
    fn from(var: UpgradeBridgeContractCall) -> Self {
        GovernanceCalls::UpgradeBridgeContract(var)
    }
}
impl ::std::convert::From<UpgradeContractCall> for GovernanceCalls {
    fn from(var: UpgradeContractCall) -> Self {
        GovernanceCalls::UpgradeContract(var)
    }
}
