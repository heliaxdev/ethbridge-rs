#![allow(dead_code)]
#[doc = "`Erc20Transfer(address,address,uint256,string,uint256,string)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
pub struct Erc20Transfer {
    pub from: ::ethabi::ethereum_types::Address,
    pub to: ::ethabi::ethereum_types::Address,
    pub amount: ::ethabi::ethereum_types::U256,
    pub fee_from: ::std::string::String,
    pub fee: ::ethabi::ethereum_types::U256,
    pub sender: ::std::string::String,
}
#[doc = "`NamadaTransfer(address,uint256,string)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
pub struct NamadaTransfer {
    pub from: ::ethabi::ethereum_types::Address,
    pub amount: ::ethabi::ethereum_types::U256,
    pub to: ::std::string::String,
}
#[doc = "`RelayProof((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],(address,address,uint256,string,uint256,string)[],bytes32,bytes32[],bool[],uint256,string)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
pub struct RelayProof {
    pub validator_set_args: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub pool_root: [u8; 32],
    pub proof: ::std::vec::Vec<[u8; 32]>,
    pub proof_flags: ::std::vec::Vec<bool>,
    pub batch_nonce: ::ethabi::ethereum_types::U256,
    pub relayer_address: ::std::string::String,
}
#[doc = "`Signature(bytes32,bytes32,uint8)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: u8,
}
#[doc = "`ValidatorSetArgs(address[],uint256[],uint256)`"]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
pub struct ValidatorSetArgs {
    pub validators: ::std::vec::Vec<::ethabi::ethereum_types::Address>,
    pub powers: ::std::vec::Vec<::ethabi::ethereum_types::U256>,
    pub nonce: ::ethabi::ethereum_types::U256,
}
