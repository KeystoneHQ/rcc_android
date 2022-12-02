use crate::{CommandResponse, Cosmos};
use rcc_cosmos::Cosmos as CosmosParser;
use crate::cosmos::Method;
use rcc_trait_chain::Chain;
use crate::processors::block_chain::error::ParseError;

pub fn process(data: Cosmos) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            parse_transaction(data.data)
        }
        _ => Err(ParseError::CosmosParseError("message type is not supported".to_string()))
    }
}

fn parse_transaction(data: String) -> Result<String, ParseError> {
    let data = match hex::decode(data) {
        Ok(data) => data,
        Err(_) =>
            return Err(ParseError::CosmosParseError("decode hex failed".to_string()))
    };
    CosmosParser::parse(&data).map_err(|e| ParseError::CosmosParseError(e.to_string()))
}