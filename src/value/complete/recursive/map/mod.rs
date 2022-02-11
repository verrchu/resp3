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
    combinator::{map_res, opt},
    multi::many_m_n,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map {
    val: BTreeMap<Value, Value>,
    attr: Option<Attribute>,
}

impl From<Map> for Value {
    fn from(input: Map) -> Value {
        Value::Map(input)
    }
}

impl Map {
    #[allow(clippy::mutable_key_type)] // FIXME
    pub fn val(&self) -> &BTreeMap<Value, Value> {
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

impl Map {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("%"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Map (str::from_utf8)")
                    .and_then(|v| usize::from_str(v).context("Value::Map (usize::from_str)"))
            })
        };

        let parse_items = |len| many_m_n(len, len, pair(Value::parse, Value::parse));

        let parse_attr = opt(Attribute::parse);
        let parse_val = parse_len.flat_map(parse_items);

        pair(parse_attr, parse_val)
            .map(|(attr, val)| Map {
                val: val.into_iter().collect(),
                attr,
            })
            .parse(input)
    }
}

impl<I: IntoIterator<Item = (Value, Value)>> From<I> for Map {
    fn from(input: I) -> Self {
        Self {
            val: input.into_iter().collect(),
            attr: None,
        }
    }
}

impl TryFrom<&Map> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Map) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::Map (Bytes::from)")?;
            buf.write(&bytes).context("Value::Map (buf::write)")?;
        }

        buf.write(b"%")
            .and_then(|_| buf.write(input.val().len().to_string().as_bytes()))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::Map (buf::write)")?;

        for (k, v) in input.val().iter() {
            let bytes = Bytes::try_from(k).context("Value::Map (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Map (buf::write)")?;

            let bytes = Bytes::try_from(v).context("Value::Map (Bytes::try_from)")?;
            buf.write(&bytes).context("Value::Map (buf::write)")?;
        }

        buf.flush().context("Value::Map (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<Map> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Map) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
