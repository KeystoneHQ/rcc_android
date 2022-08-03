use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignError {
    #[error("error info : `{0}`")]
    DecodeHexError(String),

    #[error("error info : `{0}`")]
    AlgoNotSupported(String),

    #[error("error info : `{0}`")]
    SignFailed(String),
}