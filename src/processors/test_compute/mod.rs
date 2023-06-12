
use bip39::Mnemonic;
use hex;

pub fn test_bip39() -> String {
    const test_entropy:[u8;32] = [0u8; 32];
    let a: Mnemonic = bip39::Mnemonic::from_entropy(&test_entropy).unwrap();
    let test_seed: [u8;64] = a.to_seed_normalized("");
    hex::encode(test_seed)
}