use std::{
    collections::BTreeSet,
    str::{self, FromStr},
};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    error::{Error, ErrorKind},
    multi::many_m_n,
    sequence::delimited,
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Set(BTreeSet<Value>);

impl From<Set> for Value {
    fn from(input: Set) -> Value {
        Value::Set(input)
    }
}

impl Set {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, len) = delimited(tag("~"), digit1, tag(DELIMITER))
            .parse(input)
            .and_then(|(i, o)| {
                let o = unsafe { str::from_utf8_unchecked(o) };
                let o = usize::from_str(o)
                    .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, o))
            })?;

        many_m_n(len, len, Value::parse).map(Set::from).parse(input)
    }
}

impl<I: IntoIterator<Item = Value>> From<I> for Set {
    fn from(input: I) -> Self {
        Self(input.into_iter().collect())
    }
}
