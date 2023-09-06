#![allow(dead_code)]
use ::ethbridge_structs::*;
#[doc = "Container type for all input parameters for the `currentBridgeValidatorSetHash` function with signature `currentBridgeValidatorSetHash()` and selector `0x7379d426`"]
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
    name = "currentBridgeValidatorSetHash",
    abi = "currentBridgeValidatorSetHash()"
)]
pub struct CurrentBridgeValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `currentGovernanceValidatorSetHash` function with signature `currentGovernanceValidatorSetHash()` and selector `0x0df6a752`"]
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
    name = "currentGovernanceValidatorSetHash",
    abi = "currentGovernanceValidatorSetHash()"
)]
pub struct CurrentGovernanceValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `nextBridgeValidatorSetHash` function with signature `nextBridgeValidatorSetHash()` and selector `0xf5cd6594`"]
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
    name = "nextBridgeValidatorSetHash",
    abi = "nextBridgeValidatorSetHash()"
)]
pub struct NextBridgeValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `nextGovernanceValidatorSetHash` function with signature `nextGovernanceValidatorSetHash()` and selector `0x1c461a91`"]
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
    name = "nextGovernanceValidatorSetHash",
    abi = "nextGovernanceValidatorSetHash()"
)]
pub struct NextGovernanceValidatorSetHashCall;
#[doc = "Container type for all input parameters for the `transferToChain` function with signature `transferToChain((uint256,address,string)[],uint256)` and selector `0x224738eb`"]
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
    name = "transferToChain",
    abi = "transferToChain((uint256,address,string)[],uint256)"
)]
pub struct TransferToChainCall {
    pub transfers: ::std::vec::Vec<ChainTransfer>,
    pub confirmations: ::ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `transferToChainNonce` function with signature `transferToChainNonce()` and selector `0x21b1eb5f`"]
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
#[ethcall(name = "transferToChainNonce", abi = "transferToChainNonce()")]
pub struct TransferToChainNonceCall;
#[doc = "Container type for all input parameters for the `transferToErc` function with signature `transferToErc((bytes32[],uint256),(bytes32,bytes32,uint8)[],((bytes32,uint256,address,address)[],bytes32,bytes32[],bool[],uint256,string))` and selector `0x998c0030`"]
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
    abi = "transferToErc((bytes32[],uint256),(bytes32,bytes32,uint8)[],((bytes32,uint256,address,address)[],bytes32,bytes32[],bool[],uint256,string))"
)]
pub struct TransferToErcCall {
    pub validator_set_args: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
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
#[doc = "Container type for all input parameters for the `updateValidatorSet` function with signature `updateValidatorSet((bytes32[],uint256),bytes32,bytes32,(bytes32,bytes32,uint8)[])` and selector `0xfd48739e`"]
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
    name = "updateValidatorSet",
    abi = "updateValidatorSet((bytes32[],uint256),bytes32,bytes32,(bytes32,bytes32,uint8)[])"
)]
pub struct UpdateValidatorSetCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub bridge_validator_set_hash: [u8; 32],
    pub governance_validator_set_hash: [u8; 32],
    pub signatures: ::std::vec::Vec<Signature>,
}
#[doc = "Container type for all input parameters for the `upgrade` function with signature `upgrade((bytes32[],uint256),(bytes32,bytes32,uint8)[],address,uint256)` and selector `0xc3203d76`"]
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
    name = "upgrade",
    abi = "upgrade((bytes32[],uint256),(bytes32,bytes32,uint8)[],address,uint256)"
)]
pub struct UpgradeCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub to: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `upgradeNonce` function with signature `upgradeNonce()` and selector `0x1af628b9`"]
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
#[ethcall(name = "upgradeNonce", abi = "upgradeNonce()")]
pub struct UpgradeNonceCall;
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
#[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw((bytes32[],uint256),(bytes32,uint256,address,address)[],(bytes32,bytes32,uint8)[],uint256)` and selector `0x43d80041`"]
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
    name = "withdraw",
    abi = "withdraw((bytes32[],uint256),(bytes32,uint256,address,address)[],(bytes32,bytes32,uint8)[],uint256)"
)]
pub struct WithdrawCall {
    pub current_validator_set_args: ValidatorSetArgs,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub signatures: ::std::vec::Vec<Signature>,
    pub nonce: ::ethers::core::types::U256,
}
#[doc = "Container type for all input parameters for the `withdrawNonce` function with signature `withdrawNonce()` and selector `0xb8a4e151`"]
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
#[ethcall(name = "withdrawNonce", abi = "withdrawNonce()")]
pub struct WithdrawNonceCall;
#[doc = "Container type for all of the contract's call "]
#[derive(Clone, :: ethers_contract :: EthAbiType, Debug, PartialEq, Eq, Hash)]
pub enum BridgeCalls {
    CurrentBridgeValidatorSetHash(CurrentBridgeValidatorSetHashCall),
    CurrentGovernanceValidatorSetHash(CurrentGovernanceValidatorSetHashCall),
    NextBridgeValidatorSetHash(NextBridgeValidatorSetHashCall),
    NextGovernanceValidatorSetHash(NextGovernanceValidatorSetHashCall),
    TransferToChain(TransferToChainCall),
    TransferToChainNonce(TransferToChainNonceCall),
    TransferToErc(TransferToErcCall),
    TransferToErc20Nonce(TransferToErc20NonceCall),
    UpdateValidatorSet(UpdateValidatorSetCall),
    Upgrade(UpgradeCall),
    UpgradeNonce(UpgradeNonceCall),
    ValidatorSetNonce(ValidatorSetNonceCall),
    Withdraw(WithdrawCall),
    WithdrawNonce(WithdrawNonceCall),
}
impl ::ethers::core::abi::AbiDecode for BridgeCalls {
    fn decode(
        data: impl AsRef<[u8]>,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
        let data = data.as_ref();
        if let Ok(decoded) =
            <CurrentBridgeValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::CurrentBridgeValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <CurrentGovernanceValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::CurrentGovernanceValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <NextBridgeValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::NextBridgeValidatorSetHash(decoded));
        }
        if let Ok(decoded) =
            <NextGovernanceValidatorSetHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::NextGovernanceValidatorSetHash(decoded));
        }
        if let Ok(decoded) = <TransferToChainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::TransferToChain(decoded));
        }
        if let Ok(decoded) =
            <TransferToChainNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::TransferToChainNonce(decoded));
        }
        if let Ok(decoded) = <TransferToErcCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::TransferToErc(decoded));
        }
        if let Ok(decoded) =
            <TransferToErc20NonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::TransferToErc20Nonce(decoded));
        }
        if let Ok(decoded) =
            <UpdateValidatorSetCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::UpdateValidatorSet(decoded));
        }
        if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::Upgrade(decoded));
        }
        if let Ok(decoded) = <UpgradeNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::UpgradeNonce(decoded));
        }
        if let Ok(decoded) = <ValidatorSetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
        {
            return Ok(Self::ValidatorSetNonce(decoded));
        }
        if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::Withdraw(decoded));
        }
        if let Ok(decoded) = <WithdrawNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
            return Ok(Self::WithdrawNonce(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData.into())
    }
}
impl ::ethers::core::abi::AbiEncode for BridgeCalls {
    fn encode(self) -> Vec<u8> {
        match self {
            Self::CurrentBridgeValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
            Self::CurrentGovernanceValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
            Self::NextBridgeValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
            Self::NextGovernanceValidatorSetHash(element) => {
                ::ethers::core::abi::AbiEncode::encode(element)
            }
            Self::TransferToChain(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToChainNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToErc(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::TransferToErc20Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpdateValidatorSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::UpgradeNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::ValidatorSetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
            Self::WithdrawNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
        }
    }
}
impl ::core::fmt::Display for BridgeCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::CurrentBridgeValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::CurrentGovernanceValidatorSetHash(element) => {
                ::core::fmt::Display::fmt(element, f)
            }
            Self::NextBridgeValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::NextGovernanceValidatorSetHash(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToChain(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToChainNonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToErc(element) => ::core::fmt::Display::fmt(element, f),
            Self::TransferToErc20Nonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpdateValidatorSet(element) => ::core::fmt::Display::fmt(element, f),
            Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpgradeNonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::ValidatorSetNonce(element) => ::core::fmt::Display::fmt(element, f),
            Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            Self::WithdrawNonce(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<CurrentBridgeValidatorSetHashCall> for BridgeCalls {
    fn from(value: CurrentBridgeValidatorSetHashCall) -> Self {
        Self::CurrentBridgeValidatorSetHash(value)
    }
}
impl ::core::convert::From<CurrentGovernanceValidatorSetHashCall> for BridgeCalls {
    fn from(value: CurrentGovernanceValidatorSetHashCall) -> Self {
        Self::CurrentGovernanceValidatorSetHash(value)
    }
}
impl ::core::convert::From<NextBridgeValidatorSetHashCall> for BridgeCalls {
    fn from(value: NextBridgeValidatorSetHashCall) -> Self {
        Self::NextBridgeValidatorSetHash(value)
    }
}
impl ::core::convert::From<NextGovernanceValidatorSetHashCall> for BridgeCalls {
    fn from(value: NextGovernanceValidatorSetHashCall) -> Self {
        Self::NextGovernanceValidatorSetHash(value)
    }
}
impl ::core::convert::From<TransferToChainCall> for BridgeCalls {
    fn from(value: TransferToChainCall) -> Self {
        Self::TransferToChain(value)
    }
}
impl ::core::convert::From<TransferToChainNonceCall> for BridgeCalls {
    fn from(value: TransferToChainNonceCall) -> Self {
        Self::TransferToChainNonce(value)
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
impl ::core::convert::From<UpdateValidatorSetCall> for BridgeCalls {
    fn from(value: UpdateValidatorSetCall) -> Self {
        Self::UpdateValidatorSet(value)
    }
}
impl ::core::convert::From<UpgradeCall> for BridgeCalls {
    fn from(value: UpgradeCall) -> Self {
        Self::Upgrade(value)
    }
}
impl ::core::convert::From<UpgradeNonceCall> for BridgeCalls {
    fn from(value: UpgradeNonceCall) -> Self {
        Self::UpgradeNonce(value)
    }
}
impl ::core::convert::From<ValidatorSetNonceCall> for BridgeCalls {
    fn from(value: ValidatorSetNonceCall) -> Self {
        Self::ValidatorSetNonce(value)
    }
}
impl ::core::convert::From<WithdrawCall> for BridgeCalls {
    fn from(value: WithdrawCall) -> Self {
        Self::Withdraw(value)
    }
}
impl ::core::convert::From<WithdrawNonceCall> for BridgeCalls {
    fn from(value: WithdrawNonceCall) -> Self {
        Self::WithdrawNonce(value)
    }
}
#[doc = "Container type for all return fields from the `currentBridgeValidatorSetHash` function with signature `currentBridgeValidatorSetHash()` and selector `0x7379d426`"]
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
pub struct CurrentBridgeValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `currentGovernanceValidatorSetHash` function with signature `currentGovernanceValidatorSetHash()` and selector `0x0df6a752`"]
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
pub struct CurrentGovernanceValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `nextBridgeValidatorSetHash` function with signature `nextBridgeValidatorSetHash()` and selector `0xf5cd6594`"]
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
pub struct NextBridgeValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `nextGovernanceValidatorSetHash` function with signature `nextGovernanceValidatorSetHash()` and selector `0x1c461a91`"]
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
pub struct NextGovernanceValidatorSetHashReturn(pub [u8; 32]);
#[doc = "Container type for all return fields from the `transferToChainNonce` function with signature `transferToChainNonce()` and selector `0x21b1eb5f`"]
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
pub struct TransferToChainNonceReturn(pub ::ethers::core::types::U256);
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
#[doc = "Container type for all return fields from the `upgradeNonce` function with signature `upgradeNonce()` and selector `0x1af628b9`"]
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
pub struct UpgradeNonceReturn(pub ::ethers::core::types::U256);
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
#[doc = "Container type for all return fields from the `withdrawNonce` function with signature `withdrawNonce()` and selector `0xb8a4e151`"]
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
pub struct WithdrawNonceReturn(pub ::ethers::core::types::U256);
