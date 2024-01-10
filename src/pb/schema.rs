// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contracts {
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub token_0: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_1: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pool: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferInfos {
    #[prost(message, repeated, tag="1")]
    pub transfer_infos: ::prost::alloc::vec::Vec<TransferInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferInfo {
    #[prost(string, tag="1")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushiWethPools {
    #[prost(message, repeated, tag="1")]
    pub sushi_weth_pools: ::prost::alloc::vec::Vec<SushiWethPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushiWethPool {
    #[prost(string, tag="1")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub topic_2: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub weth_amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_num: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
