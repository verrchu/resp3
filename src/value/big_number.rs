use std::str;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    error::{Error, ErrorKind},
    sequence::{delimited, pair},
    IResult, Parser,
};
use num_bigint::BigInt;
use num_traits::Num;

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq)]
pub struct BigNumber(pub BigInt);

impl From<BigNumber> for Value {
    fn from(input: BigNumber) -> Value {
        Value::BigNumber(input)
    }
}

impl BigNumber {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        delimited(tag("("), pair(opt(tag("-")), digit1), tag(DELIMITER))
            .parse(input)
            .and_then(|(i, (sign, number))| {
                let number = unsafe { str::from_utf8_unchecked(number) };
                let number = sign
                    .map(|_| format!("-{number}"))
                    .unwrap_or_else(|| number.to_string());
                let number = BigInt::from_str_radix(&number, 10)
                    .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, BigNumber(number)))
            })
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
