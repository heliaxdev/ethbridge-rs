#![allow(dead_code)]
#[doc = "`Erc20Transfer(address,address,uint256,string,uint256,string)`"]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiCodec))]
pub struct Erc20Transfer {
    pub from: ::ethabi::ethereum_types::Address,
    pub to: ::ethabi::ethereum_types::Address,
    pub amount: ::ethabi::ethereum_types::U256,
    pub fee_from: String,
    pub fee: ::ethabi::ethereum_types::U256,
    pub sender: String,
}
#[doc = "`NamadaTransfer(address,uint256,string)`"]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiCodec))]
pub struct NamadaTransfer {
    pub from: ::ethabi::ethereum_types::Address,
    pub amount: ::ethabi::ethereum_types::U256,
    pub to: String,
}
#[doc = "`RelayProof((address[],uint256[],uint256),(bytes32,bytes32,uint8)[],(address,address,uint256,string,uint256,string)[],bytes32,bytes32[],bool[],uint256,string)`"]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiCodec))]
pub struct RelayProof {
    pub validator_set_args: ValidatorSetArgs,
    pub signatures: ::std::vec::Vec<Signature>,
    pub transfers: ::std::vec::Vec<Erc20Transfer>,
    pub pool_root: [u8; 32],
    pub proof: Vec<[u8; 32]>,
    pub proof_flags: Vec<bool>,
    pub batch_nonce: ::ethabi::ethereum_types::U256,
    pub relayer_address: String,
}
#[doc = "`Signature(bytes32,bytes32,uint8)`"]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiCodec))]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: u8,
}
#[doc = "`ValidatorSetArgs(address[],uint256[],uint256)`"]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiType))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers::contract::EthAbiCodec))]
pub struct ValidatorSetArgs {
    pub validators: Vec<::ethabi::ethereum_types::Address>,
    pub powers: Vec<::ethabi::ethereum_types::U256>,
    pub nonce: ::ethabi::ethereum_types::U256,
}
