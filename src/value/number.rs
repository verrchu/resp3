use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    error::{Error, ErrorKind},
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::DELIMITER;

#[derive(Debug, PartialEq, Eq)]
pub struct Number(i64);

impl Number {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(tag(":"), pair(opt(tag("-")), digit1), tag(DELIMITER))
            .parse(input)
            .and_then(|(i, (sign, number))| {
                let number = sign
                    .map(|_| format!("-{number}"))
                    .unwrap_or_else(|| number.to_string());

                let o = i64::from_str(&number)
                    .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, Number(o)))
            })
    }
}

impl From<i64> for Number {
    fn from(input: i64) -> Self {
        Self(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(Number::parse(":1234\r\n"), Ok(("", Number(1234))));
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(Number::parse(":-1234\r\n"), Ok(("", Number(-1234))));
    }
}
