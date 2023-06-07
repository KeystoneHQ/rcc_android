use hex::FromHexError;
use rcc_cardano::errors::CardanoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("near parse failed, reason: `{0}`")]
    NearParseError(String),
    #[error("solana parse failed, reason: `{0}`")]
    SolanaParseError(String),
    #[error("`{0}` parse failed, reason: `{1}`")]
    ChainParseError(String, String),
    #[error("aptos parse failed, reason: `{0}`")]
    AptosParseError(String),
    #[error("cosmos parse failed, reason: `{0}`")]
    CosmosParseError(String),
    #[error("arweave parse failed, reason: `{0}`")]
    ArweaveParseError(String),
    #[error("cardano parse failed, reason: `{0}`")]
    CardanoParseError(String),
    #[error("invalid hex text, reason: `{0}`")]
    HexEncodingError(String),
}

impl From<FromHexError> for ParseError {
    fn from(value: FromHexError) -> Self {
        ParseError::HexEncodingError(value.to_string())
    }
}

impl From<CardanoError> for ParseError {
    fn from(value: CardanoError) -> Self {
        ParseError::CardanoParseError(value.to_string())
    }
}