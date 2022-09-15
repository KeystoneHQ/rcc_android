use rcc_trait_chain::Chain;
use crate::{Polkadot};
use crate::processors::block_chain::error::ParseError::{ChainParseError};
use crate::polkadot::Method;
use crate::processors::block_chain::error::ParseError;
use rcc_polkadot;

pub fn process(data: Polkadot) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(data)) => {
            Ok(rcc_polkadot::transaction_parser::parse_transaction(data.transaction_data, data.db_path))
        }
        Some(Method::InitPolkadotDb(data)) => {
            Ok(rcc_polkadot::init_polkadot_db(data.db_path))
        }
        Some(Method::GetPacketsTotal(data)) => {
            Ok(rcc_polkadot::scanner::get_packets_total(data.payload))
        }
        Some(Method::DecodeSequence(data)) => {
            Ok(rcc_polkadot::scanner::decode_sequence(data.payload))
        }
        None => {
            Err(ChainParseError(format!("Polkadot") , format!("Invalid data")))
        }
    }
}
