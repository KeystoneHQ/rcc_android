pub mod SignerService {
    use crate::pb::abi::GetRsaPublicKey;
    use crate::processors::error::SignerError;
    use crate::{SignRequest};
    use rcc_signer::{Signer, SigningAlgorithm};

    pub fn process(params: SignRequest) -> Result<String, SignerError> {
        let signer = Signer::new_with_se(params.port_name);
        let sign_data = match hex::decode(&params.data) {
            Ok(data) => data,
            Err(_) => {
                return Err(SignerError::DecodeHexError(
                    "sign data decode failed".to_string(),
                ));
            }
        };

        let algo = match params.algo {
            0 => SigningAlgorithm::Secp256k1,
            _ => {
                return Err(SignerError::AlgoNotSupported(
                    "Algo is not supported".to_string(),
                ));
            }
        };

        match signer.sign_data(
            params.seed_id as u8,
            params.password,
            sign_data,
            algo,
            params.derivation_path,
            None, // TODO: RSA logic
        ) {
            Ok(signature) => Ok(hex::encode(signature)),
            Err(err) => Err(SignerError::SignFailed(err.to_string())),
        }
    }

    pub fn get_rsa_public_key(params: GetRsaPublicKey) -> Result<String, SignerError> {
        let signer = Signer::new_with_se(params.port_name);
        match signer.get_rsa_public_key(params.seed_id as u8, params.password) {
            Ok(key) => Ok(hex::encode(key)),
            Err(err) => Err(SignerError::RSAPublicKeyError(err.to_string()))
        }
    }
}
