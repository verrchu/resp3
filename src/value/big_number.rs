use std::str;

use anyhow::Context;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    sequence::{delimited, pair},
    IResult, Parser,
};
use num_bigint::BigInt;
use num_traits::Num;

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigNumber(pub BigInt);

impl From<BigNumber> for Value {
    fn from(input: BigNumber) -> Value {
        Value::BigNumber(input)
    }
}

impl BigNumber {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_val = {
            let parser = delimited(tag("("), pair(opt(tag("-")), digit1), tag(DELIMITER));

            map_res(parser, |(sign, number)| {
                let number = str::from_utf8(number).context("Value::BigNumber (str::from_utf8)")?;
                let number = sign
                    .map(|_| format!("-{number}"))
                    .unwrap_or_else(|| number.to_string());
                let number = BigInt::from_str_radix(&number, 10)
                    .context("Value::BigNUmber (BigInt::from_str_radix)")?;

                Ok::<_, anyhow::Error>(number)
            })
        };

        parse_val.map(BigNumber::from).parse(input)
    }
}

impl From<BigInt> for BigNumber {
    fn from(input: BigInt) -> Self {
        Self(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        let str = ['1'; 100].into_iter().collect::<String>();
        let raw = format!("({str}\r\n");

        let (i, o) = BigNumber::parse(raw.as_bytes()).unwrap();
        assert!(i.is_empty());

        assert_eq!(o.0.to_string(), str);
    }

    #[test]
    fn test_negative_number() {
        let str = ['1'; 100].into_iter().collect::<String>();
        let str = format!("-{str}");

        let raw = format!("({str}\r\n");

        let (i, o) = BigNumber::parse(raw.as_bytes()).unwrap();
        assert!(i.is_empty());

        assert_eq!(o.0.to_string(), str);
    }
}
