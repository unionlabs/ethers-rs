//! Boilerplate error definitions.
use crate::abi::{human_readable, InvalidOutputType};
use thiserror::Error;

/// A type alias for std's Result with the Error as our error type.
pub type Result<T, E = ParseError> = std::result::Result<T, E>;

/// Error that can occur during human readable parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    Message(String),
    // ethabi parser error
    #[error("{0:?}")]
    ParseError(ethabi::Error),
    // errors from human readable lexer
    #[error(transparent)]
    LexerError(#[from] human_readable::lexer::LexerError),
}

impl From<ethabi::Error> for ParseError {
    fn from(value: ethabi::Error) -> Self {
        Self::ParseError(value)
    }
}

macro_rules! _format_err {
    ($($tt:tt)*) => {
        $crate::abi::ParseError::Message(format!($($tt)*))
    };
}
pub(crate) use _format_err as format_err;

macro_rules! _bail {
    ($($tt:tt)*) => { return Err($crate::abi::error::format_err!($($tt)*)) };
}
use crate::types::ParseBytesError;
pub(crate) use _bail as bail;

/// ABI codec related errors
#[derive(Error, Debug)]
pub enum AbiError {
    /// Thrown when the ABI decoding fails
    #[error("{0:?}")]
    DecodingError(ethabi::Error),

    /// Thrown when detokenizing an argument
    #[error(transparent)]
    DetokenizationError(#[from] InvalidOutputType),

    #[error("missing or wrong function selector")]
    WrongSelector,

    #[error(transparent)]
    ParseBytesError(#[from] ParseBytesError),
}

impl From<ethabi::Error> for AbiError {
    fn from(value: ethabi::Error) -> Self {
        Self::DecodingError(value)
    }
}
