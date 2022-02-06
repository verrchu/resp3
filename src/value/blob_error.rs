use std::str::{self, FromStr};

use super::{Value, DELIMITER};

use bytes::Bytes;
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    error::{Error, ErrorKind},
    sequence::{preceded, terminated},
    Err, IResult, Parser,
};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]+").unwrap());

#[derive(Debug, PartialEq, Eq)]
pub struct BlobError {
    pub code: String,
    pub msg: Bytes,
}

impl From<BlobError> for Value {
    fn from(input: BlobError) -> Value {
        Value::BlobError(input)
    }
}

impl BlobError {
    pub(crate) fn new(code: impl Into<String>, msg: impl Into<Bytes>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
        }
    }
}

impl BlobError {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, len) = terminated(preceded(tag("!"), digit1), tag(DELIMITER))
            .parse(input)
            .and_then(|(i, o)| {
                let o = str::from_utf8(o)
                    .map_err(|_| Err::Error(Error::new(input, ErrorKind::Digit)))?;
                let o = u64::from_str(o)
                    .map_err(|_| Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, o))
            })?;

        let (input, msg) = terminated(take(len), tag(DELIMITER)).parse(input)?;

        let code = Lazy::force(&CODE).to_owned();
        let (msg, code) = terminated(re_find(code), tag(" ")).parse(msg)?;

        let code =
            str::from_utf8(code).map_err(|_| Err::Error(Error::new(msg, ErrorKind::Alpha)))?;

        Ok((input, BlobError::new(code, msg.to_vec())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            BlobError::parse(&b"!10\r\nERR reason\r\n"[..]),
            Ok((&b""[..], BlobError::new("ERR", b"reason".to_vec())))
        );
    }
}
