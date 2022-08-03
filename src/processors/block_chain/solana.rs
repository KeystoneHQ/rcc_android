use rcc_trait_chain::Chain;
use crate::{CommandResponse, Solana};
use crate::processors::block_chain::error::ParseError;
use crate::processors::block_chain::error::ParseError::SolanaParseError;
use crate::solana::Method;

pub fn process(data: Solana) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            let message = hex::decode(data).map_err(|e| SolanaParseError(format!("decode hex failed")))?;
            rcc_solana::Sol::parse(&message).map_err(|e| SolanaParseError(format!("invalid transaction, {}", e.to_string())))
        }
        Some(Method::VerifyMessage(data)) => {
            let mut message = hex::decode(data).map_err(|e| SolanaParseError(format!("decode hex failed")))?;
            match rcc_solana::Sol::validate_message(&mut message) {
                true => Ok(format!("transaction")),
                false => Ok(format!("message")),
            }
        }
        None => {
            Err(SolanaParseError(format!("Invalid data")))
        }
    }
}
