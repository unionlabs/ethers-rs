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

impl PartialEq for AbiError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AbiError::DecodingError(this), AbiError::DecodingError(other)) => {
                match (this, other) {
                    (ethabi::Error::InvalidName(this), ethabi::Error::InvalidName(other)) => {
                        this == other
                    }
                    (ethabi::Error::InvalidData, ethabi::Error::InvalidData) => true,
                    #[cfg(feature = "std")]
                    (ethabi::Error::SerdeJson(this), ethabi::Error::SerdeJson(other)) => {
                        this.to_string() == other.to_string()
                    }
                    (ethabi::Error::ParseInt(this), ethabi::Error::ParseInt(other)) => {
                        this == other
                    }
                    (ethabi::Error::Hex(this), ethabi::Error::Hex(other)) => this == other,
                    (ethabi::Error::Other(this), ethabi::Error::Other(other)) => this == other,
                    _ => false,
                }
            }
            (AbiError::DetokenizationError(this), AbiError::DetokenizationError(other)) => {
                this == other
            }
            (AbiError::WrongSelector, AbiError::WrongSelector) => todo!(),
            (AbiError::ParseBytesError(this), AbiError::ParseBytesError(other)) => this == other,
            _ => false,
        }
    }
}

impl From<ethabi::Error> for AbiError {
    fn from(value: ethabi::Error) -> Self {
        Self::DecodingError(value)
    }
}
