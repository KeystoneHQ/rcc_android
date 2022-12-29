use crate::{CommandResponse, GetRsaPublicKeyRequest};
use rcc_signer::{Signer, SigningAlgorithm};
use crate::processors::error::{GetRsaPublicKeyError, SignError};

pub fn process(params: GetRsaPublicKeyRequest) -> Result<String, GetRsaPublicKeyError> {
    let signer = Signer::new_with_se(params.port_name);

    match signer.get_rsa_public_key(
        params.seed_id as u8,
        params.password
    ) {
        Ok(signature) => Ok(hex::encode(signature)),
        Err(err) => Err(GetRsaPublicKeyError::GetPublicKeyFailed(err.to_string())),
    }
}
