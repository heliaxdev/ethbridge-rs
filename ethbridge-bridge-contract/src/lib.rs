#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use ::ethbridge_bridge_events::*;
use ::ethbridge_structs::*;
# [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"string\",\n        \"name\": \"to\",\n        \"type\": \"string\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"InvalidTransferToNamada\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"nonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"from\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"to\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"feeFrom\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fee\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"sender\",\n            \"type\": \"string\"\n          }\n        ],\n        \"indexed\": false,\n        \"internalType\": \"struct ICommon.ERC20Transfer[]\",\n        \"name\": \"transfers\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"string\",\n        \"name\": \"relayerAddress\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"TransferToERC\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"nonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"from\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"to\",\n            \"type\": \"string\"\n          }\n        ],\n        \"indexed\": false,\n        \"internalType\": \"struct ICommon.NamadaTransfer[]\",\n        \"name\": \"trasfers\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"confirmations\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"TransferToNamada\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"validators\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"powers\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"nonce\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n        \"name\": \"validatorSetArgs\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.Signature[]\",\n        \"name\": \"signatures\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"message\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"authorize\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address[]\",\n                \"name\": \"validators\",\n                \"type\": \"address[]\"\n              },\n              {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"powers\",\n                \"type\": \"uint256[]\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"nonce\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct ICommon.ValidatorSetArgs\",\n            \"name\": \"validatorSetArgs\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n              }\n            ],\n            \"internalType\": \"struct ICommon.Signature[]\",\n            \"name\": \"signatures\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"string\",\n                \"name\": \"feeFrom\",\n                \"type\": \"string\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"fee\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"string\",\n                \"name\": \"sender\",\n                \"type\": \"string\"\n              }\n            ],\n            \"internalType\": \"struct ICommon.ERC20Transfer[]\",\n            \"name\": \"transfers\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"poolRoot\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32[]\",\n            \"name\": \"proof\",\n            \"type\": \"bytes32[]\"\n          },\n          {\n            \"internalType\": \"bool[]\",\n            \"name\": \"proofFlags\",\n            \"type\": \"bool[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"batchNonce\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"relayerAddress\",\n            \"type\": \"string\"\n          }\n        ],\n        \"internalType\": \"struct IBridge.RelayProof\",\n        \"name\": \"relayProof\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"transferToERC\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"from\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"to\",\n            \"type\": \"string\"\n          }\n        ],\n        \"internalType\": \"struct ICommon.NamadaTransfer[]\",\n        \"name\": \"trasfers\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"confirmations\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferToNamada\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"tokensCap\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"updateTokenWhitelist\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_validatorSetHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"updateValidatorSetHash\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ;
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
    #[doc = "Calls the contract's `transferToERC` (0xbaf4770c) function"]
    pub fn transfer_to_erc(
        &self,
        relay_proof: RelayProof,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([186, 244, 119, 12], (relay_proof,))
            .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `transferToNamada` (0x072e77cb) function"]
    pub fn transfer_to_namada(
        &self,
        trasfers: ::std::vec::Vec<NamadaTransfer>,
        confirmations: ethers::core::types::U256,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
        self.0
            .method_hash([7, 46, 119, 203], (trasfers, confirmations))
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
    #[doc = "Gets the contract's `TransferToERC` event"]
    pub fn transfer_to_erc_filter(
        &self,
    ) -> ethers::contract::builders::Event<M, TransferToERCFilter> {
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
