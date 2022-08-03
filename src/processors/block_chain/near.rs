use crate::{CommandResponse, Near};
use rcc_near::Near as NearParser;
use crate::near::Method;
use rcc_trait_chain::Chain;
use crate::processors::block_chain::error::ParseError;

pub fn process(data: Near) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            parse_transaction(data.data)
        }
        _ => Err(ParseError::NearParseError("message type is not supported".to_string()))
    }
}

fn parse_transaction(data: String) -> Result<String, ParseError> {
    let data = match hex::decode(data) {
        Ok(data) => data,
        Err(_) =>
            return Err(ParseError::NearParseError("decode hex failed".to_string()))
    };
    NearParser::parse(&data).map_err(|e| ParseError::NearParseError(e.to_string()))
}