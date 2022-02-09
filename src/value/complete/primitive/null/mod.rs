#[cfg(test)]
pub(crate) mod tests;

use std::io::Write;

use anyhow::Context;
use bytes::Bytes;
use nom::{bytes::complete::tag, sequence::terminated, IResult, Parser};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl From<Null> for Value {
    fn from(_input: Null) -> Value {
        Value::Null
    }
}

impl Null {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        terminated(tag("_"), tag(DELIMITER))
            .map(|_| Null)
            .parse(input)
    }
}

impl TryFrom<Null> for Bytes {
    type Error = anyhow::Error;

    fn try_from(_input: Null) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write("_".as_bytes())
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::Null (buf::write)")?;
        buf.flush().context("Value::Null (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
