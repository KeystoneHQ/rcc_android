use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignError {
    #[error("error info : `{0}`")]
    DecodeHexError(String),

    #[error("error info : `{0}`")]
    AlgoNotSupported(String),

    #[error("error info : `{0}`")]
    SignFailed(String),

     #[error("error info : `{0}`")]
     ParseSigningOptionFailed(String),
}

#[derive(Error, Debug)]
pub enum GetRsaPublicKeyError {
    #[error("error info : `{0}`")]
    AlgoNotSupported(String),

    #[error("error info : `{0}`")]
    GetPublicKeyFailed(String),
}