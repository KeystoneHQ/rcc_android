use std::collections::HashMap;
use std::iter::Map;
use crate::{CommandResponse, SignRequest};
use rcc_signer::{Signer, SigningAlgorithm};
use rcc_signer::SigningOption;
use crate::processors::error::SignError;

pub fn process(params: SignRequest) -> Result<String, SignError> {
    let signer = Signer::new_with_se(params.port_name);
    let sign_data = match hex::decode(&params.data) {
        Ok(data) => data,
        Err(_) => {
            return Err(SignError::DecodeHexError("sign data decode failed".to_string()));
        }
    };
    let mut signing_option: Option<SigningOption> = None;
    let algo = match params.algo {
        0 => SigningAlgorithm::Secp256k1,
        3 => {

            let option: HashMap<String, String> = params.signing_option;
            if let Some(salt_len) = option.get("salt_len"){
                let parsed_salt_len: i32 = salt_len.parse().map_err(|_| SignError::ParseSigningOptionFailed("parse salt_len to int failed".to_string()))?;
                signing_option = Some(SigningOption::RSA {
                    salt_len: parsed_salt_len
                })
            }
        SigningAlgorithm::RSA
        },
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
        signing_option
    ) {
        Ok(signature) => Ok(hex::encode(signature)),
        Err(err) => Err(SignError::SignFailed(err.to_string())),
    }
}
