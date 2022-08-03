use crate::{CommandResponse, SignRequest};
use rcc_signer::{Signer, SigningAlgorithm};

pub fn process(params: SignRequest, request_id:u32) -> CommandResponse {
    let signer = Signer::new_with_se(params.port_name);
    let sign_data = match hex::decode(&params.data) {
        Ok(data) => data,
        Err(_) => {
            return CommandResponse::error(
                request_id,
                "sign data decode failed".to_string(),
            )
        }
    };

    let algo = match params.algo {
        0 => SigningAlgorithm::Secp256k1,
        _ => {
            return CommandResponse::error(request_id, "Algo is not supported".to_string())
        }
    };

    match signer.sign_data(
        params.seed_id as u8,
        params.password,
        sign_data,
        algo,
        params.derivation_path,
    ) {
        Ok(signature) => CommandResponse::success(request_id, hex::encode(signature)),
        Err(err) => CommandResponse::error(request_id, err.to_string()),
    }
}
