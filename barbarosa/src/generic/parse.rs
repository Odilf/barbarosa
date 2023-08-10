//! Generic parsing utilities.
//!
//! See [Parsable] for more info.

use std::num::ParseIntError;

use pest::{iterators::Pair, Parser, RuleType};
use thiserror::Error;

// /// A result type alias for parsing.
// pub type Result<T, Rule> = std::result::Result<T, Error<Rule>>;

/// A trait for types that can be parsed from a string.
///
/// Mainly used to parse moves and algorithms.
///
/// Any type that implements [FromStr] implements this trait automatically. The only
/// reason not to use [FromStr] directly is because it doesn't allow implementing in
/// foreign types, which is something that might be needed.
///
/// [FromStr]: std::str::FromStr
pub trait Parsable: Sized {
    /// The type of the pest rule used to parse this type.
    type Rule: RuleType;

    /// Tries to parse the given string into the type.
    fn parse(s: &str) -> Result<Self>;
}

/// A trait for types that can be parsed from a [`pest::iterators::Pair`].
///
/// If this trait is implemented, [`Parsable::parse`] is automatically implemented
pub trait FromPest
where
    Self: Sized,
{
    /// The type of the pest rule used to parse this type.
    type Rule: RuleType;

    /// The parser used to parse this type.
    type Parser: Parser<Self::Rule>;

    /// The specific variant of the rule enum being parsed.
    fn rule() -> Self::Rule;

    /// Parses the given [`pest::iterators::Pair`] into the type.
    fn from_pest(pair: pest::iterators::Pair<Self::Rule>) -> Result<Self>;
}

impl<T: FromPest> Parsable for T {
    type Rule = T::Rule;

    fn parse(s: &str) -> Result<T> {
        let mut pairs = T::Parser::parse(T::rule(), s)?;
        let pair = pairs.next().unwrap();

        // Check if the entire string was parsed
        if pair.as_span().end() != s.len() {
            let err = pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: "Expected end of input".to_string(),
                },
                pair.as_span(),
            );

            return Err(ParseError::Pest(Box::new(err)));
        }

        debug_assert!(pairs.next().is_none());

        T::from_pest(pair)
    }
}

/// An error that can occur while parsing.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ParseError<R: RuleType>
where
    Self: 'static,
{
    #[error("{0}")]
    Pest(Box<pest::error::Error<R>>),

    #[error("Found unexpected string: {0}")]
    Unreachable(String),

    #[error("Error while parsing number: {0}")]
    NumParseError(#[from] ParseIntError),

    #[error("Unexpected end of tokens")]
    UnexpectedEndOfTokens,

    #[error("Statically uknown error: {0}")]
    Uknown(Box<dyn std::error::Error + Send + Sync>),
}

/// Type alias for a `pest` result, just much more concise.
pub type Result<T> = std::result::Result<T, ParseError<<T as Parsable>::Rule>>;

impl<R: RuleType> From<pest::error::Error<R>> for ParseError<R> {
    fn from(value: pest::error::Error<R>) -> Self {
        Self::Pest(Box::new(value))
    }
}

/// A trait for types that can be converted into a [ParseError].
/// 
/// Basically used for convience if you want to do `pair.next().unwrap()` but with proper
/// error handling (which would now be `pair.next().into_err()?`).
pub trait IntoParseErr<T, R: RuleType> {
    /// Converts `T` into a result of a [`ParseError`]
    fn into_err(self) -> std::result::Result<T, ParseError<R>>;
}

impl<'i, R: RuleType> IntoParseErr<Pair<'i, R>, R> for Option<Pair<'i, R>> {
    fn into_err(self) -> std::result::Result<Pair<'i, R>, ParseError<R>> {
        match self {
            Some(pair) => Ok(pair),
            None => Err(ParseError::UnexpectedEndOfTokens),
        }
    }
}

impl<T: RuleType> ParseError<T> {
    /// Convinience function for creating a [ParseError::Uknown] variant.
    pub fn uknown<Err: std::error::Error + Send + Sync + 'static>(err: Err) -> Self {
        Self::Uknown(Box::new(err))
    }
}
