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
        let parse_len = {
            let parser = delimited(tag("~"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Set (str::from_utf8)")
                    .and_then(|v| usize::from_str(v).context("Value::Set (usize::from_str)"))
            })
        };

        let parse_val = |len| many_m_n(len, len, Value::parse);
        parse_len.flat_map(parse_val).map(Set::from).parse(input)
    }
}

impl<I: IntoIterator<Item = Value>> From<I> for Set {
    fn from(input: I) -> Self {
        Self(input.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::*;
    use num_bigint::BigInt;
    use num_traits::Num;

    #[test]
    fn test_empty() {
        assert_eq!(Set::parse(&b"~0\r\n"[..]), Ok((&b""[..], Set::from([]))));
    }

    #[test]
    fn test_heterogenous_simple() {
        let raw = "\
           ~10\r\n\
               (12345\r\n\
               !10\r\nERR reason\r\n\
               $4\r\ntest\r\n\
               #f\r\n\
               ,-1.234\r\n\
               _\r\n\
               :1234\r\n\
               -ERR reason\r\n\
               +test\r\n\
               =8\r\ntxt:test\r\n\
                   ";

        assert_eq!(
            Set::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Set::from([
                    Value::BigNumber(BigNumber(BigInt::from_str_radix("12345", 10).unwrap())),
                    Value::BlobError(BlobError::new("ERR", b"reason".to_vec())),
                    Value::BlobString(BlobString::from(b"test".to_vec())),
                    Value::Boolean(Boolean(false)),
                    Value::Double(Double::from(-1.234)),
                    Value::Null,
                    Value::Number(Number(1234)),
                    Value::SimpleError(SimpleError::new("ERR", "reason")),
                    Value::SimpleString(SimpleString::from("test")),
                    Value::VerbatimString(VerbatimString::txt(b"test".to_vec())),
                ])
            ))
        );
    }

    #[test]
    fn test_nested_array() {
        let raw = "\
           ~2\r\n\
               ~1\r\n+test\r\n\
               ~2\r\n#f\r\n:-1\r\n\
                   ";

        assert_eq!(
            Set::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Set::from([
                    Value::Set(Set::from([Value::SimpleString(SimpleString::from("test"))])),
                    Value::Set(Set::from([
                        Value::Boolean(Boolean(false)),
                        Value::Number(Number(-1))
                    ])),
                ])
            ))
        );
    }
}
