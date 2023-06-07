use crate::{CommandResponse, GetRsaPublicKeyRequest, SetupAdaRootKeyRequest};
use rcc_signer::{Signer};
use crate::processors::error::{ADAKeyError};

pub fn process(params: SetupAdaRootKeyRequest) -> Result<bool, ADAKeyError> {
    let signer = Signer::new_with_se(params.port_name);

    match signer.setup_ada_root_key(
        params.seed_id as u8,
        params.password,
        params.passphrase,
    ) {
        Ok(result) => Ok(result),
        Err(err) => Err(ADAKeyError::SetupError(err.to_string())),
    }
}
