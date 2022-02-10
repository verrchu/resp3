#[cfg(test)]
pub(crate) mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

use super::{Attribute, Value, DELIMITER};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    combinator::opt,
    sequence::{delimited, pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobString {
    val: Bytes,
    attr: Option<Attribute>,
}

impl From<BlobString> for Value {
    fn from(input: BlobString) -> Value {
        Value::BlobString(input)
    }
}

impl BlobString {
    pub fn val(&self) -> &Bytes {
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

impl BlobString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("$"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::BlobString (str::from_utf8)")
                    .and_then(|v| u64::from_str(v).context("Value::BlobString (u64::from_str)"))
            })
        };

        let parse_attr = opt(Attribute::parse);
        let parse_val = |len| terminated(take(len), tag(DELIMITER));

        pair(parse_attr, parse_len.flat_map(parse_val))
            .map(|(attr, val)| BlobString {
                val: Bytes::from(val.to_vec()),
                attr,
            })
            .parse(input)
    }
}

impl<B: Into<Bytes>> From<B> for BlobString {
    fn from(input: B) -> Self {
        Self {
            val: input.into(),
            attr: None,
        }
    }
}

impl TryFrom<&BlobString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &BlobString) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::BlobString (Bytes::from)")?;
            buf.write(&bytes)
                .context("Value::BlobString (buf::write)")?;
        }

        buf.write(b"$")
            .and_then(|_| buf.write(input.val().len().to_string().as_bytes()))
            .and_then(|_| buf.write(DELIMITER))
            .and_then(|_| buf.write(input.val()))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::BlobString (buf::write)")?;

        buf.flush().context("Value::BlobString (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<BlobString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: BlobString) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
