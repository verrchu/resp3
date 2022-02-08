mod array;
mod big_number;
mod blob_error;
mod blob_string;
mod boolean;
mod double;
mod map;
mod null;
mod number;
mod set;
mod simple_error;
mod simple_string;
mod verbatim_string;

pub use array::Array;
pub use big_number::BigNumber;
pub use blob_error::BlobError;
pub use blob_string::BlobString;
pub use boolean::Boolean;
pub use double::Double;
pub use map::Map;
pub use null::Null;
pub use number::Number;
pub use set::Set;
pub use simple_error::SimpleError;
pub use simple_string::SimpleString;
pub use verbatim_string::VerbatimString;

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
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
