use crate::block_chain_request::Chain;
use crate::{BlockChainRequest, CommandResponse};

mod solana;

pub fn process(params: BlockChainRequest) -> CommandResponse {
    match params.chain {
        Some(Chain::Solana(solana)) => solana::process(solana),
        None => {
            unimplemented!()
        }
    }
}
