use crate::block_chain_request::Chain;
use crate::{BlockChainRequest, CommandResponse};
use crate::processors::block_chain::error::ParseError;

mod solana;
mod near;
mod error;
mod polkadot;
mod aptos;
mod cosmos;
mod arweave;

pub fn process(params: BlockChainRequest) -> Result<String, ParseError> {
    match params.chain {
        Some(Chain::Solana(solana)) => solana::process(solana),
        Some(Chain::Near(near)) => near::process(near),
        Some(Chain::Polkadot(polkadot)) => polkadot::process(polkadot),
        Some(Chain::Aptos(aptos)) => aptos::process(aptos),
        Some(Chain::Cosmos(cosmos)) => cosmos::process(cosmos),
        Some(Chain::Arweave(arweave)) => arweave::process(arweave),
        None => {
            unimplemented!()
        }
    }
}
