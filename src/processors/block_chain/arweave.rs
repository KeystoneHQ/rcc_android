use crate::{CommandResponse, Arweave};
use crate::arweave::Method;
use rcc_trait_chain::Chain;
use rcc_cosmos::Cosmos as CosmosParser;
use rcc_arweave::Arweave as ArweaveParser;
use crate::processors::block_chain::error::ParseError;

pub fn process(data: Arweave) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            parse_transaction(data.data)
        }
        _ => Err(ParseError::ArweaveParseError("message type is not supported".to_string()))
    }
}

fn parse_transaction(data: String) -> Result<String, ParseError> {
    let data = match hex::decode(data) {
        Ok(data) => data,
        Err(_) =>
            return Err(ParseError::ArweaveParseError("decode hex failed".to_string()))
    };
    ArweaveParser::parse(&data).map_err(|e| ParseError::CosmosParseError(e.to_string()))
}