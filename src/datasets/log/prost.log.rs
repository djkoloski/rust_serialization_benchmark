#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(uint32, tag = "1")]
    pub x0: u32,
    #[prost(uint32, tag = "2")]
    pub x1: u32,
    #[prost(uint32, tag = "3")]
    pub x2: u32,
    #[prost(uint32, tag = "4")]
    pub x3: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub userid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub request: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub code: u32,
    #[prost(uint64, tag = "7")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Logs {
    #[prost(message, repeated, tag = "1")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
