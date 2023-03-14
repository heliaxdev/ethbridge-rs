#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use ::ethbridge_governance_events::*;
use ::ethbridge_structs::*;
# [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_version\",\"type\":\"uint8\"},{\"internalType\":\"address[]\",\"name\":\"_validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"_thresholdVotingPower\",\"type\":\"uint256\"},{\"internalType\":\"contract IProxy\",\"name\":\"_proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"}],\"name\":\"NewContract\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256[]\",\"name\":\"tokenCap\",\"type\":\"uint256[]\"}],\"name\":\"UpdateBridgeWhitelist\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"}],\"name\":\"UpgradedContract\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"validatorSetNonce\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"bridgeValidatoreSetHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"governanceValidatoreSetHash\",\"type\":\"bytes32\"}],\"name\":\"ValidatorSetUpdate\",\"type\":\"event\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_validators\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"addContract\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_currentValidatorSetArgs\",\"type\":\"tuple\"},{\"internalType\":\"address[]\",\"name\":\"_tokens\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_tokensCap\",\"type\":\"uint256[]\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"}],\"name\":\"updateBridgeWhitelist\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_currentValidatorSetArgs\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"_bridgeValidatorSetHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_governanceValidatorSetHash\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"name\":\"updateValidatorsSet\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_validators\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"upgradeBridgeContract\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_validators\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"upgradeContract\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"validatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"validatorSetNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"whitelistNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]" ;
#[doc = "The parsed JSON ABI of the contract."]
pub static GOVERNANCE_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
    ::ethers_contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
pub struct Governance<M>(::ethers_contract::Contract<M>);
impl<M> ::core::clone::Clone for Governance<M> {
    fn clone(&self) -> Self {
        Self(::core::clone::Clone::clone(&self.0))
    }
}
impl<M> ::core::ops::Deref for Governance<M> {
    type Target = ::ethers_contract::Contract<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M> ::core::ops::DerefMut for Governance<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<M> ::core::fmt::Debug for Governance<M> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!(Governance))
            .field(&self.address())
            .finish()
    }
}
impl<M: ::ethers::providers::Middleware> Governance<M> {
    #[doc = r" Creates a new contract instance with the specified `ethers` client at"]
    #[doc = r" `address`. The contract derefs to a `ethers::Contract` object."]
    pub fn new<T: Into<::ethers::core::types::Address>>(
        address: T,
        client: ::std::sync::Arc<M>,
    ) -> Self {
        Self(::ethers_contract::Contract::new(
            address.into(),
            GOVERNANCE_ABI.clone(),
            client,
        ))
    }
    #[doc = "Calls the contract's `addContract` (0x0f91cffd) function"]
    pub fn add_contract(
        &self,
        validators: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        name: ::std::string::String,
        address: ::ethers::core::types::Address,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([15, 145, 207, 253], (validators, signatures, name, address))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updateBridgeWhitelist` (0x9d8dcf32) function"]
    pub fn update_bridge_whitelist(
        &self,
        current_validator_set_args: ValidatorSetArgs,
        tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        tokens_cap: ::std::vec::Vec<::ethers::core::types::U256>,
        signatures: ::std::vec::Vec<Signature>,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash(
                [157, 141, 207, 50],
                (current_validator_set_args, tokens, tokens_cap, signatures),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updateValidatorsSet` (0x06d66449) function"]
    pub fn update_validators_set(
        &self,
        current_validator_set_args: ValidatorSetArgs,
        bridge_validator_set_hash: [u8; 32],
        governance_validator_set_hash: [u8; 32],
        signatures: ::std::vec::Vec<Signature>,
        nonce: ::ethers::core::types::U256,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash(
                [6, 214, 100, 73],
                (
                    current_validator_set_args,
                    bridge_validator_set_hash,
                    governance_validator_set_hash,
                    signatures,
                    nonce,
                ),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `upgradeBridgeContract` (0xba07df0d) function"]
    pub fn upgrade_bridge_contract(
        &self,
        validators: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        address: ::ethers::core::types::Address,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([186, 7, 223, 13], (validators, signatures, address))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `upgradeContract` (0x1e3fc46d) function"]
    pub fn upgrade_contract(
        &self,
        validators: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        name: ::std::string::String,
        address: ::ethers::core::types::Address,
    ) -> ::ethers_contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([30, 63, 196, 109], (validators, signatures, name, address))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `validatorSetHash` (0xcdea2912) function"]
    pub fn validator_set_hash(&self) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([205, 234, 41, 18], ())
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
    #[doc = "Calls the contract's `whitelistNonce` (0xb5c3030b) function"]
    pub fn whitelist_nonce(
        &self,
    ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
        self.0
            .method_hash([181, 195, 3, 11], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Gets the contract's `NewContract` event"]
    pub fn new_contract_filter(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, NewContractFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `UpdateBridgeWhitelist` event"]
    pub fn update_bridge_whitelist_filter(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, UpdateBridgeWhitelistFilter>
    {
        self.0.event()
    }
    #[doc = "Gets the contract's `UpgradedContract` event"]
    pub fn upgraded_contract_filter(
        &self,
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, UpgradedContractFilter> {
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
    ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, GovernanceEvents> {
        self.0
            .event_with_filter(::core::default::Default::default())
    }
}
impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for Governance<M> {
    fn from(contract: ::ethers_contract::Contract<M>) -> Self {
        Self::new(contract.address(), contract.client())
    }
}
