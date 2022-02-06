use std::str::{self, FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    error::{Error, ErrorKind},
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq)]
pub struct Double(pub f64);

impl From<Double> for Value {
    fn from(input: Double) -> Value {
        Value::Double(input)
    }
}

impl Double {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_inf = pair(opt(tag("-")), tag("inf"))
            .map(|(sign, _inf)| sign.map(|_| f64::NEG_INFINITY).unwrap_or(f64::INFINITY))
            .map(Ok);

        let parse_num = tuple((opt(tag("-")), digit1, opt(preceded(tag("."), digit1)))).map(
            |(sign, int, frac): (Option<&[u8]>, &[u8], Option<&[u8]>)| {
                let int = unsafe { str::from_utf8_unchecked(int) };
                let frac = unsafe { frac.map(|frac| String::from_utf8_unchecked(frac.to_vec())) };

                let n = match (sign, frac) {
                    (Some(_sign), Some(frac)) => format!("-{int}.{frac}"),
                    (Some(_sign), None) => format!("-{int}"),
                    (None, Some(frac)) => format!("{int}.{frac}"),
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
        assert_eq!(
            Double::parse(&b",1.234\r\n"[..]),
            Ok((&b""[..], Double(1.234)))
        );
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(
            Double::parse(&b",-1.234\r\n"[..]),
            Ok((&b""[..], Double(-1.234)))
        );
    }

    #[test]
    fn test_positive_infinity() {
        assert_eq!(
            Double::parse(&b",inf\r\n"[..]),
            Ok((&b""[..], Double(f64::INFINITY)))
        );
    }

    #[test]
    fn test_negative_infinity() {
        assert_eq!(
            Double::parse(&b",-inf\r\n"[..]),
            Ok((&b""[..], Double(f64::NEG_INFINITY)))
        );
    }
}
