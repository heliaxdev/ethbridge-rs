#![allow(dead_code)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[doc = "`ChainTransfer(uint256,address,string)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct ChainTransfer {
    pub amount: ::ethabi::ethereum_types::U256,
    pub from: ::ethabi::ethereum_types::Address,
    pub to: ::std::string::String,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[doc = "`Erc20Transfer(bytes32,uint256,address,address)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Erc20Transfer {
    pub data_digest: [u8; 32],
    pub amount: ::ethabi::ethereum_types::U256,
    pub from: ::ethabi::ethereum_types::Address,
    pub to: ::ethabi::ethereum_types::Address,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[doc = "`RelayProof((bytes32,uint256,address,address)[],bytes32,bytes32[],bool[],uint256,string)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct RelayProof {
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub pool_root: [u8; 32],
    pub proof: ::std::vec::Vec<[u8; 32]>,
    pub proof_flags: ::std::vec::Vec<bool>,
    pub batch_nonce: ::ethabi::ethereum_types::U256,
    pub relayer_address: ::std::string::String,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[doc = "`Signature(bytes32,bytes32,uint8)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: u8,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[doc = "`ValidatorSetArgs(bytes32[],uint256)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct ValidatorSetArgs {
    pub validator_set: ::std::vec::Vec<[u8; 32]>,
    pub nonce: ::ethabi::ethereum_types::U256,
}
