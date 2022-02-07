use std::str;

use anyhow::Context;
use nom::{bytes::complete::tag, combinator::map_res, sequence::delimited, IResult, Parser};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleString(String);

impl From<SimpleString> for Value {
    fn from(input: SimpleString) -> Value {
        Value::SimpleString(input)
    }
}

impl SimpleString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let re = Lazy::force(&RE).to_owned();

        let parser = delimited(tag("+"), re_find(re), tag(DELIMITER));
        let wrapper = map_res(parser, |v: &[u8]| {
            str::from_utf8(v).context("Value::SimpleString (str::from_utf8)")
        });

        wrapper.map(SimpleString::from).parse(input)
    }
}

impl<S: Into<String>> From<S> for SimpleString {
    fn from(input: S) -> Self {
        Self(input.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::{Error, ErrorKind};

    #[test]
    fn test_basic() {
        assert_eq!(
            SimpleString::parse(&b"+hello world\r\n"[..]),
            Ok((&b""[..], SimpleString(String::from("hello world"))))
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            SimpleString::parse(&b"+hello\nworld\r\n"[..]),
            Err(nom::Err::Error(Error::new(
                &b"\nworld\r\n"[..],
                ErrorKind::Tag
            )))
        );

        assert_eq!(
            SimpleString::parse(&b"+hello\rworld\r\n"[..]),
            Err(nom::Err::Error(Error::new(
                &b"\rworld\r\n"[..],
                ErrorKind::Tag
            )))
        );
    }
}
