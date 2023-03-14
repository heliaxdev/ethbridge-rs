#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use ::ethbridge_bridge_events::*;
use ::ethbridge_structs::*;
# [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_version\",\"type\":\"uint8\"},{\"internalType\":\"address[]\",\"name\":\"_currentValidators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_currentPowers\",\"type\":\"uint256[]\"},{\"internalType\":\"address[]\",\"name\":\"_nextValidators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_nextPowers\",\"type\":\"uint256[]\"},{\"internalType\":\"address[]\",\"name\":\"_tokenList\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_tokenCap\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"_thresholdVotingPower\",\"type\":\"uint256\"},{\"internalType\":\"contract IProxy\",\"name\":\"_proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"to\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"InvalidTransferToNamada\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"feeFrom\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"sender\",\"type\":\"string\"}],\"indexed\":false,\"internalType\":\"struct ICommon.Erc20Transfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\"},{\"indexed\":false,\"internalType\":\"bool[]\",\"name\":\"validMap\",\"type\":\"bool[]\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"relayerAddress\",\"type\":\"string\"}],\"name\":\"TransferToErc\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"to\",\"type\":\"string\"}],\"indexed\":false,\"internalType\":\"struct ICommon.NamadaTransfer[]\",\"name\":\"trasfers\",\"type\":\"tuple[]\"},{\"indexed\":false,\"internalType\":\"bool[]\",\"name\":\"validMap\",\"type\":\"bool[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"confirmations\",\"type\":\"uint256\"}],\"name\":\"TransferToNamada\",\"type\":\"event\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"_validatorSetArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"_signatures\",\"type\":\"tuple[]\"},{\"internalType\":\"bytes32\",\"name\":\"_message\",\"type\":\"bytes32\"}],\"name\":\"authorize\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentValidatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"}],\"name\":\"getWhitelistAmountFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextValidatorSetHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"internalType\":\"struct ICommon.ValidatorSetArgs\",\"name\":\"validatorSetArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"internalType\":\"struct ICommon.Signature[]\",\"name\":\"signatures\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"feeFrom\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"sender\",\"type\":\"string\"}],\"internalType\":\"struct ICommon.Erc20Transfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\"},{\"internalType\":\"bytes32\",\"name\":\"poolRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\"},{\"internalType\":\"bool[]\",\"name\":\"proofFlags\",\"type\":\"bool[]\"},{\"internalType\":\"uint256\",\"name\":\"batchNonce\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"relayerAddress\",\"type\":\"string\"}],\"internalType\":\"struct ICommon.RelayProof\",\"name\":\"relayProof\",\"type\":\"tuple\"}],\"name\":\"transferToErc\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"to\",\"type\":\"string\"}],\"internalType\":\"struct ICommon.NamadaTransfer[]\",\"name\":\"_transfers\",\"type\":\"tuple[]\"},{\"internalType\":\"uint256\",\"name\":\"confirmations\",\"type\":\"uint256\"}],\"name\":\"transferToNamada\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_tokens\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_tokensCap\",\"type\":\"uint256[]\"}],\"name\":\"updateTokenWhitelist\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_validatorSetHash\",\"type\":\"bytes32\"}],\"name\":\"updateValidatorSetHash\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]\n" ;
#[doc = r" The parsed JSON-ABI of the contract."]
pub static BRIDGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
    ethers::contract::Lazy::new(|| {
        ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
pub struct Bridge<M>(ethers::contract::Contract<M>);
impl<M> Clone for Bridge<M> {
    fn clone(&self) -> Self {
        Bridge(self.0.clone())
    }
}
impl<M> std::ops::Deref for Bridge<M> {
    type Target = ethers::contract::Contract<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M> std::fmt::Debug for Bridge<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple(stringify!(Bridge))
            .field(&self.address())
            .finish()
    }
}
impl<M: ethers::providers::Middleware> Bridge<M> {
    #[doc = r" Creates a new contract instance with the specified `ethers`"]
    #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
    #[doc = r" object"]
    pub fn new<T: Into<ethers::core::types::Address>>(
        address: T,
        client: ::std::sync::Arc<M>,
    ) -> Self {
        ethers::contract::Contract::new(address.into(), BRIDGE_ABI.clone(), client).into()
    }
    #[doc = "Calls the contract's `authorize` (0x1a8cbe45) function"]
    pub fn authorize(
        &self,
        validator_set_args: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        message: [u8; 32],
    ) -> ethers::contract::builders::ContractCall<M, bool> {
        self.0
            .method_hash(
                [26, 140, 190, 69],
                (validator_set_args, signatures, message),
            )
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `currentValidatorSetHash` (0xf896f1a5) function"]
    pub fn current_validator_set_hash(
        &self,
    ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([248, 150, 241, 165], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `getWhitelistAmountFor` (0x62a2599f) function"]
    pub fn get_whitelist_amount_for(
        &self,
        token_address: ethers::core::types::Address,
    ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
        self.0
            .method_hash([98, 162, 89, 159], token_address)
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `nextValidatorSetHash` (0x752d3b89) function"]
    pub fn next_validator_set_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
        self.0
            .method_hash([117, 45, 59, 137], ())
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToErc` (0xd251761a) function"]
    pub fn transfer_to_erc(
        &self,
        relay_proof: RelayProof,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([210, 81, 118, 26], (relay_proof,))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToNamada` (0x072e77cb) function"]
    pub fn transfer_to_namada(
        &self,
        transfers: ::std::vec::Vec<NamadaTransfer>,
        confirmations: ethers::core::types::U256,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([7, 46, 119, 203], (transfers, confirmations))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updateTokenWhitelist` (0x6980dd15) function"]
    pub fn update_token_whitelist(
        &self,
        tokens: ::std::vec::Vec<ethers::core::types::Address>,
        tokens_cap: ::std::vec::Vec<ethers::core::types::U256>,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([105, 128, 221, 21], (tokens, tokens_cap))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updateValidatorSetHash` (0x15d409c6) function"]
    pub fn update_validator_set_hash(
        &self,
        validator_set_hash: [u8; 32],
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([21, 212, 9, 198], validator_set_hash)
            .expect("method not found (this should never happen)")
    }
    #[doc = "Gets the contract's `InvalidTransferToNamada` event"]
    pub fn invalid_transfer_to_namada_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, InvalidTransferToNamadaFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `TransferToErc` event"]
    pub fn transfer_to_erc_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, TransferToErcFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `TransferToNamada` event"]
    pub fn transfer_to_namada_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, TransferToNamadaFilter> {
        self.0.event()
    }
    #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
    pub fn events(&self) -> ethers::contract::builders::Event<M, BridgeEvents> {
        self.0.event_with_filter(Default::default())
    }
}
impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Bridge<M> {
    fn from(contract: ethers::contract::Contract<M>) -> Self {
        Self(contract)
    }
}
