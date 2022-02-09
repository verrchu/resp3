mod complete;

pub use complete::{
    primitive::{BigNumber, Boolean, Double, Null, Number},
    recursive::{Array, Map, Set},
    textual::{BlobError, BlobString, SimpleError, SimpleString, VerbatimString},
};

use bytes::Bytes;
use nom::{branch::alt, IResult, Parser};

static DELIMITER: &str = "\r\n";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Array(Array),
    BigNumber(BigNumber),
    BlobError(BlobError),
    BlobString(BlobString),
    Boolean(Boolean),
    Double(Double),
    Map(Map),
    Null,
    Number(Number),
    Set(Set),
    SimpleError(SimpleError),
    SimpleString(SimpleString),
    VerbatimString(VerbatimString),
}

impl Value {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            Array::parse.map(Value::from),
            BigNumber::parse.map(Value::from),
            BlobError::parse.map(Value::from),
            BlobString::parse.map(Value::from),
            Boolean::parse.map(Value::from),
            Double::parse.map(Value::from),
            Map::parse.map(Value::from),
            Null::parse.map(Value::from),
            Number::parse.map(Value::from),
            Set::parse.map(Value::from),
            SimpleError::parse.map(Value::from),
            SimpleString::parse.map(Value::from),
            VerbatimString::parse.map(Value::from),
        ))
        .parse(input)
    }
}

impl TryFrom<Value> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Value) -> anyhow::Result<Bytes> {
        match input {
            Value::Array(inner) => Bytes::try_from(inner),
            Value::BigNumber(inner) => Bytes::try_from(inner),
            Value::BlobError(inner) => Bytes::try_from(inner),
            Value::BlobString(inner) => Bytes::try_from(inner),
            Value::Boolean(inner) => Bytes::try_from(inner),
            Value::Double(inner) => Bytes::try_from(inner),
            Value::Map(inner) => Bytes::try_from(inner),
            Value::Null => Bytes::try_from(Null),
            Value::Number(inner) => Bytes::try_from(inner),
            Value::Set(inner) => Bytes::try_from(inner),
            Value::SimpleError(inner) => Bytes::try_from(inner),
            Value::SimpleString(inner) => Bytes::try_from(inner),
            Value::VerbatimString(inner) => Bytes::try_from(inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{complete::primitive::double, *};
    use bytes::Bytes;
    use num_bigint::BigInt;

    #[test]
    fn test_basic_array() {
        assert_eq!(
            Value::parse(&b"*1\r\n+test\r\n"[..]),
            Ok((
                &b""[..],
                Value::Array(Array::from([Value::SimpleString(SimpleString::from(
                    "test"
                ))]))
            ))
        );
    }

    #[test]
    fn test_basic_boolean() {
        assert_eq!(
            Value::parse(&b"#f\r\n"[..]),
            Ok((&b""[..], Value::Boolean(Boolean(false))))
        );
    }

    #[test]
    fn test_basic_big_number() {
        let raw = ['1'; 100].into_iter().collect::<String>();

        assert_eq!(
            Value::parse(format!("({raw}\r\n").as_bytes()),
            Ok((
                &b""[..],
                Value::BigNumber(BigNumber(BigInt::from_str(&raw).unwrap()))
            ))
        );
    }

    #[test]
    fn test_basic_blob_error() {
        assert_eq!(
            Value::parse(&b"!10\r\nERR reason\r\n"[..]),
            Ok((
                &b""[..],
                Value::BlobError(BlobError::new("ERR", b"reason".to_vec()))
            ))
        );
    }

    #[test]
    fn test_basic_blob_string() {
        assert_eq!(
            Value::parse(&b"$4\r\ntest\r\n"[..]),
            Ok((
                &b""[..],
                Value::BlobString(BlobString(Bytes::from(b"test".to_vec())))
            ))
        );
    }

    #[test]
    fn test_basic_double() {
        assert_eq!(
            Value::parse(&b",-inf\r\n"[..]),
            Ok((&b""[..], Value::Double(Double::Inf(double::Sign::Minus))))
        );
    }

    #[test]
    fn test_basic_number() {
        assert_eq!(
            Value::parse(&b":0\r\n"[..]),
            Ok((&b""[..], Value::Number(Number(0))))
        );
    }

    #[test]
    fn test_basic_null() {
        assert_eq!(Value::parse(&b"_\r\n"[..]), Ok((&b""[..], Value::Null)));
    }

    #[test]
    fn test_basic_simple_error() {
        assert_eq!(
            Value::parse(&b"-ERR reason\r\n"[..]),
            Ok((
                &b""[..],
                Value::SimpleError(SimpleError::new("ERR", "reason"))
            ))
        );
    }

    #[test]
    fn test_basic_simple_string() {
        assert_eq!(
            Value::parse(&b"+test\r\n"[..]),
            Ok((&b""[..], Value::SimpleString(SimpleString::from("test"))))
        );
    }

    #[test]
    fn test_basic_verbatim_string() {
        assert_eq!(
            Value::parse(&b"=10\r\ntxt:123456\r\n"[..]),
            Ok((
                &b""[..],
                Value::VerbatimString(VerbatimString::Txt(Bytes::from(b"123456".to_vec())))
            ))
        );
    }
}
