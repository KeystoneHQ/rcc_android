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
        Some(Method::HandleStub(data)) => {
            Ok(rcc_polkadot::handle_stub(data.db_path, data.checksum))
        }
        Some(Method::ImportAddress(data)) => {
            Ok(rcc_polkadot::import_address(data.db_path, data.public_key, data.derivation_path))
        }
        Some(Method::GetSignContent(data)) => {
            Ok(rcc_polkadot::get_sign_content(data.db_path, data.checksum))
        }
        None => {
            Err(ChainParseError(format!("Polkadot") , format!("Invalid data")))
        }
    }
}
