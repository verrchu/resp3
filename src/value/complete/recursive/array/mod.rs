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
    combinator::map_res,
    combinator::opt,
    multi::many_m_n,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array {
    val: Vec<Value>,
    attr: Option<Attribute>,
}

impl From<Array> for Value {
    fn from(input: Array) -> Value {
        Value::Array(input)
    }
}

impl Array {
    pub fn val(&self) -> &[Value] {
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

        let parse_items = |len| many_m_n(len, len, Value::parse);
        let parse_val = parse_len.flat_map(parse_items);

        let parse_attr = opt(Attribute::parse);
        pair(parse_attr, parse_val)
            .map(|(attr, val)| Array { val, attr })
            .parse(input)
    }
}

impl<I: IntoIterator<Item = Value>> From<I> for Array {
    fn from(input: I) -> Self {
        Self {
            val: input.into_iter().collect(),
            attr: None,
        }
    }
}

impl TryFrom<&Array> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Array) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::Array (Bytes::from)")?;
            buf.write(&bytes).context("Value::Array (buf::write)")?;
        }

        buf.write(b"*")
            .and_then(|_| buf.write(input.val().len().to_string().as_bytes()))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::Array (buf::write)")?;

        for value in input.val().iter() {
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
