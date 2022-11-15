use crate::{CommandResponse, GetRsaPublicKeyRequest};
use rcc_signer::{Signer, SigningAlgorithm};
use crate::processors::error::{GetPublicKeyError, SignError};

pub fn process(params: GetRsaPublicKeyRequest) -> Result<String, GetPublicKeyError> {
    let signer = Signer::new_with_se(params.port_name);

    let algo = match params.algo {
        3 => SigningAlgorithm::RSA,
        _ => {
            return Err(GetPublicKeyError::AlgoNotSupported("Algo is not supported".to_string()));
        }
    };

    match signer.get_rsa_public_key(
        params.seed_id as u8,
        params.password
    ) {
        Ok(signature) => Ok(hex::encode(signature)),
        Err(err) => Err(GetPublicKeyError::GetPublicKeyFailed(err.to_string())),
    }
}
