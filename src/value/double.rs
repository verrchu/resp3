use std::str::{self, FromStr};

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// stores bits forwm of f64
pub struct Double(u64);

impl From<Double> for Value {
    fn from(input: Double) -> Value {
        Value::Double(input)
    }
}

impl Double {
    // this method should be used to retrieve the actual f64 value
    pub fn value(&self) -> f64 {
        f64::from_bits(self.0)
    }
}

impl Double {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_inf = pair(opt(tag("-")), tag("inf"))
            .map(|(sign, _inf)| sign.map(|_| f64::NEG_INFINITY).unwrap_or(f64::INFINITY));

        let parse_num = {
            let parser = tuple((opt(tag("-")), digit1, opt(preceded(tag("."), digit1))));

            let wrapper = map_res(parser, |(sign, int, frac)| {
                let int = str::from_utf8(int).context("Value::Double (str::from_utf8)")?;
                let frac = frac
                    .map(|frac| String::from_utf8(frac.to_vec()))
                    .transpose()
                    .context("Value::Double (String::from_utf8)")?;

                Ok::<_, anyhow::Error>(match (sign, frac) {
                    (Some(_sign), Some(frac)) => format!("-{int}.{frac}"),
                    (Some(_sign), None) => format!("-{int}"),
                    (None, Some(frac)) => format!("{int}.{frac}"),
                    (None, None) => int.to_string(),
                })
            });

            map_res(wrapper, |number| {
                f64::from_str(&number).context("Value::Double (f64::from_str)")
            })
        };

        delimited(tag(","), alt((parse_num, parse_inf)), tag(DELIMITER))
            .map(Double::from)
            .parse(input)
    }
}

impl From<f64> for Double {
    fn from(input: f64) -> Self {
        Self(input.to_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(
            Double::parse(&b",1.234\r\n"[..]),
            Ok((&b""[..], Double::from(1.234)))
        );
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(
            Double::parse(&b",-1.234\r\n"[..]),
            Ok((&b""[..], Double::from(-1.234)))
        );
    }

    #[test]
    fn test_positive_infinity() {
        assert_eq!(
            Double::parse(&b",inf\r\n"[..]),
            Ok((&b""[..], Double::from(f64::INFINITY)))
        );
    }

    #[test]
    fn test_negative_infinity() {
        assert_eq!(
            Double::parse(&b",-inf\r\n"[..]),
            Ok((&b""[..], Double::from(f64::NEG_INFINITY)))
        );
    }
}
