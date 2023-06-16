//! Generic parsing utilities.
//! 
//! See [Parsable] for more info.

use std::str::FromStr;

use thiserror::Error;

/// A result type alias for parsing.
pub type Result<T> = std::result::Result<T, Error>;

/// A trait for types that can be parsed from a string.
/// 
/// Mainly used to parse moves and algorithms. 
/// 
/// Any type that implements [FromStr] implements this trait automatically. The only
/// reason not to use [FromStr] directly is because it doesn't allow implementing in
/// foreign types, which is something that might be needed. 
pub trait Parsable: Sized {
    /// Tries to parse the given string into the type.
    fn parse(s: &str) -> Result<Self>;
}

impl<T: FromStr> Parsable for T {
    fn parse(s: &str) -> Result<Self> {
        s.parse().map_err(|_| Error::InvalidChar(s.chars().next().unwrap()))
    }
}

/// An error that can occur while parsing.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of string")]
    UnexpectedEnd,
    #[error("expected end of string, found {0}")]
    ExpectedEnd(char),
    #[error("invalid character: {0}")]
    InvalidChar(char),
}
