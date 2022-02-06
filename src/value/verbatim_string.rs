use std::str::{self, FromStr};

use bytes::Bytes;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    error::{Error, ErrorKind},
    sequence::{preceded, separated_pair, terminated},
    Err, IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq)]
pub enum VerbatimString {
    Txt(Bytes),
    Mkd(Bytes),
}

impl From<VerbatimString> for Value {
    fn from(input: VerbatimString) -> Value {
        Value::VerbatimString(input)
    }
}

impl VerbatimString {
    pub fn txt(bytes: impl Into<Bytes>) -> Self {
        Self::Txt(bytes.into())
    }

    pub fn mkd(bytes: impl Into<Bytes>) -> Self {
        Self::Mkd(bytes.into())
    }
}

impl VerbatimString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, len) = terminated(preceded(tag("="), digit1), tag(DELIMITER))
            .parse(input)
            .and_then(|(i, o)| {
                let o = str::from_utf8(o)
                    .map_err(|_| Err::Error(Error::new(input, ErrorKind::Digit)))?;
                let o = u64::from_str(o)
                    .map_err(|_| Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, o))
            })?;

        let parse_msg = separated_pair(alt((tag("txt"), tag("mkd"))), tag(":"), take(len - 4));
        let (input, (ty, msg)) = terminated(parse_msg, tag(DELIMITER)).parse(input)?;

        let value = match ty {
            b"txt" => VerbatimString::Txt(Bytes::from(msg.to_vec())),
            b"mkd" => VerbatimString::Mkd(Bytes::from(msg.to_vec())),
            _ => unreachable!(),
        };

        Ok((input, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            VerbatimString::parse(&b"=15\r\ntxt:hello world\r\n"[..]),
            Ok((
                &b""[..],
                VerbatimString::Txt(Bytes::from(b"hello world".to_vec()))
            ))
        );

        assert_eq!(
            VerbatimString::parse(&b"=15\r\nmkd:hello world\r\n"[..]),
            Ok((
                &b""[..],
                VerbatimString::Mkd(Bytes::from(b"hello world".to_vec()))
            ))
        );
    }
}
