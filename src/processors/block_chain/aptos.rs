use crate::{CommandResponse, Aptos};
use rcc_aptos::Aptos as AptosParser;
use crate::aptos::Method;
use rcc_trait_chain::Chain;
use crate::processors::block_chain::error::ParseError;

pub fn process(data: Aptos) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            parse_transaction(data.data)
        }
        _ => Err(ParseError::AptosParseError("message type is not supported".to_string()))
    }
}

fn parse_transaction(data: String) -> Result<String, ParseError> {
    let data = match hex::decode(data) {
        Ok(data) => data,
        Err(_) =>
            return Err(ParseError::AptosParseError("decode hex failed".to_string()))
    };
    AptosParser::parse(&data).map_err(|e| ParseError::AptosParseError(e.to_string()))
}