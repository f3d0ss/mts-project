///`Chain(string,uint256,string,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Chain {
    pub name: ::std::string::String,
    pub chain_id: ::ethers::core::types::U256,
    pub chain_alias: ::std::string::String,
    pub rpc_url: ::std::string::String,
}
