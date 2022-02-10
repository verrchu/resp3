#[cfg(test)]
pub(crate) mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    sequence::{delimited, pair},
    IResult, Parser,
};
use num_bigint::BigInt;

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigNumber {
    val: BigInt,
    attr: Option<Attribute>,
}

impl From<BigNumber> for Value {
    fn from(input: BigNumber) -> Value {
        Value::BigNumber(input)
    }
}

impl BigNumber {
    pub fn val(&self) -> &BigInt {
        &self.val
    }

    pub fn attr(&self) -> Option<&Attribute> {
        self.attr.as_ref()
    }

    pub fn with_attr(mut self, attr: Attribute) -> Self {
        self.attr = Some(attr);
        self
    }
}

impl BigNumber {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_attr = opt(Attribute::parse);
        let parse_val = {
            let parser = delimited(tag("("), pair(opt(tag("-")), digit1), tag(DELIMITER));

            map_res(parser, |(sign, number)| {
                let number = str::from_utf8(number).context("Value::BigNumber (str::from_utf8)")?;
                let number = sign
                    .map(|_| format!("-{number}"))
                    .unwrap_or_else(|| number.to_string());
                let number = BigInt::from_str(&number)
                    .context("Value::BigNUmber (BigInt::from_str_radix)")?;

                Ok::<_, anyhow::Error>(number)
            })
        };

        pair(parse_attr, parse_val)
            .map(|(attr, val)| BigNumber { val, attr })
            .parse(input)
    }
}

impl From<BigInt> for BigNumber {
    fn from(input: BigInt) -> Self {
        Self {
            val: input,
            attr: None,
        }
    }
}

impl TryFrom<&BigNumber> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &BigNumber) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::BigNumber (Bytes::from)")?;
            buf.write(&bytes).context("Value::BigNumber (buf::write)")?;
        }

        buf.write(b"(")
            .and_then(|_| buf.write(input.val.to_string().as_bytes()))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::BigNumber (buf::write)")?;

        buf.flush().context("Value::BigNumber (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<BigNumber> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: BigNumber) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
