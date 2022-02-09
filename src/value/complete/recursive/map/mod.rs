use std::{
    collections::BTreeMap,
    io::Write,
    str::{self, FromStr},
};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::many_m_n,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map(BTreeMap<Value, Value>);

impl From<Map> for Value {
    fn from(input: Map) -> Value {
        Value::Map(input)
    }
}

impl Map {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("%"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Map (str::from_utf8)")
                    .and_then(|v| usize::from_str(v).context("Value::Map (usize::from_str)"))
            })
        };

        let parse_val = |len| many_m_n(len, len, pair(Value::parse, Value::parse));
        parse_len.flat_map(parse_val).map(Map::from).parse(input)
    }
}

impl<I: IntoIterator<Item = (Value, Value)>> From<I> for Map {
    fn from(input: I) -> Self {
        Self(input.into_iter().collect())
    }
}

impl TryFrom<Map> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Map) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        buf.write(b"~")
            .and_then(|_| buf.write(input.0.len().to_string().as_bytes()))
            .and_then(|_| buf.write(b"\r\n"))
            .context("Value::Map (buf::write)")?;

        for (k, v) in input.0.into_iter() {
            let bytes = Bytes::try_from(k).context("Value::Map (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Map (buf::write)")?;

            let bytes = Bytes::try_from(v).context("Value::Map (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Map (buf::write)")?;
        }

        buf.flush().context("Value::Map (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::value::{complete::primitive::double, *};
    use num_bigint::BigInt;

    #[test]
    fn test_empty() {
        assert_eq!(Map::parse(&b"%0\r\n"[..]), Ok((&b""[..], Map::from([]))));
    }

    #[test]
    fn test_heterogenous_keys_simple() {
        let raw = "\
                   %10\r\n\
                   (12345\r\n_\r\n\
                   !10\r\nERR reason\r\n_\r\n\
                   $4\r\ntest\r\n_\r\n\
                   #f\r\n_\r\n\
                   ,-inf\r\n_\r\n\
                   _\r\n_\r\n\
                   :1234\r\n_\r\n\
                   -ERR reason\r\n_\r\n\
                   +test\r\n_\r\n\
                   =8\r\ntxt:test\r\n_\r\n\
                   ";

        assert_eq!(
            Map::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Map::from([
                    (
                        Value::BigNumber(BigNumber(BigInt::from_str("12345").unwrap())),
                        Value::Null
                    ),
                    (
                        Value::BlobError(BlobError::new("ERR", b"reason".to_vec())),
                        Value::Null
                    ),
                    (
                        Value::BlobString(BlobString::from(b"test".to_vec())),
                        Value::Null
                    ),
                    (Value::Boolean(Boolean(false)), Value::Null),
                    (Value::Double(Double::Inf(double::Sign::Minus)), Value::Null),
                    (Value::Null, Value::Null),
                    (Value::Number(Number(1234)), Value::Null),
                    (
                        Value::SimpleError(SimpleError::new("ERR", "reason")),
                        Value::Null
                    ),
                    (Value::SimpleString(SimpleString::from("test")), Value::Null),
                    (
                        Value::VerbatimString(VerbatimString::txt(b"test".to_vec())),
                        Value::Null
                    ),
                ])
            ))
        );
    }

    #[test]
    fn test_heterogenous_value_simple() {
        let raw = "\
                   %10\r\n\
                   :0\r\n(12345\r\n\
                   :1\r\n!10\r\nERR reason\r\n\
                   :2\r\n$4\r\ntest\r\n\
                   :3\r\n#f\r\n\
                   :4\r\n,-inf\r\n\
                   :5\r\n_\r\n\
                   :6\r\n:1234\r\n\
                   :7\r\n-ERR reason\r\n\
                   :8\r\n+test\r\n\
                   :9\r\n=8\r\ntxt:test\r\n\
                   ";

        assert_eq!(
            Map::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Map::from([
                    (
                        Value::Number(Number::from(0)),
                        Value::BigNumber(BigNumber(BigInt::from_str("12345").unwrap()))
                    ),
                    (
                        Value::Number(Number::from(1)),
                        Value::BlobError(BlobError::new("ERR", b"reason".to_vec()))
                    ),
                    (
                        Value::Number(Number::from(2)),
                        Value::BlobString(BlobString::from(b"test".to_vec()))
                    ),
                    (
                        Value::Number(Number::from(3)),
                        Value::Boolean(Boolean(false))
                    ),
                    (
                        Value::Number(Number::from(4)),
                        Value::Double(Double::Inf(double::Sign::Minus))
                    ),
                    (Value::Number(Number::from(5)), Value::Null),
                    (Value::Number(Number::from(6)), Value::Number(Number(1234))),
                    (
                        Value::Number(Number::from(7)),
                        Value::SimpleError(SimpleError::new("ERR", "reason"))
                    ),
                    (
                        Value::Number(Number::from(8)),
                        Value::SimpleString(SimpleString::from("test"))
                    ),
                    (
                        Value::Number(Number::from(9)),
                        Value::VerbatimString(VerbatimString::txt(b"test".to_vec()))
                    ),
                ])
            ))
        );
    }

    #[test]
    fn test_nested_map_key() {
        let raw = "\
            %1\r\n\
                %2\r\n\
                    :1\r\n#f\r\n\
                    :2\r\n#t\r\n\
                _\r\n";

        assert_eq!(
            Map::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Map::from([(
                    Value::Map(Map::from([
                        (
                            Value::Number(Number::from(1)),
                            Value::Boolean(Boolean::from(false))
                        ),
                        (
                            Value::Number(Number::from(2)),
                            Value::Boolean(Boolean::from(true))
                        ),
                    ])),
                    Value::Null
                )])
            ))
        );
    }

    #[test]
    fn test_nested_map_value() {
        let raw = "\
            %1\r\n\
                _\r\n\
                %2\r\n\
                    :1\r\n#f\r\n\
                    :2\r\n#t\r\n\
                  ";

        assert_eq!(
            Map::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Map::from([(
                    Value::Null,
                    Value::Map(Map::from([
                        (
                            Value::Number(Number::from(1)),
                            Value::Boolean(Boolean::from(false))
                        ),
                        (
                            Value::Number(Number::from(2)),
                            Value::Boolean(Boolean::from(true))
                        ),
                    ]))
                )])
            ))
        );
    }
}
