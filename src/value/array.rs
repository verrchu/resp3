use std::str::{self, FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    error::{Error, ErrorKind},
    multi::many_m_n,
    sequence::delimited,
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq)]
pub struct Array(Vec<Value>);

impl From<Array> for Value {
    fn from(input: Array) -> Value {
        Value::Array(input)
    }
}

impl Array {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, len) = delimited(tag("*"), digit1, tag(DELIMITER))
            .parse(input)
            .and_then(|(i, o)| {
                let o = unsafe { str::from_utf8_unchecked(o) };
                let o = usize::from_str(o)
                    .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, o))
            })?;

        println!("{len}");

        many_m_n(len, len, Value::parse).map(Array).parse(input)
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
        assert_eq!(Array::parse(&b"*0\r\n"[..]), Ok((&b""[..], Array(vec![]))));
    }

    #[test]
    fn test_heterogenous_simple() {
        let raw = "\
                   *10\r\n\
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
            Array::parse(raw.as_bytes()),
            Ok((
                &b""[..],
                Array(vec![
                    Value::BigNumber(BigNumber(BigInt::from_str_radix("12345", 10).unwrap())),
                    Value::BlobError(BlobError::new("ERR", b"reason".to_vec())),
                    Value::BlobString(BlobString::from(b"test".to_vec())),
                    Value::Boolean(Boolean(false)),
                    Value::Double(Double(-1.234)),
                    Value::Null,
                    Value::Number(Number(1234)),
                    Value::SimpleError(SimpleError::new("ERR", "reason")),
                    Value::SimpleString(SimpleString::from("test")),
                    Value::VerbatimString(VerbatimString::txt(b"test".to_vec())),
                ])
            ))
        );
    }
}
