use crate::processors::error::SignError;
use crate::{CommandResponse, SignRequest};
use rcc_signer::{RSASignType, SigningOption};
use rcc_signer::{Signer, SigningAlgorithm};
use std::collections::HashMap;
use std::iter::Map;

pub fn process(params: SignRequest) -> Result<String, SignError> {
    let signer = Signer::new_with_se(params.port_name);
    let sign_data = match hex::decode(&params.data) {
        Ok(data) => data,
        Err(_) => {
            return Err(SignError::DecodeHexError(
                "sign data decode failed".to_string(),
            ));
        }
    };
    let mut signing_option: Option<SigningOption> = None;
    let algo = match params.algo {
        0 => SigningAlgorithm::Secp256k1,
        2 => {
            let option: HashMap<String, String> = params.signing_option;
            if let Some(bool) = option.get("sign_ada") {
                if bool.eq("true") {
                    signing_option = Some(SigningOption::ADA)
                }
            }
            SigningAlgorithm::Ed25519
        }
        3 => {
            let option: HashMap<String, String> = params.signing_option;
            let mut parsed_salt_len: i32 = 0;
            let mut sign_type = RSASignType::Common;
            if let Some(salt_len) = option.get("salt_len") {
                parsed_salt_len = salt_len.parse().map_err(|_| {
                    SignError::ParseSigningOptionFailed("parse salt_len to int failed".to_string())
                })?;
            }
            if let Some(st) = option.get("sign_type") {
                match st.as_str() {
                    "ar_message" => sign_type = RSASignType::ARMessage,
                    _ => {}
                }
            }
            signing_option = Some(SigningOption::RSA {
                salt_len: parsed_salt_len,
                sign_type,
            });
            SigningAlgorithm::RSA
        }
        _ => {
            return Err(SignError::AlgoNotSupported(
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
        signing_option,
    ) {
        Ok(signature) => Ok(hex::encode(signature)),
        Err(err) => Err(SignError::SignFailed(err.to_string())),
    }
}
