use nom::{
    bytes::complete::tag,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use nom_regex::str::re_find;
use once_cell::sync::Lazy;
use regex::Regex;

static MSG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());
static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]+").unwrap());

use super::DELIMITER;

#[derive(Debug, PartialEq)]
pub struct SimpleError {
    code: String,
    msg: String,
}

impl SimpleError {
    fn new(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
        }
    }
}

impl SimpleError {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let msg = Lazy::force(&MSG).to_owned();
        let code = Lazy::force(&CODE).to_owned();

        let (input, code) = preceded(tag("-"), re_find(code)).parse(input)?;

        delimited(tag(" "), re_find(msg), tag(DELIMITER))
            .map(|msg| SimpleError::new(code, msg))
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
            SimpleError::parse("-ERR reason\r\n"),
            Ok(("", SimpleError::new("ERR", "reason")))
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            SimpleError::parse("-ERR some\nreason\r\n"),
            Err(nom::Err::Error(Error::new("\nreason\r\n", ErrorKind::Tag)))
        );

        assert_eq!(
            SimpleError::parse("-ERR some\rreason\r\n"),
            Err(nom::Err::Error(Error::new("\rreason\r\n", ErrorKind::Tag)))
        );
    }
}
