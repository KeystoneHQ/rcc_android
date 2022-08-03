use crate::block_chain_request::Chain;
use crate::{BlockChainRequest, CommandResponse};

mod solana;
mod near;

pub fn process(params: BlockChainRequest, request_id: u32) -> CommandResponse {
    match params.chain {
        Some(Chain::Solana(solana)) => solana::process(solana),
        Some(Chain::Near(near)) => near::process(near, request_id),
        None => {
            unimplemented!()
        }
    }
}
