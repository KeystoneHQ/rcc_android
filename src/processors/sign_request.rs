use crate::{CommandResponse, SignRequest};
use rcc_signer::{Signer, SigningAlgorithm};
use crate::processors::error::SignError;

pub fn process(params: SignRequest) -> Result<String, SignError> {
    let signer = Signer::new_with_se(params.port_name);
    let sign_data = match hex::decode(&params.data) {
        Ok(data) => data,
        Err(_) => {
            return Err(SignError::DecodeHexError("sign data decode failed".to_string()));
        }
    };

    let algo = match params.algo {
        0 => SigningAlgorithm::Secp256k1,
        _ => {
            return Err(SignError::AlgoNotSupported("Algo is not supported".to_string()));
        }
    };

    match signer.sign_data(
        params.seed_id as u8,
        params.password,
        sign_data,
        algo,
        params.derivation_path,
    ) {
        Ok(signature) => Ok(hex::encode(signature)),
        Err(err) => Err(SignError::SignFailed(err.to_string())),
    }
}
