use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    error::{Error, ErrorKind},
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

use super::DELIMITER;

#[derive(Debug, PartialEq)]
pub struct Double(f64);

impl Double {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let parse_inf = pair(opt(tag("-")), tag("inf"))
            .map(|(sign, _inf)| sign.map(|_| f64::NEG_INFINITY).unwrap_or(f64::INFINITY))
            .map(Ok);

        let parse_num = tuple((opt(tag("-")), digit1, opt(preceded(tag("."), digit1)))).map(
            |(sign, int, fract): (Option<&str>, &str, Option<&str>)| {
                let n = match (sign, fract) {
                    (Some(_sign), Some(fract)) => format!("-{int}.{fract}"),
                    (Some(_sign), None) => format!("-{int}"),
                    (None, Some(fract)) => format!("{int}.{fract}"),
                    (None, None) => int.to_string(),
                };

                f64::from_str(&n)
            },
        );

        delimited(tag(","), alt((parse_num, parse_inf)), tag(DELIMITER))
            .parse(input)
            .and_then(|(i, o)| {
                let o = o.map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, Double(o)))
            })
    }
}

impl From<f64> for Double {
    fn from(input: f64) -> Self {
        Self(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(Double::parse(",1.234\r\n"), Ok(("", Double(1.234))));
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(Double::parse(",-1.234\r\n"), Ok(("", Double(-1.234))));
    }

    #[test]
    fn test_positive_infinity() {
        assert_eq!(Double::parse(",inf\r\n"), Ok(("", Double(f64::INFINITY))));
    }

    #[test]
    fn test_negative_infinity() {
        assert_eq!(
            Double::parse(",-inf\r\n"),
            Ok(("", Double(f64::NEG_INFINITY)))
        );
    }
}
