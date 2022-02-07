use std::str::{self, FromStr};

use anyhow::Context;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Number(pub i64);

impl From<Number> for Value {
    fn from(input: Number) -> Value {
        Value::Number(input)
    }
}

impl Number {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_val = {
            let parser = pair(opt(tag("-")), digit1);
            let wrapper = map_res(parser, |(sign, number)| {
                let number = str::from_utf8(number).context("Value::Number (str::from_utf8)")?;
                let number = sign
                    .map(|_| format!("-{number}"))
                    .unwrap_or_else(|| number.to_string());

                Ok::<_, anyhow::Error>(number)
            });

            map_res(wrapper, |number| {
                i64::from_str(&number).context("Value::Number (i64::from_str)")
            })
        };

        delimited(tag(":"), parse_val, tag(DELIMITER))
            .map(Number::from)
            .parse(input)
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
        assert_eq!(
            Number::parse(&b":1234\r\n"[..]),
            Ok((&b""[..], Number(1234)))
        );
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(
            Number::parse(&b":-1234\r\n"[..]),
            Ok((&b""[..], Number(-1234)))
        );
    }
}
