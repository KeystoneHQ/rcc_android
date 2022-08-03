use crate::block_chain_request::Chain;
use crate::{BlockChainRequest, CommandResponse};
use crate::processors::block_chain::error::ParseError;

mod solana;
mod near;
mod error;

pub fn process(params: BlockChainRequest) -> Result<String, ParseError> {
    match params.chain {
        Some(Chain::Solana(solana)) => solana::process(solana),
        Some(Chain::Near(near)) => near::process(near),
        None => {
            unimplemented!()
        }
    }
}