use crate::{CommandResponse, GetAdaExtendedPublicKeyRequest};
use rcc_signer::{Signer};
use crate::processors::error::{ADAKeyError};

pub fn process(params: GetAdaExtendedPublicKeyRequest) -> Result<String, ADAKeyError> {
    let signer = Signer::new_with_se(params.port_name);

    match signer.get_ada_extended_public_key(
        params.seed_id as u8,
        params.password,
        params.path,
    ) {
        Ok(result) => Ok(result),
        Err(err) => Err(ADAKeyError::GetKeyError(err.to_string())),
    }
}
