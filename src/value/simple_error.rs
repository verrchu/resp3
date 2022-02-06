use std::str;

use nom::{
    bytes::complete::tag,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static MSG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());
static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]+").unwrap());

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleError {
    pub code: String,
    pub msg: String,
}

impl From<SimpleError> for Value {
    fn from(input: SimpleError) -> Value {
        Value::SimpleError(input)
    }
}

impl SimpleError {
    pub fn new(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
        }
    }
}

impl SimpleError {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let msg = Lazy::force(&MSG).to_owned();
        let code = Lazy::force(&CODE).to_owned();

        let (input, code) = preceded(tag("-"), re_find(code)).parse(input)?;

        delimited(tag(" "), re_find(msg), tag(DELIMITER))
            .map(|msg| {
                let code = unsafe { str::from_utf8_unchecked(code) };
                let msg = unsafe { str::from_utf8_unchecked(msg) };

                SimpleError::new(code, msg)
            })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::{Error, ErrorKind};

    #[test]
    fn test_basic() {
        assert_eq!(
            SimpleError::parse(&b"-ERR reason\r\n"[..]),
            Ok((&b""[..], SimpleError::new("ERR", "reason")))
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            SimpleError::parse(&b"-ERR some\nreason\r\n"[..]),
            Err(nom::Err::Error(Error::new(
                &b"\nreason\r\n"[..],
                ErrorKind::Tag
            )))
        );

        assert_eq!(
            SimpleError::parse(&b"-ERR some\rreason\r\n"[..]),
            Err(nom::Err::Error(Error::new(
                &b"\rreason\r\n"[..],
                ErrorKind::Tag
            )))
        );
    }
}
