#[cfg(test)]
pub(crate) mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::many_m_n,
    sequence::delimited, IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array(Vec<Value>);

impl From<Array> for Value {
    fn from(input: Array) -> Value {
        Value::Array(input)
    }
}

impl Array {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("*"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Array (str::from_utf8)")
                    .and_then(|v| usize::from_str(v).context("Value::Array (usize::from_str)"))
            })
        };

        let parse_val = |len| many_m_n(len, len, Value::parse);
        parse_len.flat_map(parse_val).map(Array::from).parse(input)
    }
}

impl<I: IntoIterator<Item = Value>> From<I> for Array {
    fn from(input: I) -> Self {
        Self(input.into_iter().collect())
    }
}

impl TryFrom<&Array> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Array) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        buf.write("*".as_bytes())
            .and_then(|_| buf.write(input.0.len().to_string().as_bytes()))
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::Array (buf::write)")?;

        for value in input.0.iter() {
            let bytes = Bytes::try_from(value).context("Value::Array (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Array (buf::write)")?;
        }

        buf.flush().context("Value::Array (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<Array> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Array) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
