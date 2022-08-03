use crate::{CommandResponse, Near};
use rcc_near::Near as NearParser;
use crate::near::Method;
use rcc_trait_chain::Chain;

pub fn process(data: Near, request_id: u32) -> CommandResponse {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            parse_transaction(data.data, request_id)
        }
        _ => return CommandResponse::error(
            request_id,
            "message type is not supported".to_string(),
        )
    }
}

fn parse_transaction(data: String, request_id: u32) -> CommandResponse {
    let data = match hex::decode(data) {
        Ok(data) => data,
        Err(_) => return CommandResponse::error(
            request_id,
            "decode hex failed".to_string(),
        )
    };
    match NearParser::parse(&data) {
        Ok(result) => {
            CommandResponse::success(request_id, result)
        }
        Err(e) => CommandResponse::error(
            request_id,
            e.to_string(),
        )
    }
}