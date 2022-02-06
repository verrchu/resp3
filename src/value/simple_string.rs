use nom::{bytes::complete::tag, sequence::delimited, IResult, Parser};
use nom_regex::str::re_find;
use once_cell::sync::Lazy;
use regex::Regex;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());

use super::DELIMITER;

#[derive(Debug, PartialEq)]
pub struct SimpleString(String);

impl SimpleString {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let re = Lazy::force(&RE).to_owned();

        delimited(tag("+"), re_find(re), tag(DELIMITER))
            .map(Self::from)
            .parse(input)
    }
}

impl<'a> From<&'a str> for SimpleString {
    fn from(input: &'a str) -> Self {
        Self(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::{Error, ErrorKind};

    #[test]
    fn test_basic() {
        assert_eq!(
            SimpleString::parse("+hello world\r\n"),
            Ok(("", SimpleString(String::from("hello world"))))
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            SimpleString::parse("+hello\nworld\r\n"),
            Err(nom::Err::Error(Error::new("\nworld\r\n", ErrorKind::Tag)))
        );

        assert_eq!(
            SimpleString::parse("+hello\rworld\r\n"),
            Err(nom::Err::Error(Error::new("\rworld\r\n", ErrorKind::Tag)))
        );
    }
}
