use super::error::ParseError;
use crate::pb::abi::cardano::Method;
use crate::pb::abi::{Cardano, ComposeWitnessSet, DerivePublicKey, GenerateAddress, ParseCardanoTransaction};
use bitcoin::bip32::{DerivationPath, IntoDerivationPath};
use rcc_cardano::structs::{CardanoCertKey, CardanoSignarure, CardanoUtxo, ParseContext};
use rcc_cardano::transaction::{parse_tx, parse_tx_to_json};
use rcc_cardano::address;
use std::str::FromStr;
use rcc_cardano::address::{AddressType, derive_address};
use serde_json::json;

pub fn process(data: Cardano) -> Result<String, ParseError> {
    match data.method {
        Some(Method::ParseTransaction(request)) => {
            parse_transaction(request)
        }
        Some(Method::GenerateAddress(request)) => {
            generate_address(request)
        }
        Some(Method::DerivePublicKey(request)) => {
            derive_public_key(request)
        }
        Some(Method::ComposeWitnessSet(request)) => {
            compose_witness_set(request)
        }
        _ => Err(ParseError::CardanoParseError("message type is not supported".to_string()))
    }
}

fn parse_transaction(data: ParseCardanoTransaction) -> Result<String, ParseError> {
    let tx_data = hex::decode(data.data)?;
    let cardano_xpub = data.xpub;
    let master_fingerprint = hex::decode(data.master_fingerprint)?;
    let utxos = data
        .utxos
        .iter()
        .map(|v| {
            let path = DerivationPath::from_str(&v.path)
                .map_err(|e| ParseError::CardanoParseError(e.to_string()))?;
            let mfp = hex::decode(v.master_fingerprint.clone())?;
            let transaction_hash = hex::decode(v.transaction_hash.clone())?;
            let utxo = CardanoUtxo::new(
                mfp,
                v.address.clone(),
                path,
                v.value.clone(),
                transaction_hash,
                v.index.clone(),
            );
            Ok(utxo)
        })
        .collect::<Result<Vec<CardanoUtxo>, ParseError>>()?;
    let cert_keys = data
        .cert_keys
        .iter()
        .map(|v| {
            let mfp = hex::decode(v.master_fingerprint.clone())?;
            let key = hex::decode(v.key_hash.clone())?;
            let path = DerivationPath::from_str(&v.path).map_err(|e| ParseError::CardanoParseError(e.to_string()))?;
            let cert_key = CardanoCertKey::new(mfp, key, path);
            Ok(cert_key)
        })
        .collect::<Result<Vec<CardanoCertKey>, ParseError>>()?;
    let context = ParseContext::new(utxos, cert_keys, cardano_xpub, master_fingerprint);
    let result = parse_tx_to_json(tx_data, context)?;
    Ok(result)
}

fn generate_address(request: GenerateAddress) -> Result<String, ParseError> {
    let xpub = request.xpub;
    let index = request.index;
    let address_type = match request.t {
        0 => AddressType::Base,
        1 => AddressType::Stake,
        2 => AddressType::Enterprise,
        _ => return Err(ParseError::CardanoParseError("Invalid Address type".to_string()))
    };
    let address = derive_address(xpub, 0, index, address_type, 1)?;
    Ok(address)
}

fn derive_public_key(request: DerivePublicKey) -> Result<String, ParseError> {
    address::derive_public_key(request.xpub, request.sub_path).map_err(|e| ParseError::CardanoParseError(e.to_string()))
}

fn compose_witness_set(request: ComposeWitnessSet) -> Result<String, ParseError> {
    let signatures = request.signatures.iter().map(|v| {
        let public_key = hex::decode(v.public_key.clone()).map_err(|e| ParseError::CardanoParseError(e.to_string()))?;
        let signature = hex::decode(v.signature.clone()).map_err(|e| ParseError::CardanoParseError(e.to_string()))?;
        Ok(CardanoSignarure::new(public_key, signature))
    }).collect::<Result<Vec<CardanoSignarure>, ParseError>>()?;
    rcc_cardano::transaction::compose_witness_set(signatures).map_err(|e| ParseError::CardanoParseError(e.to_string()))
}