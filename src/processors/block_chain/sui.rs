use crate::processors::block_chain::error::ParseError;
use crate::sui::Method;
use crate::Sui;

enum ParseType {
    Transaction,
    Message
}

pub fn process(data: Sui) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => parse(ParseType::Transaction, data.data),
        Some(Method::ParseMessage(data)) => parse(ParseType::Message, data.data),
        _ => Err(ParseError::SuiParseError(
            "message type is not supported".to_string(),
        )),
    }
}

fn parse(t: ParseType, data: String) -> Result<String, ParseError> {
    let data = match hex::decode(data) {
        Ok(data) => data,
        Err(_) => return Err(ParseError::SuiParseError("decode hex failed".to_string())),
    };
    match t {
        ParseType::Transaction => {
             let result = rcc_sui::parse_tx(data).map_err(|e| ParseError::SuiParseError(e.to_string()))?;
             serde_json::to_string(&result).map_err(|e| ParseError::SuiParseError(e.to_string()))
        }
        ParseType::Message => {
            let result = rcc_sui::parse_msg(data).map_err(|e| ParseError::SuiParseError(e.to_string()))?;
            serde_json::to_string(&result).map_err(|e| ParseError::SuiParseError(e.to_string()))
        }
    }
}
