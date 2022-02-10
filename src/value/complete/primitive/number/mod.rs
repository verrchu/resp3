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

use super::{Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl TryFrom<&Number> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Number) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write(":".as_bytes())
            .and_then(|_| buf.write(input.0.to_string().as_bytes()))
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::Number (buf::write)")?;
        buf.flush().context("Value::Number (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<Number> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Number) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
