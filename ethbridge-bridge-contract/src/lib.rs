#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use ::ethbridge_bridge_events::*;
use ::ethbridge_structs::*;
# [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_version\",\"type\":\"uint8\"},{\"internalType\":\"bytes32[]\",\"name\":\"_currentBridgeValidatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[]\",\"name\":\"_nextBridgeValidatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[]\",\"name\":\"_currentGovernanceValidatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[]\",\"name\":\"_nextGovernanceValidatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"contract IProxy\",\"name\":\"_proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"MerkleProofInvalidMultiproof\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"to\",\"type\":\"string\"}],\"indexed\":false,\"internalType\":\"struct ICommon.ChainTransfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"confirmations\",\"type\":\"uint256\"}],\"name\":\"TransferToChain\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"dataDigest\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"indexed\":false,\"internalType\":\"struct ICommon.Erc20Transfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"relayerAddress\",\"type\":\"string\"}],\"name\":\"TransferToErc\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"validatorSetNonce\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"bridgeValidatorSetHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"governanceValidatorSetHash\",\"type\":\"bytes32\"}],\"name\":\"ValidatorSetUpdate\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"currentBridgeValidatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentGovernanceValidatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextBridgeValidatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextGovernanceValidatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"to\",\"type\":\"string\"}],\"internalType\":\"struct ICommon.ChainTransfer[]\",\"name\":\"_transfers\",\"type\":\"tuple[]\"},{\"internalType\":\"uint256\",\"name\":\"confirmations\",\"type\":\"uint256\"}],\"name\":\"transferToChain\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"transferToChainNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"validatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"validatorSetArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"signatures\",\"type\":\"tuple[]\"},{\"components\":[{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"dataDigest\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"internalType\":\"struct ICommon.Erc20Transfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\"},{\"internalType\":\"bytes32\",\"name\":\"poolRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\"},{\"internalType\":\"bool[]\",\"name\":\"proofFlags\",\"type\":\"bool[]\"},{\"internalType\":\"uint256\",\"name\":\"batchNonce\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"relayerAddress\",\"type\":\"string\"}],\"internalType\":\"struct ICommon.RelayProof\",\"name\":\"relayProof\",\"type\":\"tuple\"}],\"name\":\"transferToErc\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"transferToErc20Nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"validatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_currentValidatorSetArgs\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"_bridgeValidatorSetHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_governanceValidatorSetHash\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"}],\"name\":\"updateValidatorSet\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"validatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_currentValidatorSetArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"name\":\"upgrade\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"upgradeNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"validatorSetNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"validatorSet\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_currentValidatorSetArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"dataDigest\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"internalType\":\"struct ICommon.Erc20Transfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"withdrawNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]" ;
#[doc = "The parsed JSON ABI of the contract."]
pub static BRIDGE_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
    ::ethers_contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
pub struct Bridge<M>(::ethers_contract::Contract<M>);
impl<M> ::core::clone::Clone for Bridge<M> {
    fn clone(&self) -> Self {
        Self(::core::clone::Clone::clone(&self.0))
    }
}
impl<M> ::core::ops::Deref for Bridge<M> {
    type Target = ::ethers_contract::Contract<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M> ::core::ops::DerefMut for Bridge<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<M> ::core::fmt::Debug for Bridge<M> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!(Bridge))
            .field(&self.address())
            .finish()
    }
}
impl<M: ::ethers::providers::Middleware> Bridge<M> {
    #[doc = r" Creates a new contract instance with the specified `ethers` client at"]
    #[doc = r" `address`. The contract derefs to a `ethers::Contract` object."]
    pub fn new<T: Into<::ethers::core::types::Address>>(
        address: T,
        client: ::std::sync::Arc<M>,
    ) -> Self {
        Self(::ethers_contract::Contract::new(
            address.into(),
            BRIDGE_ABI.clone(),
            client,
        ))
    }
    #[doc = "Calls the contract's `currentBridgeValidatorSetHash` (0x7379d426) function"]
    pub fn current_bridge_validator_set_hash(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([115, 121, 212, 38], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `currentGovernanceValidatorSetHash` (0x0df6a752) function"]
    pub fn current_governance_validator_set_hash(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([13, 246, 167, 82], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `nextBridgeValidatorSetHash` (0xf5cd6594) function"]
    pub fn next_bridge_validator_set_hash(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([245, 205, 101, 148], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `nextGovernanceValidatorSetHash` (0x1c461a91) function"]
    pub fn next_governance_validator_set_hash(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([28, 70, 26, 145], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToChain` (0x224738eb) function"]
    pub fn transfer_to_chain(
        &self,
        transfers: ::std::vec::Vec<ChainTransfer>,
        confirmations: ::ethers::core::types::U256,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([34, 71, 56, 235], (transfers, confirmations))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToChainNonce` (0x21b1eb5f) function"]
    pub fn transfer_to_chain_nonce(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
        self.0
            .method_hash([33, 177, 235, 95], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToErc` (0x998c0030) function"]
    pub fn transfer_to_erc(
        &self,
        validator_set_args: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        relay_proof: RelayProof,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash(
                [153, 140, 0, 48],
                (validator_set_args, signatures, relay_proof),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToErc20Nonce` (0x5f9fd577) function"]
    pub fn transfer_to_erc_20_nonce(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
        self.0
            .method_hash([95, 159, 213, 119], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updateValidatorSet` (0xfd48739e) function"]
    pub fn update_validator_set(
        &self,
        current_validator_set_args: ValidatorSetArgs,
        bridge_validator_set_hash: [u8; 32],
        governance_validator_set_hash: [u8; 32],
        signatures: ::std::vec::Vec<Signature>,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash(
                [253, 72, 115, 158],
                (
                    current_validator_set_args,
                    bridge_validator_set_hash,
                    governance_validator_set_hash,
                    signatures,
                ),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `upgrade` (0xc3203d76) function"]
    pub fn upgrade(
        &self,
        current_validator_set_args: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        to: ::ethers::core::types::Address,
        nonce: ::ethers::core::types::U256,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash(
                [195, 32, 61, 118],
                (current_validator_set_args, signatures, to, nonce),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `upgradeNonce` (0x1af628b9) function"]
    pub fn upgrade_nonce(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
        self.0
            .method_hash([26, 246, 40, 185], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `validatorSetNonce` (0x486f2e46) function"]
    pub fn validator_set_nonce(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
        self.0
            .method_hash([72, 111, 46, 70], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `withdraw` (0x43d80041) function"]
    pub fn withdraw(
        &self,
        current_validator_set_args: ValidatorSetArgs,
        transfers: ::std::vec::Vec<Erc20Transfer>,
        signatures: ::std::vec::Vec<Signature>,
        nonce: ::ethers::core::types::U256,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash(
                [67, 216, 0, 65],
                (current_validator_set_args, transfers, signatures, nonce),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `withdrawNonce` (0xb8a4e151) function"]
    pub fn withdraw_nonce(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
        self.0
            .method_hash([184, 164, 225, 81], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Gets the contract's `TransferToChain` event"]
    pub fn transfer_to_chain_filter(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, TransferToChainFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `TransferToErc` event"]
    pub fn transfer_to_erc_filter(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, TransferToErcFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `ValidatorSetUpdate` event"]
    pub fn validator_set_update_filter(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ValidatorSetUpdateFilter> {
        self.0.event()
    }
    #[doc = r" Returns an `Event` builder for all the events of this contract."]
    pub fn events(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, BridgeEvents> {
        self.0
            .event_with_filter(::core::default::Default::default())
    }
}
impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for Bridge<M> {
    fn from(contract: ::ethers_contract::Contract<M>) -> Self {
        Self::new(contract.address(), contract.client())
    }
}
