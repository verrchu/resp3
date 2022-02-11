#[cfg(test)]
pub(crate) mod tests;

use std::{
    collections::BTreeMap,
    io::Write,
    str::{self, FromStr},
};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::many_m_n,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attribute(BTreeMap<Value, Value>);

impl Attribute {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("|"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Map (str::from_utf8)")
                    .and_then(|v| usize::from_str(v).context("Value::Map (usize::from_str)"))
            })
        };

        let parse_val = |len| many_m_n(len, len, pair(Value::parse, Value::parse));
        parse_len
            .flat_map(parse_val)
            .map(Attribute::from)
            .parse(input)
    }
}

impl<I: IntoIterator<Item = (Value, Value)>> From<I> for Attribute {
    fn from(input: I) -> Self {
        Self(input.into_iter().collect())
    }
}

impl TryFrom<&Attribute> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Attribute) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        buf.write(b"|")
            .and_then(|_| buf.write(input.0.len().to_string().as_bytes()))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::Attribute (buf::write)")?;

        for (k, v) in input.0.iter() {
            let bytes = Bytes::try_from(k).context("Value::Attribute (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Attribute (buf::write)")?;

            let bytes = Bytes::try_from(v).context("Value::Attribute (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Attribute (buf::write)")?;
        }

        buf.flush().context("Value::Attribute (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<Attribute> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Attribute) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
