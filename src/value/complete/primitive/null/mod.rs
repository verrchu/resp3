#[cfg(test)]
pub(crate) mod tests;

use std::io::Write;

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag,
    combinator::opt,
    sequence::{pair, terminated},
    IResult, Parser,
};

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null {
    attr: Option<Attribute>,
}

impl From<Null> for Value {
    fn from(input: Null) -> Value {
        Value::Null(input)
    }
}

impl Null {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_val = terminated(tag("_"), tag(DELIMITER));
        pair(opt(Attribute::parse), parse_val)
            .map(|(attr, _)| Null { attr })
            .parse(input)
    }
}

impl TryFrom<Null> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Null) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr {
            let bytes = Bytes::try_from(attr).context("Value::Null (Bytes::from)")?;
            buf.write(&bytes).context("Value::Null (buf::write)")?;
        }

        buf.write("_".as_bytes())
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::Null (buf::write)")?;

        buf.flush().context("Value::Null (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
