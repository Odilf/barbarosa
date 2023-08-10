//! Generic parsing utilities.
//!
//! See [Parsable] for more info.

use pest::{Parser, RuleType};
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
pub trait FromPest {
    /// The type of the pest rule used to parse this type.
    type Rule: RuleType;

    /// The parser used to parse this type.
    type Parser: Parser<Self::Rule>;

    /// The specific variant of the rule enum being parsed.
    fn rule() -> Self::Rule;

    /// Parses the given [`pest::iterators::Pair`] into the type.
    fn from_pest(pair: pest::iterators::Pair<Self::Rule>) -> Self;
}

impl<T: FromPest> Parsable for T {
    type Rule = T::Rule;

    fn parse(s: &str) -> Result<Self> {
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

        Ok(T::from_pest(pair))
    }
}

/// An error that can occur while parsing.
#[derive(Debug, Error)]
pub enum ParseError<T: Parsable>
where
    T::Rule: 'static,
{
    #[allow(missing_docs)]
    Pest(Box<pest::error::Error<T::Rule>>),
}

impl<T: Parsable> From<pest::error::Error<T::Rule>> for ParseError<T> {
    fn from(value: pest::error::Error<T::Rule>) -> Self {
        Self::Pest(Box::new(value))
    }
}

/// Type alias for a `pest` result, just much more concise.
pub type Result<T> = std::result::Result<T, ParseError<T>>;
