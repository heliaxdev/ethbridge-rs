#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use ::ethbridge_governance_events::*;
use ::ethbridge_structs::*;
# [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NewContract\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"nonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256[]\",\n        \"name\": \"tokenCap\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"UpdateBridgeWhitelist\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"UpgradedContract\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"validatorSetNonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"bridgeValidatoreSetHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"governanceValidatoreSetHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"ValidatorSetUpdate\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"validators\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"powers\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"nonce\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n        \"name\": \"validators\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.Signature[]\",\n        \"name\": \"signatures\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"validators\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"powers\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"nonce\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n        \"name\": \"currentValidatorSetArgs\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"tokensCap\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.Signature[]\",\n        \"name\": \"signatures\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"updateBridgeWhitelist\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"validators\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"powers\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"nonce\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n        \"name\": \"currentValidatorSetArgs\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"bridgeValidatorSetHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"governanceValidatorSetHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.Signature[]\",\n        \"name\": \"signatures\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"nonce\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"updateValidatorsSet\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"validators\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"powers\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"nonce\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n        \"name\": \"_validators\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.Signature[]\",\n        \"name\": \"_signatures\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"upgradeBridgeContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"validators\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"powers\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"nonce\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n        \"name\": \"validators\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.Signature[]\",\n        \"name\": \"signatures\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"upgradeContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ;
#[doc = r" The parsed JSON-ABI of the contract."]
pub static GOVERNANCE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
    ethers::contract::Lazy::new(|| {
        ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
pub struct Governance<M>(ethers::contract::Contract<M>);
impl<M> Clone for Governance<M> {
    fn clone(&self) -> Self {
        Governance(self.0.clone())
    }
}
impl<M> std::ops::Deref for Governance<M> {
    type Target = ethers::contract::Contract<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M> std::fmt::Debug for Governance<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple(stringify!(Governance))
            .field(&self.address())
            .finish()
    }
}
impl<M: ethers::providers::Middleware> Governance<M> {
    #[doc = r" Creates a new contract instance with the specified `ethers`"]
    #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
    #[doc = r" object"]
    pub fn new<T: Into<ethers::core::types::Address>>(
        address: T,
        client: ::std::sync::Arc<M>,
    ) -> Self {
        ethers::contract::Contract::new(address.into(), GOVERNANCE_ABI.clone(), client).into()
    }
    #[doc = "Calls the contract's `addContract` (0x0f91cffd) function"]
    pub fn add_contract(
        &self,
        validators: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        name: String,
        addr: ethers::core::types::Address,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([15, 145, 207, 253], (validators, signatures, name, addr))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updateBridgeWhitelist` (0x9d8dcf32) function"]
    pub fn update_bridge_whitelist(
        &self,
        current_validator_set_args: ValidatorSetArgs,
        tokens: ::std::vec::Vec<ethers::core::types::Address>,
        tokens_cap: ::std::vec::Vec<ethers::core::types::U256>,
        signatures: ::std::vec::Vec<Signature>,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        nonce: ethers::core::types::U256,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        address: ethers::core::types::Address,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([186, 7, 223, 13], (validators, signatures, address))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `upgradeContract` (0x1e3fc46d) function"]
    pub fn upgrade_contract(
        &self,
        validators: ValidatorSetArgs,
        signatures: ::std::vec::Vec<Signature>,
        name: String,
        addr: ethers::core::types::Address,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([30, 63, 196, 109], (validators, signatures, name, addr))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Gets the contract's `NewContract` event"]
    pub fn new_contract_filter(&self) -> ethers::contract::builders::Event<M, NewContractFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `UpdateBridgeWhitelist` event"]
    pub fn update_bridge_whitelist_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, UpdateBridgeWhitelistFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `UpgradedContract` event"]
    pub fn upgraded_contract_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, UpgradedContractFilter> {
        self.0.event()
    }
    #[doc = "Gets the contract's `ValidatorSetUpdate` event"]
    pub fn validator_set_update_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, ValidatorSetUpdateFilter> {
        self.0.event()
    }
    #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
    pub fn events(&self) -> ethers::contract::builders::Event<M, GovernanceEvents> {
        self.0.event_with_filter(Default::default())
    }
}
impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Governance<M> {
    fn from(contract: ethers::contract::Contract<M>) -> Self {
        Self(contract)
    }
}
