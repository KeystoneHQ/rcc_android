#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignRequest {
    #[prost(uint32, tag = "1")]
    pub seed_id: u32,
    #[prost(enumeration = "sign_request::SignAlgo", tag = "2")]
    pub algo: i32,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub port_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SignRequest`.
pub mod sign_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SignAlgo {
        Secp256k1 = 0,
        Secp256R1 = 1,
        Ed25519 = 2,
        Rsa = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseTransaction {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Solana {
    #[prost(oneof = "solana::Method", tags = "1, 2")]
    pub method: ::core::option::Option<solana::Method>,
}
/// Nested message and enum types in `Solana`.
pub mod solana {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag = "1")]
        ParseTransaction(super::ParseTransaction),
        #[prost(message, tag = "2")]
        VerifyMessage(super::VerifyMessage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessage {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockChainRequest {
    #[prost(oneof = "block_chain_request::Chain", tags = "1")]
    pub chain: ::core::option::Option<block_chain_request::Chain>,
}
/// Nested message and enum types in `BlockChainRequest`.
pub mod block_chain_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chain {
        #[prost(message, tag = "1")]
        Solana(super::Solana),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    #[prost(oneof = "command_request::RequestData", tags = "2, 3")]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag = "2")]
        SignRequest(super::SignRequest),
        #[prost(message, tag = "3")]
        BlockChainRequest(super::BlockChainRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(uint32, tag = "1")]
    pub response_id: u32,
    #[prost(uint32, tag = "2")]
    pub status: u32,
    #[prost(string, tag = "3")]
    pub response: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
