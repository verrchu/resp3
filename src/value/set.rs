use std::{
    collections::BTreeSet,
    str::{self, FromStr},
};

use anyhow::Context;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::many_m_n,
    sequence::delimited, IResult, Parser,
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
        let mut parse_len = {
            let parser = delimited(tag("~"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Set (str::from_utf8)")
                    .and_then(|v| usize::from_str(v).context("Value::Set (usize::from_str)"))
            })
        };

        // TODO: use flat_map instead
        let (input, len) = parse_len.parse(input)?;

        many_m_n(len, len, Value::parse).map(Set::from).parse(input)
    }
}

impl<I: IntoIterator<Item = Value>> From<I> for Set {
    fn from(input: I) -> Self {
        Self(input.into_iter().collect())
    }
}
