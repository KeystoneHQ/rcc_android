#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignRequest {
    #[prost(uint32, tag="1")]
    pub seed_id: u32,
    #[prost(enumeration="sign_request::SignAlgo", tag="2")]
    pub algo: i32,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub port_name: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="7")]
    pub signing_option: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Solana {
    #[prost(oneof="solana::Method", tags="1, 2")]
    pub method: ::core::option::Option<solana::Method>,
}
/// Nested message and enum types in `Solana`.
pub mod solana {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParseTransaction),
        #[prost(message, tag="2")]
        VerifyMessage(super::VerifyMessage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessage {
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Near {
    #[prost(oneof="near::Method", tags="1")]
    pub method: ::core::option::Option<near::Method>,
}
/// Nested message and enum types in `Near`.
pub mod near {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParseTransaction),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polkadot {
    #[prost(oneof="polkadot::Method", tags="1, 2, 3, 4, 5, 6, 7")]
    pub method: ::core::option::Option<polkadot::Method>,
}
/// Nested message and enum types in `Polkadot`.
pub mod polkadot {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParsePolkadotTransaction),
        #[prost(message, tag="2")]
        InitPolkadotDb(super::InitialPolkadotDb),
        #[prost(message, tag="3")]
        GetPacketsTotal(super::GetPacketsTotal),
        #[prost(message, tag="4")]
        DecodeSequence(super::DecodeSequence),
        #[prost(message, tag="5")]
        HandleStub(super::HandleStub),
        #[prost(message, tag="6")]
        ImportAddress(super::ImportAddress),
        #[prost(message, tag="7")]
        GetSignContent(super::GetSignContent),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParsePolkadotTransaction {
    #[prost(string, tag="1")]
    pub transaction_data: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub db_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialPolkadotDb {
    #[prost(string, tag="1")]
    pub db_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPacketsTotal {
    #[prost(string, tag="1")]
    pub payload: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodeSequence {
    #[prost(string, repeated, tag="1")]
    pub payload: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleStub {
    #[prost(string, tag="1")]
    pub db_path: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub checksum: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAddress {
    #[prost(string, tag="1")]
    pub db_path: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub derivation_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignContent {
    #[prost(string, tag="1")]
    pub db_path: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub checksum: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aptos {
    #[prost(oneof="aptos::Method", tags="1")]
    pub method: ::core::option::Option<aptos::Method>,
}
/// Nested message and enum types in `Aptos`.
pub mod aptos {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParseTransaction),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cosmos {
    #[prost(oneof="cosmos::Method", tags="1")]
    pub method: ::core::option::Option<cosmos::Method>,
}
/// Nested message and enum types in `Cosmos`.
pub mod cosmos {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParseTransaction),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arweave {
    #[prost(oneof="arweave::Method", tags="1")]
    pub method: ::core::option::Option<arweave::Method>,
}
/// Nested message and enum types in `Arweave`.
pub mod arweave {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParseTransaction),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cardano {
    #[prost(oneof="cardano::Method", tags="1, 2")]
    pub method: ::core::option::Option<cardano::Method>,
}
/// Nested message and enum types in `Cardano`.
pub mod cardano {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="1")]
        ParseTransaction(super::ParseCardanoTransaction),
        #[prost(message, tag="2")]
        GenerateAddress(super::GenerateAddress),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseCardanoTransaction {
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub xpub: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub master_fingerprint: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub utxos: ::prost::alloc::vec::Vec<CardanoUtxo>,
    #[prost(message, repeated, tag="5")]
    pub cert_keys: ::prost::alloc::vec::Vec<CardanoCertKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardanoUtxo {
    #[prost(string, tag="1")]
    pub master_fingerprint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub value: u64,
    #[prost(string, tag="5")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardanoCertKey {
    #[prost(string, tag="1")]
    pub master_fingerprint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAddress {
    #[prost(string, tag="1")]
    pub xpub: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub index: u32,
    #[prost(uint32, tag="3")]
    pub t: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockChainRequest {
    #[prost(oneof="block_chain_request::Chain", tags="1, 2, 3, 4, 5, 6, 7")]
    pub chain: ::core::option::Option<block_chain_request::Chain>,
}
/// Nested message and enum types in `BlockChainRequest`.
pub mod block_chain_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chain {
        #[prost(message, tag="1")]
        Solana(super::Solana),
        #[prost(message, tag="2")]
        Near(super::Near),
        #[prost(message, tag="3")]
        Polkadot(super::Polkadot),
        #[prost(message, tag="4")]
        Aptos(super::Aptos),
        #[prost(message, tag="5")]
        Cosmos(super::Cosmos),
        #[prost(message, tag="6")]
        Arweave(super::Arweave),
        #[prost(message, tag="7")]
        Cardano(super::Cardano),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRsaPublicKeyRequest {
    #[prost(uint32, tag="1")]
    pub seed_id: u32,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub port_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupAdaRootKeyRequest {
    #[prost(uint32, tag="1")]
    pub seed_id: u32,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub passphrase: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub port_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdaExtendedPublicKeyRequest {
    #[prost(uint32, tag="1")]
    pub seed_id: u32,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub port_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(uint32, tag="1")]
    pub request_id: u32,
    #[prost(oneof="command_request::RequestData", tags="2, 3, 4, 5, 6")]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag="2")]
        SignRequest(super::SignRequest),
        #[prost(message, tag="3")]
        BlockChainRequest(super::BlockChainRequest),
        #[prost(message, tag="4")]
        GetRsaPublicKeyRequest(super::GetRsaPublicKeyRequest),
        #[prost(message, tag="5")]
        SetupAdaRootKeyRequest(super::SetupAdaRootKeyRequest),
        #[prost(message, tag="6")]
        GetAdaExtendedPublicKeyRequest(super::GetAdaExtendedPublicKeyRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(uint32, tag="1")]
    pub response_id: u32,
    #[prost(uint32, tag="2")]
    pub status: u32,
    #[prost(string, tag="3")]
    pub response: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub error_message: ::prost::alloc::string::String,
}
